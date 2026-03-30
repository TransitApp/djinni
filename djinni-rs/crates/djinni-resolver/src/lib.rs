// vinsg
// Type resolution and validation, translated from resolver.scala

use std::collections::HashMap;

use djinni_ast::ast::*;
use djinni_ast::meta::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("{loc}: {message}")]
    Error { loc: Loc, message: String },
}

type Scope = HashMap<String, Meta>;

pub fn resolve(
    metas: &HashMap<String, Meta>,
    idl: &mut [TypeDecl],
) -> Result<(), ResolveError> {
    let mut top_scope = metas.clone();

    // Load top-level names into scope
    let mut dupe_checker = DupeChecker::new("type");
    for td in idl.iter() {
        dupe_checker.check(td.ident())?;

        let meta = match td {
            TypeDecl::Intern {
                ident,
                params,
                body,
                ..
            } => {
                let def_type = match body {
                    TypeDef::Enum(_) => {
                        if !params.is_empty() {
                            return Err(ResolveError::Error {
                                loc: ident.loc.clone(),
                                message: "enums can't have type parameters".into(),
                            });
                        }
                        DefType::Enum
                    }
                    TypeDef::Record(_) => DefType::Record,
                    TypeDef::Interface(_) => DefType::Interface,
                    TypeDef::ProtobufMessage(_) => unreachable!(),
                };
                Meta::MDef(MDef {
                    name: ident.name.clone(),
                    num_params: params.len(),
                    def_type,
                    body: body.clone(),
                })
            }
            TypeDecl::Extern {
                ident,
                params,
                body,
                properties,
                ..
            } => {
                meta_from_yaml(ident, params, body, properties)
            }
            TypeDecl::Protobuf { ident, body, .. } => {
                if let TypeDef::ProtobufMessage(proto) = body {
                    Meta::MProtobuf(MProtobuf {
                        name: ident.name.clone(),
                        num_params: 0,
                        body: proto.clone(),
                    })
                } else {
                    unreachable!()
                }
            }
        };
        top_scope.insert(td.ident().name.clone(), meta);
    }

    // Resolve type references
    for td in idl.iter_mut() {
        let mut scope = top_scope.clone();

        // Load type parameters into scope
        let mut tp_dupe_checker = DupeChecker::new("type parameter");
        for tp in td.params() {
            tp_dupe_checker.check(&tp.ident)?;
            scope.insert(
                tp.ident.name.clone(),
                Meta::MParam(MParam {
                    name: tp.ident.name.clone(),
                }),
            );
        }

        resolve_type_def(&scope, td.body_mut())?;
    }

    // Second pass: update MDef.body inside resolved MExprs with now-resolved bodies.
    // The MDef.body was cloned before resolution, so field TypeRefs are None.
    // Build a map of resolved bodies, then fix up all MExpr trees.
    let resolved_bodies: HashMap<String, TypeDef> = idl
        .iter()
        .filter_map(|td| {
            if let TypeDecl::Intern { ident, body, .. } = td {
                Some((ident.name.clone(), body.clone()))
            } else {
                None
            }
        })
        .collect();

    for td in idl.iter_mut() {
        let mut visited = std::collections::HashSet::new();
        fixup_typedef(td.body_mut(), &resolved_bodies, &mut visited);
    }

    Ok(())
}

fn fixup_typedef(td: &mut TypeDef, resolved: &HashMap<String, TypeDef>, visited: &mut std::collections::HashSet<String>) {
    match td {
        TypeDef::Record(r) => {
            for f in &mut r.fields {
                fixup_typeref(&mut f.ty, resolved, visited);
            }
            for c in &mut r.consts {
                fixup_typeref(&mut c.ty, resolved, visited);
            }
        }
        TypeDef::Interface(i) => {
            for m in &mut i.methods {
                for p in &mut m.params {
                    fixup_typeref(&mut p.ty, resolved, visited);
                }
                if let Some(ref mut ret) = m.ret {
                    fixup_typeref(ret, resolved, visited);
                }
            }
            for c in &mut i.consts {
                fixup_typeref(&mut c.ty, resolved, visited);
            }
        }
        _ => {}
    }
}

fn fixup_typeref(tr: &mut TypeRef, resolved: &HashMap<String, TypeDef>, visited: &mut std::collections::HashSet<String>) {
    if let Some(ref mut mexpr) = tr.resolved {
        fixup_mexpr(mexpr, resolved, visited);
    }
}

fn fixup_mexpr(mexpr: &mut MExpr, resolved: &HashMap<String, TypeDef>, visited: &mut std::collections::HashSet<String>) {
    if let Meta::MDef(ref mut d) = mexpr.base {
        if !visited.contains(&d.name) {
            visited.insert(d.name.clone());
            if let Some(body) = resolved.get(&d.name) {
                d.body = body.clone();
            }
            fixup_typedef(&mut d.body, resolved, visited);
        }
    }
    for arg in &mut mexpr.args {
        fixup_mexpr(arg, resolved, visited);
    }
}

