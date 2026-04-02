// vinsg
// Djinni IDL parser

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

use djinni_ast::ast::*;

#[derive(Parser)]
#[grammar = "djinni.pest"]
struct DjinniParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{file}:{line}:{col}: {message}")]
    Syntax {
        file: String,
        line: usize,
        col: usize,
        message: String,
    },
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Circular import detected: {0}")]
    CircularImport(String),
    #[error("YAML parse error in {file}: {message}")]
    YamlError { file: String, message: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct ParserContext {
    include_paths: Vec<String>,
    visited_files: HashSet<PathBuf>,
    file_stack: Vec<PathBuf>,
}

impl ParserContext {
    pub fn new(include_paths: Vec<String>) -> Self {
        Self {
            include_paths,
            visited_files: HashSet::new(),
            file_stack: Vec::new(),
        }
    }

    fn current_file(&self) -> &Path {
        self.file_stack.last().unwrap()
    }

    fn resolve_import(&self, filename: &str) -> Result<PathBuf, ParseError> {
        for path in &self.include_paths {
            let rel_path = if path.is_empty() {
                self.current_file().parent().unwrap().to_path_buf()
            } else {
                PathBuf::from(path)
            };
            let candidate = rel_path.join(filename);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
        Err(ParseError::FileNotFound(format!(
            "Unable to find file \"{}\" at {}",
            filename,
            self.current_file().display()
        )))
    }

    pub fn parse_file(
        &mut self,
        idl_file: &Path,
        in_files: &mut Vec<PathBuf>,
    ) -> Result<(Vec<TypeDecl>, Vec<String>), ParseError> {
        // Canonicalize the path so all subsequent relative imports resolve correctly
        let normalized = if idl_file.is_relative() {
            std::env::current_dir()
                .unwrap_or_default()
                .join(idl_file)
        } else {
            idl_file.to_path_buf()
        };
        // Use std canonicalize if file exists, otherwise just normalize
        let normalized = normalized.canonicalize().unwrap_or_else(|_| normalize_path(&normalized));
        in_files.push(normalized.clone());

        self.visited_files.insert(normalized.clone());
        self.file_stack.push(normalized.clone());

        let content = fs::read_to_string(&normalized)?;
        let result = self.parse_idl_content(&content, &normalized);

        self.file_stack.pop();
        let idl = result?;

        let mut types = idl.type_decls;
        let mut flags = idl.flags;

        for import in &idl.imports {
            let file = normalize_path(import.file());
            if self.file_stack.contains(&file) {
                return Err(ParseError::CircularImport(file.display().to_string()));
            }
            if !self.visited_files.contains(&file) {
                match import {
                    FileRef::Idl(_) => {
                        let (t, f) = self.parse_file(&file, in_files)?;
                        types = [t, types].concat();
                        flags = [f, flags].concat();
                    }
                    FileRef::Extern(_) => {
                        let t = self.parse_extern_file(&file, in_files)?;
                        types = [t, types].concat();
                    }
                    FileRef::Protobuf(_) => {
                        let t = self.parse_protobuf_file(&file, in_files)?;
                        types = [t, types].concat();
                    }
                }
            }
        }

        Ok((types, flags))
    }

    fn parse_idl_content(
        &self,
        content: &str,
        file: &Path,
    ) -> Result<IdlFile, ParseError> {
        let pairs = DjinniParser::parse(Rule::idl_file, content).map_err(|e| {
            let (line, col) = match e.line_col {
                pest::error::LineColLocation::Pos((l, c)) => (l, c),
                pest::error::LineColLocation::Span((l, c), _) => (l, c),
            };
            ParseError::Syntax {
                file: file.display().to_string(),
                line,
                col,
                message: e.to_string(),
            }
        })?;

        let file_pair = pairs.into_iter().next().unwrap();
        self.build_idl_file(file_pair, file)
    }

    fn build_idl_file(
        &self,
        pair: pest::iterators::Pair<Rule>,
        file: &Path,
    ) -> Result<IdlFile, ParseError> {
        let mut imports = Vec::new();
        let mut type_decls = Vec::new();
        let mut flags = Vec::new();

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::flag => {
                    let qs = inner.into_inner().next().unwrap();
                    let s = extract_quoted_string(qs);
                    flags.push(s);
                }
                Rule::import_ref => {
                    let mut parts = inner.into_inner();
                    let dir = parts.next().unwrap().as_str();
                    let path_str = extract_quoted_string(parts.next().unwrap());
                    let resolved = self.resolve_import(&path_str)?;
                    let file_ref = match dir {
                        "import" => FileRef::Idl(resolved),
                        "extern" => FileRef::Extern(resolved),
                        "protobuf" => FileRef::Protobuf(resolved),
                        _ => unreachable!(),
                    };
                    imports.push(file_ref);
                }
                Rule::type_decl => {
                    let td = self.build_type_decl(inner, file)?;
                    type_decls.push(td);
                }
                Rule::EOI => {}
                _ => {}
            }
        }

        Ok(IdlFile {
            imports,
            type_decls,
            flags,
        })
    }

    fn build_type_decl(
        &self,
        pair: pest::iterators::Pair<Rule>,
        file: &Path,
    ) -> Result<TypeDecl, ParseError> {
        let mut inner = pair.into_inner();
        let doc = build_doc(inner.next().unwrap());
        let ident = build_ident(inner.next().unwrap(), file);

        let next = inner.next().unwrap();
        let (params, body_pair) = match next.as_rule() {
            Rule::type_params => {
                let params = build_type_params(next, file);
                (params, inner.next().unwrap())
            }
            _ => (Vec::new(), next),
        };

        let body = self.build_type_def(body_pair, file)?;
        let origin = file.file_name().unwrap().to_string_lossy().to_string();

        Ok(TypeDecl::Intern {
            ident,
            params,
            body,
            doc,
            origin,
        })
    }

    fn build_type_def(
        &self,
        pair: pest::iterators::Pair<Rule>,
        file: &Path,
    ) -> Result<TypeDef, ParseError> {
        match pair.as_rule() {
            Rule::record_def => self.build_record(pair, file),
            Rule::enum_def => self.build_enum(pair, file),
            Rule::flags_def => self.build_flags(pair, file),
            Rule::interface_def => self.build_interface(pair, file),
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }

    fn build_record(
        &self,
        pair: pest::iterators::Pair<Rule>,
        file: &Path,
    ) -> Result<TypeDef, ParseError> {
        let mut base_record = None;
        let mut ext = Ext::default();
        let mut fields = Vec::new();
        let mut consts = Vec::new();
        let mut deriving_types = std::collections::BTreeSet::new();

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::base_record => {
                    let name = inner.into_inner().next().unwrap().as_str().to_string();
                    base_record = Some(name);
                }
                Rule::ext_mods => {
                    ext = build_ext(inner, file);
                }
                Rule::record_items => {
                    for item in inner.into_inner() {
                        match item.as_rule() {
                            Rule::record_field => {
                                fields.push(build_field(item, file));
                            }
                            Rule::const_field => {
                                consts.push(build_const(item, file));
                            }
                            _ => {}
                        }
                    }
                }
                Rule::deriving_clause => {
                    for ident_pair in inner.into_inner() {
                        if ident_pair.as_rule() == Rule::ident {
                            match ident_pair.as_str() {
                                "ord" => { deriving_types.insert(DerivingType::Ord); }
                                "parcelable" => { deriving_types.insert(DerivingType::AndroidParcelable); }
                                "nscopying" => { deriving_types.insert(DerivingType::NSCopying); }
                                other => {
                                    return Err(ParseError::Syntax {
                                        file: file.display().to_string(),
                                        line: 0,
                                        col: 0,
                                        message: format!("Unrecognized deriving type: {}", other),
                                    });
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(TypeDef::Record(Record {
            ext,
            fields,
            consts,
            deriving_types,
            base_record,
        }))
    }

    fn build_enum(
        &self,
        pair: pest::iterators::Pair<Rule>,
        file: &Path,
    ) -> Result<TypeDef, ParseError> {
        let mut options = Vec::new();
        let inner_pairs: Vec<_> = pair.into_inner().collect();

        // enum_def contains alternating doc and ident pairs
        let mut i = 0;
        while i < inner_pairs.len() {
            if inner_pairs[i].as_rule() == Rule::doc {
                let doc = build_doc(inner_pairs[i].clone());
                i += 1;
                if i < inner_pairs.len() && inner_pairs[i].as_rule() == Rule::ident {
                    let ident = build_ident(inner_pairs[i].clone(), file);
                    options.push(EnumOption {
                        ident,
                        doc,
                        special_flag: None,
                    });
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        Ok(TypeDef::Enum(Enum {
            options,
            flags: false,
        }))
    }

    fn build_flags(
        &self,
        pair: pest::iterators::Pair<Rule>,
        file: &Path,
    ) -> Result<TypeDef, ParseError> {
        let mut options = Vec::new();

        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::flags_option {
                let mut parts = inner.into_inner();
                let doc = build_doc(parts.next().unwrap());
                let ident = build_ident(parts.next().unwrap(), file);
                let special = parts.next().map(|p| match p.as_str() {
                    "all" => SpecialFlag::AllFlags,
                    "none" => SpecialFlag::NoFlags,
                    _ => unreachable!(),
                });
                options.push(EnumOption {
                    ident,
                    doc,
                    special_flag: special,
                });
            }
        }

        Ok(TypeDef::Enum(Enum {
            options,
            flags: true,
        }))
    }

    fn build_interface(
        &self,
        pair: pest::iterators::Pair<Rule>,
        file: &Path,
    ) -> Result<TypeDef, ParseError> {
        let mut ext = Ext {
            java: true,
            cpp: true,
            objc: true,
            js: true,
        };
        let mut methods = Vec::new();
        let mut consts = Vec::new();

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::ext_mods => {
                    let built = build_ext(inner, file);
                    if built.any() {
                        ext = built;
                    }
                }
                Rule::interface_items => {
                    for item in inner.into_inner() {
                        match item.as_rule() {
                            Rule::method => {
                                methods.push(build_method(item, file));
                            }
                            Rule::const_field => {
                                consts.push(build_const(item, file));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(TypeDef::Interface(Interface { ext, methods, consts }))
    }

    fn parse_extern_file(
        &mut self,
        file: &Path,
        in_files: &mut Vec<PathBuf>,
    ) -> Result<Vec<TypeDecl>, ParseError> {
        in_files.push(file.to_path_buf());
        self.visited_files.insert(file.to_path_buf());
        self.file_stack.push(file.to_path_buf());

        let content = fs::read_to_string(file)?;
        let result = parse_extern_yaml(&content, file);

        self.file_stack.pop();
        result
    }

    fn parse_protobuf_file(
        &mut self,
        file: &Path,
        in_files: &mut Vec<PathBuf>,
    ) -> Result<Vec<TypeDecl>, ParseError> {
        in_files.push(file.to_path_buf());
        self.visited_files.insert(file.to_path_buf());
        self.file_stack.push(file.to_path_buf());

        let content = fs::read_to_string(file)?;
        let result = parse_protobuf_yaml(&content, file);

        self.file_stack.pop();
        result
    }
}

// --- Helper functions ---

fn extract_quoted_string(pair: pest::iterators::Pair<Rule>) -> String {
    // quoted_string = ${ "\"" ~ quoted_inner ~ "\"" }
    pair.into_inner()
        .next()
        .unwrap()
        .as_str()
        .to_string()
}

fn build_doc(pair: pest::iterators::Pair<Rule>) -> Doc {
    let mut lines = Vec::new();
    for line_pair in pair.into_inner() {
        if line_pair.as_rule() == Rule::doc_line {
            // doc_line = ${ "#" ~ doc_text ~ NEWLINE }
            let text = line_pair
                .into_inner()
                .next()
                .unwrap()
                .as_str()
                .to_string();
            lines.push(text);
        }
    }
    Doc { lines }
}

fn build_ident(pair: pest::iterators::Pair<Rule>, file: &Path) -> Ident {
    let span = pair.as_span();
    let (line, col) = span.start_pos().line_col();
    Ident {
        name: pair.as_str().to_string(),
        loc: Loc {
            file: file.to_path_buf(),
            line,
            col,
        },
    }
}

fn build_type_params(pair: pest::iterators::Pair<Rule>, file: &Path) -> Vec<TypeParam> {
    pair.into_inner()
        .filter(|p| p.as_rule() == Rule::ident)
        .map(|p| TypeParam {
            ident: build_ident(p, file),
        })
        .collect()
}

fn build_ext(pair: pest::iterators::Pair<Rule>, _file: &Path) -> Ext {
    let mut ext = Ext::default();
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::ident {
            match inner.as_str() {
                "c" => ext.cpp = true,
                "j" => ext.java = true,
                "o" => ext.objc = true,
                "w" => ext.js = true,
                _ => {}
            }
        }
    }
    ext
}

fn build_field(pair: pest::iterators::Pair<Rule>, file: &Path) -> Field {
    let mut inner = pair.into_inner();
    let doc = build_doc(inner.next().unwrap());
    let ident = build_ident(inner.next().unwrap(), file);
    let ty = build_type_ref(inner.next().unwrap(), file);
    let default_value = inner
        .next()
        .map(|p| p.as_str().to_string())
        .unwrap_or_default();

    Field {
        ident,
        ty,
        default_value,
        doc,
    }
}

fn build_type_ref(pair: pest::iterators::Pair<Rule>, file: &Path) -> TypeRef {
    // type_ref -> type_expr chain: ident followed by optional type_args
    let expr = build_type_expr(pair, file);
    TypeRef {
        expr,
        resolved: None,
    }
}

fn build_type_expr(pair: pest::iterators::Pair<Rule>, file: &Path) -> TypeExpr {
    let mut inner = pair.into_inner();
    let ident_pair = inner.next().unwrap();

    // If ident_pair is itself a type_ref, recurse
    if ident_pair.as_rule() == Rule::ident {
        let ident = build_ident(ident_pair, file);
        let args: Vec<TypeExpr> = inner
            .filter(|p| p.as_rule() == Rule::type_ref)
            .map(|p| build_type_expr(p, file))
            .collect();
        TypeExpr { ident, args }
    } else {
        // type_ref wraps another level
        build_type_expr(ident_pair, file)
    }
}

fn build_const(pair: pest::iterators::Pair<Rule>, file: &Path) -> Const {
    let mut inner = pair.into_inner();
    let doc = build_doc(inner.next().unwrap());
    let ident = build_ident(inner.next().unwrap(), file);
    let ty = build_type_ref(inner.next().unwrap(), file);
    let value = build_const_value(inner.next().unwrap(), file);

    Const {
        ident,
        ty,
        value,
        doc,
    }
}

fn build_const_value(pair: pest::iterators::Pair<Rule>, file: &Path) -> ConstValue {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::int_val => ConstValue::Int(inner.as_str().parse().unwrap()),
        Rule::float_val => ConstValue::Float(inner.as_str().parse().unwrap()),
        Rule::bool_val => ConstValue::Bool(inner.as_str().to_lowercase().parse().unwrap()),
        Rule::string_val => {
            let s = inner.into_inner().next().unwrap().as_str().to_string();
            ConstValue::String(s)
        }
        Rule::enum_val => {
            let mut parts = inner.into_inner();
            let ty = parts.next().unwrap().as_str().to_string();
            let value = parts.next().unwrap().as_str().to_string();
            ConstValue::EnumValue { ty, value }
        }
        Rule::const_ref_val => {
            let name = inner.into_inner().next().unwrap().as_str().to_string();
            ConstValue::ConstRef(name)
        }
        Rule::composite_val => {
            let mut fields = Vec::new();
            for field_pair in inner.into_inner() {
                if field_pair.as_rule() == Rule::composite_field {
                    let mut parts = field_pair.into_inner();
                    let name = parts.next().unwrap().as_str().to_string();
                    let val = build_const_value(parts.next().unwrap(), file);
                    fields.push((name, val));
                }
            }
            ConstValue::Composite(fields)
        }
        _ => unreachable!("Unexpected const value rule: {:?}", inner.as_rule()),
    }
}

fn build_method(pair: pest::iterators::Pair<Rule>, file: &Path) -> Method {
    let mut inner = pair.into_inner();
    let doc = build_doc(inner.next().unwrap());

    let mut is_static = false;
    let mut is_const = false;

    let mut next = inner.next().unwrap();
    if next.as_rule() == Rule::static_kw {
        is_static = true;
        next = inner.next().unwrap();
    }
    if next.as_rule() == Rule::const_kw {
        is_const = true;
        next = inner.next().unwrap();
    }

    let ident = build_ident(next, file);

    let mut params = Vec::new();
    let mut ret = None;
    let mut lang = Ext {
        java: true,
        cpp: true,
        objc: true,
        js: true,
    };

    for remaining in inner {
        match remaining.as_rule() {
            Rule::param_list => {
                for field_pair in remaining.into_inner() {
                    if field_pair.as_rule() == Rule::record_field {
                        params.push(build_field(field_pair, file));
                    }
                }
            }
            Rule::type_ref => {
                let type_ref = build_type_ref(remaining, file);
                // Check for void return
                if type_ref.expr.ident.name != "void" {
                    ret = Some(type_ref);
                }
            }
            Rule::ext_mods => {
                let built = build_ext(remaining, file);
                if built.any() {
                    lang = built;
                }
            }
            _ => {}
        }
    }

    Method {
        ident,
        params,
        ret,
        doc,
        is_static,
        is_const,
        lang,
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    // Normalize the path by resolving . and ..
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            _ => {
                components.push(component);
            }
        }
    }
    components.iter().collect()
}

// --- YAML extern parsing ---

fn parse_extern_yaml(content: &str, file: &Path) -> Result<Vec<TypeDecl>, ParseError> {
    let mut type_decls = Vec::new();

    // YAML can have multiple documents separated by ---
    for doc in serde_yaml::Deserializer::from_str(content) {
        let value: serde_yaml::Value = serde_yaml::Value::deserialize(doc).map_err(|e| {
            ParseError::YamlError {
                file: file.display().to_string(),
                message: e.to_string(),
            }
        })?;

        let map = value.as_mapping().ok_or_else(|| ParseError::YamlError {
            file: file.display().to_string(),
            message: "Expected YAML mapping".into(),
        })?;

        let name = map
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ParseError::YamlError {
                file: file.display().to_string(),
                message: "'name' not found".into(),
            })?
            .to_string();

        let typedef_str = map
            .get("typedef")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ParseError::YamlError {
                file: file.display().to_string(),
                message: "'typedef' not found".into(),
            })?;

        let params: Vec<TypeParam> = map
            .get("params")
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| TypeParam {
                        ident: Ident {
                            name: s.to_string(),
                            loc: Loc {
                                file: file.to_path_buf(),
                                line: 1,
                                col: 1,
                            },
                        },
                    })
                    .collect()
            })
            .unwrap_or_default();

        let body = parse_extern_typedef(typedef_str, file)?;
        let ident = Ident {
            name,
            loc: Loc {
                file: file.to_path_buf(),
                line: 1,
                col: 1,
            },
        };

        // Convert yaml::Value to HashMap<String, serde_yaml::Value>
        let mut properties = HashMap::new();
        for (k, v) in map {
            if let Some(key) = k.as_str() {
                properties.insert(key.to_string(), v.clone());
            }
        }

        type_decls.push(TypeDecl::Extern {
            ident,
            params,
            body,
            properties,
            origin: file.file_name().unwrap().to_string_lossy().to_string(),
        });
    }

    Ok(type_decls)
}

fn parse_extern_typedef(typedef_str: &str, file: &Path) -> Result<TypeDef, ParseError> {
    let parts: Vec<&str> = typedef_str.split_whitespace().collect();
    if parts.is_empty() {
        return Err(ParseError::YamlError {
            file: file.display().to_string(),
            message: "Empty typedef".into(),
        });
    }

    match parts[0] {
        "enum" => Ok(TypeDef::Enum(Enum {
            options: Vec::new(),
            flags: false,
        })),
        "flags" => Ok(TypeDef::Enum(Enum {
            options: Vec::new(),
            flags: true,
        })),
        "interface" => {
            let ext = parse_extern_ext(&parts[1..]);
            Ok(TypeDef::Interface(Interface {
                ext,
                methods: Vec::new(),
                consts: Vec::new(),
            }))
        }
        "record" => {
            let mut ext = Ext::default();
            let mut deriving_types = std::collections::BTreeSet::new();

            let mut i = 1;
            while i < parts.len() {
                if parts[i].starts_with('+') {
                    match &parts[i][1..] {
                        "c" => ext.cpp = true,
                        "j" => ext.java = true,
                        "o" => ext.objc = true,
                        "w" => ext.js = true,
                        _ => {}
                    }
                    i += 1;
                } else if parts[i] == "deriving" || parts[i].starts_with("deriving(") {
                    // Parse deriving types from the rest
                    let deriving_str = parts[i..].join(" ");
                    if let Some(start) = deriving_str.find('(') {
                        if let Some(end) = deriving_str.find(')') {
                            let inner = &deriving_str[start + 1..end];
                            for dt in inner.split(',') {
                                match dt.trim() {
                                    "ord" => { deriving_types.insert(DerivingType::Ord); }
                                    "parcelable" => { deriving_types.insert(DerivingType::AndroidParcelable); }
                                    "nscopying" => { deriving_types.insert(DerivingType::NSCopying); }
                                    _ => {}
                                }
                            }
                        }
                    }
                    break;
                } else {
                    i += 1;
                }
            }

            Ok(TypeDef::Record(Record {
                ext,
                fields: Vec::new(),
                consts: Vec::new(),
                deriving_types,
                base_record: None,
            }))
        }
        _ => Err(ParseError::YamlError {
            file: file.display().to_string(),
            message: format!("Unrecognized typedef: {}", typedef_str),
        }),
    }
}

fn parse_extern_ext(parts: &[&str]) -> Ext {
    let mut ext = Ext {
        java: true,
        cpp: true,
        objc: true,
        js: true,
    };
    let mut has_explicit = false;
    for part in parts {
        if part.starts_with('+') {
            if !has_explicit {
                ext = Ext::default();
                has_explicit = true;
            }
            match &part[1..] {
                "c" => ext.cpp = true,
                "j" => ext.java = true,
                "o" => ext.objc = true,
                "w" => ext.js = true,
                _ => {}
            }
        }
    }
    ext
}

fn parse_protobuf_yaml(content: &str, file: &Path) -> Result<Vec<TypeDecl>, ParseError> {
    let doc: serde_yaml::Value = serde_yaml::from_str(content).map_err(|e| {
        ParseError::YamlError {
            file: file.display().to_string(),
            message: e.to_string(),
        }
    })?;

    let map = doc.as_mapping().ok_or_else(|| ParseError::YamlError {
        file: file.display().to_string(),
        message: "Expected YAML mapping".into(),
    })?;

    let cpp_map = map.get("cpp").and_then(|v| v.as_mapping()).ok_or_else(|| {
        ParseError::YamlError {
            file: file.display().to_string(),
            message: "'cpp' properties not found".into(),
        }
    })?;
    let java_map = map
        .get("java")
        .and_then(|v| v.as_mapping())
        .ok_or_else(|| ParseError::YamlError {
            file: file.display().to_string(),
            message: "'java' properties not found".into(),
        })?;

    let proto = ProtobufMessage {
        cpp: ProtobufCpp {
            header: yaml_str(cpp_map, "header"),
            ns: yaml_str(cpp_map, "namespace"),
        },
        java: ProtobufJava {
            pkg: yaml_str(java_map, "class"),
            jni_class: yaml_str_opt(java_map, "jni_class"),
            jni_header: yaml_str_opt(java_map, "jni_header"),
        },
        objc: map.get("objc").and_then(|v| v.as_mapping()).map(|m| {
            ProtobufObjc {
                header: yaml_str(m, "header"),
                prefix: yaml_str(m, "prefix"),
            }
        }),
        ts: map.get("ts").and_then(|v| v.as_mapping()).map(|m| {
            ProtobufTs {
                module: yaml_str(m, "module"),
                ns: yaml_str(m, "namespace"),
            }
        }),
    };

    let messages = map
        .get("messages")
        .and_then(|v| v.as_sequence())
        .ok_or_else(|| ParseError::YamlError {
            file: file.display().to_string(),
            message: "'messages' not found".into(),
        })?;

    let origin = file.file_name().unwrap().to_string_lossy().to_string();
    let mut type_decls = Vec::new();

    for msg in messages {
        let name = msg.as_str().unwrap_or_default().to_string();
        let ident = Ident {
            name,
            loc: Loc {
                file: file.to_path_buf(),
                line: 1,
                col: 1,
            },
        };
        type_decls.push(TypeDecl::Protobuf {
            ident,
            params: Vec::new(),
            body: TypeDef::ProtobufMessage(proto.clone()),
            origin: origin.clone(),
        });
    }

    Ok(type_decls)
}

fn yaml_str(map: &serde_yaml::Mapping, key: &str) -> String {
    map.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn yaml_str_opt(map: &serde_yaml::Mapping, key: &str) -> Option<String> {
    map.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
}

use serde::Deserialize;

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn test_parse_simple_record() {
        let input = r#"
my_record = record {
    field1: i32;
    field2: string;
}
"#;
        let file = PathBuf::from("test.djinni");
        let ctx = ParserContext::new(vec![]);
        let result = ctx.parse_idl_content(input, &file);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        let idl = result.unwrap();
        assert_eq!(idl.type_decls.len(), 1);
    }

    #[test]
    fn test_parse_simple_enum() {
        let input = r#"
color = enum {
    red;
    green;
    blue;
}
"#;
        let file = PathBuf::from("test.djinni");
        let ctx = ParserContext::new(vec![]);
        let result = ctx.parse_idl_content(input, &file);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_interface() {
        let input = r#"
test_helpers = interface +c {
    static get_set_record(): set_record;
    static check_set_record(rec: set_record): bool;
}
"#;
        let file = PathBuf::from("test.djinni");
        let ctx = ParserContext::new(vec![]);
        let result = ctx.parse_idl_content(input, &file);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_constants() {
        let input = r#"
constants = record {
    const bool_constant: bool = true;
    const i32_constant: i32 = 3;
    const string_constant: string = "hello";
    const f64_constant: f64 = 5.0;
}
"#;
        let file = PathBuf::from("test.djinni");
        let ctx = ParserContext::new(vec![]);
        let result = ctx.parse_idl_content(input, &file);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_with_doc() {
        let input = r#"# My record docs
# Second line
my_record = record {
    # Field doc
    field1: i32;
}
"#;
        let file = PathBuf::from("test.djinni");
        let ctx = ParserContext::new(vec![]);
        let result = ctx.parse_idl_content(input, &file);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
        let idl = result.unwrap();
        if let TypeDecl::Intern { doc, .. } = &idl.type_decls[0] {
            assert_eq!(doc.lines.len(), 2);
        }
    }

    #[test]
    fn test_parse_record_with_deriving() {
        let input = r#"
record_with_derivings = record {
    key: i32;
    s: string;
} deriving (ord)
"#;
        let file = PathBuf::from("test.djinni");
        let ctx = ParserContext::new(vec![]);
        let result = ctx.parse_idl_content(input, &file);
        assert!(result.is_ok(), "Parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_all_test_suite_files() {
        let root = repo_root();
        let test_suite = root.join("test-suite");

        // Parse the main all.djinni which transitively includes everything
        let mut ctx = ParserContext::new(vec![
            String::new(), // Current directory
            test_suite.join("djinni").join("vendor").to_string_lossy().to_string(),
        ]);
        let mut in_files = Vec::new();
        let result = ctx.parse_file(
            &test_suite.join("djinni").join("all.djinni"),
            &mut in_files,
        );

        assert!(
            result.is_ok(),
            "Failed to parse all.djinni: {:?}",
            result.err()
        );

        let (types, _flags) = result.unwrap();
        assert!(
            types.len() > 10,
            "Expected many type declarations, got {}",
            types.len()
        );

        eprintln!("Parsed {} type declarations from {} files", types.len(), in_files.len());
    }

    #[test]
    fn test_parse_wchar_test() {
        let root = repo_root();
        let test_suite = root.join("test-suite");
        let mut ctx = ParserContext::new(vec![String::new()]);
        let mut in_files = Vec::new();
        let result = ctx.parse_file(
            &test_suite.join("djinni").join("wchar_test.djinni"),
            &mut in_files,
        );
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let (types, _) = result.unwrap();
        assert_eq!(types.len(), 2); // wchar_test_rec + wchar_test_helpers
    }

    #[test]
    fn test_parse_examples() {
        let root = repo_root();
        let examples = root.join("examples");
        let mut ctx = ParserContext::new(vec![String::new()]);
        let mut in_files = Vec::new();
        let result = ctx.parse_file(
            &examples.join("example.djinni"),
            &mut in_files,
        );
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let (types, _) = result.unwrap();
        assert!(types.len() > 0, "Expected type declarations");
        eprintln!("Examples: parsed {} type declarations", types.len());
    }

    #[test]
    fn test_parse_extern_yaml() {
        let root = repo_root();
        let yaml_file = root.join("test-suite/djinni/vendor/third-party/date.yaml");
        let content = fs::read_to_string(&yaml_file).unwrap();
        let result = parse_extern_yaml(&content, &yaml_file);
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let types = result.unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].ident().name, "extern_date");
    }
}
