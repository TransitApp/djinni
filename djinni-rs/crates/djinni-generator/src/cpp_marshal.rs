// vinsg
// CppMarshal: C++ type marshalling, translated from CppMarshal.scala

use djinni_ast::ast::*;
use djinni_ast::meta::*;
use djinni_ast::spec::{q, Spec};

use crate::gen::with_ns;

#[derive(Debug, Clone)]
pub enum SymbolReference {
    ImportRef(String),
    DeclRef {
        decl: String,
        namespace: Option<String>,
    },
}

pub struct CppMarshal<'a> {
    pub spec: &'a Spec,
}

impl<'a> CppMarshal<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        CppMarshal { spec }
    }

    // --- typename ---

    pub fn typename_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_cpp_type_mexpr(tm, None, &[])
    }

    pub fn typename_from_mexpr_scoped(&self, tm: &MExpr, scope_symbols: &[String]) -> String {
        self.to_cpp_type_mexpr(tm, None, scope_symbols)
    }

    pub fn typename_from_typeref(&self, ty: &TypeRef, scope_symbols: &[String]) -> String {
        self.typename_from_mexpr_scoped(
            ty.resolved.as_ref().expect("TypeRef not resolved"),
            scope_symbols,
        )
    }

    pub fn typename_from_name(&self, name: &str, ty: &TypeDef) -> String {
        let id = &self.spec.cpp_ident_style;
        match ty {
            TypeDef::Enum(_) => (id.enum_type)(name),
            TypeDef::Interface(_) => (id.ty)(name),
            TypeDef::Record(_) => (id.ty)(name),
            TypeDef::ProtobufMessage(_) => (id.ty)(name),
        }
    }

    // --- fq_typename ---

    pub fn fq_typename_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_cpp_type_mexpr(tm, Some(&self.spec.cpp_namespace), &[])
    }

    pub fn fq_typename_from_name(&self, name: &str, ty: &TypeDef) -> String {
        let id = &self.spec.cpp_ident_style;
        match ty {
            TypeDef::Enum(_) => {
                with_ns(Some(&self.spec.cpp_namespace), &(id.enum_type)(name))
            }
            TypeDef::Interface(_) => {
                with_ns(Some(&self.spec.cpp_namespace), &(id.ty)(name))
            }
            TypeDef::Record(_) => {
                with_ns(Some(&self.spec.cpp_namespace), &(id.ty)(name))
            }
            TypeDef::ProtobufMessage(p) => {
                with_ns(Some(&p.cpp.ns), &(id.ty)(name))
            }
        }
    }

    // --- param_type ---

    pub fn param_type_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_cpp_param_type(tm, None, &[])
    }

    pub fn param_type_from_mexpr_scoped(
        &self,
        tm: &MExpr,
        scope_symbols: &[String],
    ) -> String {
        self.to_cpp_param_type(tm, None, scope_symbols)
    }

    pub fn param_type_from_typeref(
        &self,
        ty: &TypeRef,
        scope_symbols: &[String],
    ) -> String {
        self.param_type_from_mexpr_scoped(
            ty.resolved.as_ref().expect("TypeRef not resolved"),
            scope_symbols,
        )
    }

    pub fn fq_param_type(&self, tm: &MExpr) -> String {
        self.to_cpp_param_type(tm, Some(&self.spec.cpp_namespace), &[])
    }

    // --- return_type ---

    pub fn return_type(&self, ret: Option<&TypeRef>) -> String {
        match ret {
            None => "void".to_string(),
            Some(ty) => self.to_cpp_type_typeref(ty, None, &[]),
        }
    }

    pub fn return_type_scoped(
        &self,
        ret: Option<&TypeRef>,
        scope_symbols: &[String],
    ) -> String {
        match ret {
            None => "void".to_string(),
            Some(ty) => self.to_cpp_type_typeref(ty, None, scope_symbols),
        }
    }

    pub fn fq_return_type(&self, ret: Option<&TypeRef>) -> String {
        match ret {
            None => "void".to_string(),
            Some(ty) => self.to_cpp_type_typeref(ty, Some(&self.spec.cpp_namespace), &[]),
        }
    }

    // --- field_type ---

    pub fn field_type_from_mexpr(&self, tm: &MExpr) -> String {
        self.typename_from_mexpr(tm)
    }

    pub fn field_type_from_mexpr_scoped(
        &self,
        tm: &MExpr,
        scope_symbols: &[String],
    ) -> String {
        self.typename_from_mexpr_scoped(tm, scope_symbols)
    }

    pub fn field_type_from_typeref(
        &self,
        ty: &TypeRef,
        scope_symbols: &[String],
    ) -> String {
        self.field_type_from_mexpr_scoped(
            ty.resolved.as_ref().expect("TypeRef not resolved"),
            scope_symbols,
        )
    }

    pub fn fq_field_type(&self, tm: &MExpr) -> String {
        self.fq_typename_from_mexpr(tm)
    }

    // --- hpp_references ---

    pub fn hpp_references(
        &self,
        m: &Meta,
        exclude: &str,
        forward_declare_only: bool,
    ) -> Vec<SymbolReference> {
        match m {
            Meta::MPrimitive(p) => match p.idl_name.as_str() {
                "i8" | "i16" | "i32" | "i64" => vec![SymbolReference::ImportRef("<cstdint>".into())],
                _ => vec![],
            },
            Meta::MString => vec![SymbolReference::ImportRef("<string>".into())],
            Meta::MDate => vec![SymbolReference::ImportRef("<chrono>".into())],
            Meta::MBinary => vec![
                SymbolReference::ImportRef("<vector>".into()),
                SymbolReference::ImportRef("<cstdint>".into()),
            ],
            Meta::MOptional => {
                vec![SymbolReference::ImportRef(self.spec.cpp_optional_header.clone())]
            }
            Meta::MList | Meta::MArray => vec![SymbolReference::ImportRef("<vector>".into())],
            Meta::MSet => vec![SymbolReference::ImportRef("<unordered_set>".into())],
            Meta::MMap => vec![SymbolReference::ImportRef("<unordered_map>".into())],
            Meta::MDef(d) => match &d.body {
                TypeDef::Record(r) => {
                    if d.name != exclude {
                        if forward_declare_only {
                            vec![SymbolReference::DeclRef {
                                decl: format!(
                                    "struct {};",
                                    self.typename_from_name(&d.name, &d.body)
                                ),
                                namespace: Some(self.spec.cpp_namespace.clone()),
                            }]
                        } else {
                            vec![SymbolReference::ImportRef(self.include(&d.name, r.ext.cpp))]
                        }
                    } else {
                        vec![]
                    }
                }
                TypeDef::Enum(e) => {
                    if d.name != exclude {
                        if forward_declare_only {
                            let underlying_type = if e.flags { " : int32_t" } else { "" };
                            vec![SymbolReference::DeclRef {
                                decl: format!(
                                    "enum class {}{};",
                                    self.typename_from_name(&d.name, &d.body),
                                    underlying_type
                                ),
                                namespace: Some(self.spec.cpp_namespace.clone()),
                            }]
                        } else {
                            vec![SymbolReference::ImportRef(
                                self.include(&d.name, false),
                            )]
                        }
                    } else {
                        vec![]
                    }
                }
                TypeDef::Interface(_) => {
                    let mut base = if d.name != exclude {
                        vec![
                            SymbolReference::ImportRef("<memory>".into()),
                            SymbolReference::DeclRef {
                                decl: format!(
                                    "class {};",
                                    self.typename_from_name(&d.name, &d.body)
                                ),
                                namespace: Some(self.spec.cpp_namespace.clone()),
                            },
                        ]
                    } else {
                        vec![SymbolReference::ImportRef("<memory>".into())]
                    };
                    if let Some(ref nn_hdr) = self.spec.cpp_nn_header {
                        base.insert(0, SymbolReference::ImportRef(nn_hdr.clone()));
                    }
                    base
                }
                TypeDef::ProtobufMessage(p) => {
                    vec![SymbolReference::ImportRef(p.cpp.header.clone())]
                }
            },
            Meta::MExtern(e) => match e.def_type {
                DefType::Interface => vec![
                    SymbolReference::ImportRef("<memory>".into()),
                    SymbolReference::ImportRef(e.cpp.header.clone()),
                ],
                _ => vec![SymbolReference::ImportRef(
                    self.resolve_ext_cpp_hdr(&e.cpp.header),
                )],
            },
            Meta::MProtobuf(p) => {
                vec![SymbolReference::ImportRef(p.body.cpp.header.clone())]
            }
            Meta::MParam(_) => vec![],
            Meta::MVoid => vec![],
        }
    }

    // --- cpp_references ---

    pub fn cpp_references(
        &self,
        m: &Meta,
        exclude: &str,
        forward_declare_only: bool,
    ) -> Vec<SymbolReference> {
        if !forward_declare_only {
            return vec![];
        }
        match m {
            Meta::MDef(d) => match &d.body {
                TypeDef::Record(r) => {
                    if d.name != exclude {
                        vec![SymbolReference::ImportRef(self.include(&d.name, r.ext.cpp))]
                    } else {
                        vec![]
                    }
                }
                TypeDef::Enum(_) => {
                    if d.name != exclude {
                        vec![SymbolReference::ImportRef(self.include(&d.name, false))]
                    } else {
                        vec![]
                    }
                }
                _ => vec![],
            },
            _ => vec![],
        }
    }

    // --- include ---

    pub fn include(&self, ident: &str, is_extended_record: bool) -> String {
        let prefix = if is_extended_record {
            &self.spec.cpp_extended_record_include_prefix
        } else {
            &self.spec.cpp_include_prefix
        };
        q(&format!(
            "{}{}.{}",
            prefix,
            (self.spec.cpp_file_ident_style)(ident),
            self.spec.cpp_header_ext
        ))
    }

    // --- resolve_ext_cpp_hdr ---

    pub fn resolve_ext_cpp_hdr(&self, path: &str) -> String {
        path.replace('$', &self.spec.cpp_base_lib_include_prefix)
    }

    // --- by_value ---

    pub fn by_value_mexpr(&self, tm: &MExpr) -> bool {
        match &tm.base {
            Meta::MPrimitive(_) => true,
            Meta::MDef(d) => d.def_type == DefType::Enum,
            Meta::MExtern(e) => match e.def_type {
                DefType::Interface => false,
                DefType::Enum => true,
                DefType::Record => e.cpp.by_value,
            },
            Meta::MOptional => {
                if let Some(first) = tm.args.first() {
                    self.by_value_mexpr(first)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn by_value_typedef(&self, td: &TypeDef) -> bool {
        match td {
            TypeDef::Interface(_) => false,
            TypeDef::Record(_) => false,
            TypeDef::Enum(_) => true,
            TypeDef::ProtobufMessage(_) => false,
        }
    }

    // --- extends_record ---

    pub fn extends_record(&self, name: &str) -> String {
        format!(" : public {}", name)
    }

    // --- private helpers ---

    fn to_cpp_type_typeref(
        &self,
        ty: &TypeRef,
        namespace: Option<&str>,
        scope_symbols: &[String],
    ) -> String {
        self.to_cpp_type_mexpr(
            ty.resolved.as_ref().expect("TypeRef not resolved"),
            namespace,
            scope_symbols,
        )
    }

    fn to_cpp_type_mexpr(
        &self,
        tm: &MExpr,
        namespace: Option<&str>,
        scope_symbols: &[String],
    ) -> String {
        let spec = self.spec;
        let id = &spec.cpp_ident_style;

        let with_namespace = |name: &str| -> String {
            let ns = match namespace {
                Some(ns) => Some(ns),
                None => {
                    if scope_symbols.contains(&name.to_string()) {
                        Some(spec.cpp_namespace.as_str())
                    } else {
                        None
                    }
                }
            };
            with_ns(ns, name)
        };

        let base = |m: &Meta| -> String {
            match m {
                Meta::MPrimitive(p) => p.c_name.clone(),
                Meta::MString => {
                    if spec.cpp_use_wide_strings {
                        "std::wstring".into()
                    } else {
                        "std::string".into()
                    }
                }
                Meta::MDate => "std::chrono::system_clock::time_point".into(),
                Meta::MBinary => "std::vector<uint8_t>".into(),
                Meta::MOptional => spec.cpp_optional_template.clone(),
                Meta::MList | Meta::MArray => "std::vector".into(),
                Meta::MSet => "std::unordered_set".into(),
                Meta::MMap => "std::unordered_map".into(),
                Meta::MDef(d) => match d.def_type {
                    DefType::Enum => with_namespace(&(id.enum_type)(&d.name)),
                    DefType::Record => with_namespace(&(id.ty)(&d.name)),
                    DefType::Interface => {
                        format!("std::shared_ptr<{}>", with_namespace(&(id.ty)(&d.name)))
                    }
                },
                Meta::MExtern(e) => match e.def_type {
                    DefType::Interface => {
                        format!("std::shared_ptr<{}>", e.cpp.typename)
                    }
                    _ => e.cpp.typename.clone(),
                },
                Meta::MParam(p) => (id.type_param)(&p.name),
                Meta::MProtobuf(p) => with_ns(Some(&p.body.cpp.ns), &p.name),
                Meta::MVoid => "void".into(),
            }
        };

        self.expr(tm, &base, &with_namespace)
    }

    fn expr(
        &self,
        tm: &MExpr,
        base_fn: &dyn Fn(&Meta) -> String,
        with_namespace: &dyn Fn(&str) -> String,
    ) -> String {
        let spec = self.spec;
        let id = &spec.cpp_ident_style;

        match &spec.cpp_nn_type {
            Some(nn_type) => {
                let args = if tm.args.is_empty() {
                    String::new()
                } else {
                    let parts: Vec<String> = tm
                        .args
                        .iter()
                        .map(|a| self.expr(a, base_fn, with_namespace))
                        .collect();
                    format!("<{}>", parts.join(", "))
                };
                match &tm.base {
                    Meta::MDef(d) if d.def_type == DefType::Interface => {
                        format!(
                            "{}<{}>",
                            nn_type,
                            with_namespace(&(id.ty)(&d.name))
                        )
                    }
                    Meta::MOptional => {
                        if let Some(first_arg) = tm.args.first() {
                            if let Meta::MDef(d) = &first_arg.base {
                                if d.def_type == DefType::Interface {
                                    return format!(
                                        "std::shared_ptr<{}>",
                                        with_namespace(&(id.ty)(&d.name))
                                    );
                                }
                            }
                        }
                        format!("{}{}", base_fn(&tm.base), args)
                    }
                    _ => format!("{}{}", base_fn(&tm.base), args),
                }
            }
            None => {
                let ty = if is_optional_interface(tm) {
                    &tm.args[0]
                } else {
                    tm
                };
                let prefix = if !is_interface(ty) {
                    ""
                } else if is_optional(tm) {
                    "/*nullable*/ "
                } else {
                    "/*not-null*/ "
                };
                let args = if ty.args.is_empty() {
                    String::new()
                } else {
                    let parts: Vec<String> = ty
                        .args
                        .iter()
                        .map(|a| self.expr(a, base_fn, with_namespace))
                        .collect();
                    format!("<{}>", parts.join(", "))
                };
                format!("{}{}{}", prefix, base_fn(&ty.base), args)
            }
        }
    }

    fn to_cpp_param_type(
        &self,
        tm: &MExpr,
        namespace: Option<&str>,
        scope_symbols: &[String],
    ) -> String {
        let cpp_type = self.to_cpp_type_mexpr(tm, namespace, scope_symbols);
        if self.by_value_mexpr(tm) {
            cpp_type
        } else {
            format!("const {} &", cpp_type)
        }
    }
}