fn resolve_type_def(scope: &Scope, type_def: &mut TypeDef) -> Result<(), ResolveError> {
    match type_def {
        TypeDef::Enum(e) => resolve_enum(e),
        TypeDef::Record(r) => resolve_record(scope, r),
        TypeDef::Interface(i) => resolve_interface(scope, i),
        TypeDef::ProtobufMessage(_) => Ok(()),
    }
}

fn resolve_enum(e: &Enum) -> Result<(), ResolveError> {
    let mut dupe_checker = DupeChecker::new("enum option");
    for opt in &e.options {
        dupe_checker.check(&opt.ident)?;
    }
    Ok(())
}

fn resolve_record(scope: &Scope, r: &mut Record) -> Result<(), ResolveError> {
    let mut dupe_checker = DupeChecker::new("record field");
    for f in &mut r.fields {
        dupe_checker.check(&f.ident)?;
        resolve_ref(scope, &mut f.ty)?;
    }
    for c in &mut r.consts {
        dupe_checker.check(&c.ident)?;
        resolve_ref(scope, &mut c.ty)?;
    }
    Ok(())
}

fn resolve_interface(scope: &Scope, i: &mut Interface) -> Result<(), ResolveError> {
    let mut dupe_checker = DupeChecker::new("method");
    for m in &mut i.methods {
        dupe_checker.check(&m.ident)?;
        for p in &mut m.params {
            resolve_ref(scope, &mut p.ty)?;
        }
        if let Some(ref mut ty) = m.ret {
            resolve_ref(scope, ty)?;
        }
    }
    for c in &mut i.consts {
        dupe_checker.check(&c.ident)?;
        resolve_ref(scope, &mut c.ty)?;
    }
    Ok(())
}

fn resolve_ref(scope: &Scope, r: &mut TypeRef) -> Result<(), ResolveError> {
    r.resolved = Some(build_mexpr(scope, &r.expr)?);
    Ok(())
}

fn build_mexpr(scope: &Scope, e: &TypeExpr) -> Result<MExpr, ResolveError> {
    match scope.get(&e.ident.name) {
        Some(meta) => {
            if meta.num_params() != e.args.len() {
                return Err(ResolveError::Error {
                    loc: e.ident.loc.clone(),
                    message: format!(
                        "incorrect number of arguments to type \"{}\"; expecting {}, got {}",
                        e.ident.name,
                        meta.num_params(),
                        e.args.len()
                    ),
                });
            }
            let margs: Vec<MExpr> = e
                .args
                .iter()
                .map(|arg| build_mexpr(scope, arg))
                .collect::<Result<_, _>>()?;

            if matches!(meta, Meta::MOptional)
                && !margs.is_empty()
                && matches!(margs[0].base, Meta::MOptional)
            {
                return Err(ResolveError::Error {
                    loc: e.ident.loc.clone(),
                    message: "directly nested optionals not allowed".into(),
                });
            }

            Ok(MExpr {
                base: meta.clone(),
                args: margs,
            })
        }
        None => Err(ResolveError::Error {
            loc: e.ident.loc.clone(),
            message: format!("unknown type \"{}\"", e.ident.name),
        }),
    }
}

// Duplicate name checker
struct DupeChecker {
    kind: String,
    names: HashMap<String, Loc>,
}

impl DupeChecker {
    fn new(kind: &str) -> Self {
        Self {
            kind: kind.to_string(),
            names: HashMap::new(),
        }
    }

    fn check(&mut self, ident: &Ident) -> Result<(), ResolveError> {
        if let Some(existing) = self.names.get(&ident.name) {
            return Err(ResolveError::Error {
                loc: ident.loc.clone(),
                message: format!(
                    "duplicate {} \"{}\" (previous definition: {})",
                    self.kind, ident.name, existing
                ),
            });
        }
        self.names.insert(ident.name.clone(), ident.loc.clone());
        Ok(())
    }
}

// Convert extern YAML properties to MExtern meta
fn meta_from_yaml(
    ident: &Ident,
    params: &[TypeParam],
    body: &TypeDef,
    properties: &HashMap<String, serde_yaml::Value>,
) -> Meta {
    let def_type = match body {
        TypeDef::Interface(_) => DefType::Interface,
        TypeDef::Record(_) => DefType::Record,
        TypeDef::Enum(_) => DefType::Enum,
        TypeDef::ProtobufMessage(_) => unreachable!(),
    };

    let prefix = properties
        .get("prefix")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let name = ident.name.strip_prefix(prefix).unwrap_or(&ident.name);

    let cpp = nested_map(properties, "cpp");
    let objc = nested_map(properties, "objc");
    let objcpp = nested_map(properties, "objcpp");
    let java = nested_map(properties, "java");
    let jni = nested_map(properties, "jni");
    let wasm = nested_map(properties, "wasm");
    let ts = nested_map(properties, "ts");

    Meta::MExtern(Box::new(MExtern {
        name: name.to_string(),
        num_params: params.len(),
        def_type,
        body: body.clone(),
        cpp: ExternCpp {
            typename: yaml_field_str(&cpp, "typename"),
            header: yaml_field_str(&cpp, "header"),
            by_value: yaml_field_bool(&cpp, "byValue", false),
            move_only: yaml_field_bool(&cpp, "moveOnly", false),
        },
        objc: ExternObjc {
            typename: yaml_field_str(&objc, "typename"),
            header: yaml_field_str(&objc, "header"),
            boxed: yaml_field_str(&objc, "boxed"),
            pointer: yaml_field_bool(&objc, "pointer", false),
            generic: yaml_field_bool(&objc, "generic", false),
            hash: yaml_field_str(&objc, "hash"),
            equal: yaml_field_str_default(&objc, "equal", "isEqual:"),
            protocol: yaml_field_bool(&objc, "protocol", false),
        },
        objcpp: ExternObjcpp {
            translator: yaml_field_str(&objcpp, "translator"),
            header: yaml_field_str(&objcpp, "header"),
        },
        java: ExternJava {
            typename: yaml_field_str(&java, "typename"),
            boxed: yaml_field_str(&java, "boxed"),
            reference: yaml_field_bool(&java, "reference", false),
            generic: yaml_field_bool(&java, "generic", false),
            hash: yaml_field_str(&java, "hash"),
            write_to_parcel: yaml_field_str_default(
                &java,
                "writeToParcel",
                "%s.writeToParcel(out, flags)",
            ),
            read_from_parcel: yaml_field_str_default(&java, "readFromParcel", "new %s(in)"),
        },
        jni: ExternJni {
            translator: yaml_field_str(&jni, "translator"),
            header: yaml_field_str(&jni, "header"),
            typename: yaml_field_str(&jni, "typename"),
            type_signature: yaml_field_str(&jni, "typeSignature"),
        },
        wasm: ExternWasm {
            typename: yaml_field_str_or_unspecified(&wasm, "typename"),
            translator: yaml_field_str_or_unspecified(&wasm, "translator"),
            header: yaml_field_str_or_unspecified(&wasm, "header"),
        },
        ts: ExternTs {
            typename: yaml_field_str_or_unspecified(&ts, "typename"),
            module: yaml_field_str_or_unspecified(&ts, "module"),
            generic: yaml_field_bool(&ts, "generic", false),
        },
    }))
}

fn nested_map<'a>(
    props: &'a HashMap<String, serde_yaml::Value>,
    key: &str,
) -> HashMap<String, serde_yaml::Value> {
    props
        .get(key)
        .and_then(|v| v.as_mapping())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| k.as_str().map(|s| (s.to_string(), v.clone())))
                .collect()
        })
        .unwrap_or_default()
}

fn yaml_field_str(map: &HashMap<String, serde_yaml::Value>, key: &str) -> String {
    map.get(key)
        .map(|v| match v {
            serde_yaml::Value::String(s) => s.clone(),
            serde_yaml::Value::Bool(b) => b.to_string(),
            serde_yaml::Value::Number(n) => n.to_string(),
            _ => v.as_str().unwrap_or("").to_string(),
        })
        .unwrap_or_default()
}

fn yaml_field_str_default(
    map: &HashMap<String, serde_yaml::Value>,
    key: &str,
    default: &str,
) -> String {
    map.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

fn yaml_field_str_or_unspecified(
    map: &HashMap<String, serde_yaml::Value>,
    key: &str,
) -> String {
    map.get(key)
        .map(|v| match v {
            serde_yaml::Value::String(s) => s.clone(),
            _ => v.as_str().unwrap_or("[unspecified]").to_string(),
        })
        .unwrap_or_else(|| "[unspecified]".to_string())
}

fn yaml_field_bool(
    map: &HashMap<String, serde_yaml::Value>,
    key: &str,
    default: bool,
) -> bool {
    map.get(key)
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use djinni_parser::ParserContext;
    use std::path::PathBuf;

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
    fn test_resolve_all_test_suite() {
        let root = repo_root();
        let test_suite = root.join("test-suite");

        let mut ctx = ParserContext::new(vec![
            String::new(),
            test_suite
                .join("djinni")
                .join("vendor")
                .to_string_lossy()
                .to_string(),
        ]);
        let mut in_files = Vec::new();
        let (mut types, _flags) = ctx
            .parse_file(&test_suite.join("djinni").join("all.djinni"), &mut in_files)
            .expect("parse failed");

        let defaults = defaults();
        let result = resolve(&defaults, &mut types);
        assert!(result.is_ok(), "Resolve failed: {:?}", result.err());

        // Check that all type refs are resolved
        let mut resolved_count = 0;
        for td in &types {
            if let TypeDef::Record(r) = td.body() {
                for f in &r.fields {
                    assert!(
                        f.ty.resolved.is_some(),
                        "Unresolved field type: {} in {}",
                        f.ident.name,
                        td.ident().name
                    );
                    resolved_count += 1;
                }
            }
        }
        eprintln!("Resolved {} field types across {} type declarations", resolved_count, types.len());
    }
}
