// vinsg
// JavaMarshal: shared Java/Kotlin type marshalling, translated from JavaMarshal.scala

use djinni_ast::ast::*;
use djinni_ast::meta::*;
use djinni_ast::spec::Spec;

use crate::cpp_marshal::SymbolReference;

pub struct JavaMarshal<'a> {
    pub spec: &'a Spec,
    pub kotlin: bool,
    pub java_nullable_annotation: Option<String>,
    pub java_nonnull_annotation: Option<String>,
    pub interface_nullity_annotation: Option<String>,
}

impl<'a> JavaMarshal<'a> {
    pub fn new(spec: &'a Spec, kotlin: bool) -> Self {
        let java_nullable_annotation = spec
            .java_nullable_annotation
            .as_ref()
            .map(|pkg| format!("@{}", pkg.split('.').last().unwrap_or(pkg)));
        let java_nonnull_annotation = spec
            .java_nonnull_annotation
            .as_ref()
            .map(|pkg| format!("@{}", pkg.split('.').last().unwrap_or(pkg)));
        let interface_nullity_annotation = if spec.cpp_nn_type.is_some() {
            java_nonnull_annotation.clone()
        } else {
            java_nullable_annotation.clone()
        };
        JavaMarshal {
            spec,
            kotlin,
            java_nullable_annotation,
            java_nonnull_annotation,
            interface_nullity_annotation,
        }
    }

    // --- typename ---

    pub fn typename_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_java_type(tm, &None)
    }

    pub fn typename_from_name(&self, name: &str, _ty: &TypeDef) -> String {
        (self.spec.java_ident_style.ty)(name)
    }

    pub fn typename_from_typeref(&self, ty: &TypeRef) -> String {
        match &ty.resolved {
            Some(tm) => self.to_java_type(tm, &None),
            None => String::new(),
        }
    }

    // --- fq_typename ---

    pub fn fq_typename_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_java_type(tm, &self.spec.java_package)
    }

    pub fn fq_typename_from_name(&self, name: &str, _ty: &TypeDef) -> String {
        self.with_package(&self.spec.java_package, &(self.spec.java_ident_style.ty)(name))
    }

    // --- param_type ---

    pub fn param_type_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_java_value_type(tm, &None)
    }

    pub fn fq_param_type_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_java_value_type(tm, &self.spec.java_package)
    }

    // --- return_type ---

    pub fn return_type(&self, ret: &Option<TypeRef>) -> String {
        let void_type = if self.kotlin { "" } else { "void" };
        match ret {
            None => void_type.to_string(),
            Some(ty) => match &ty.resolved {
                Some(tm) => self.to_java_value_type(tm, &None),
                None => void_type.to_string(),
            },
        }
    }

    pub fn fq_return_type(&self, ret: &Option<TypeRef>) -> String {
        let void_type = if self.kotlin { "" } else { "void" };
        match ret {
            None => void_type.to_string(),
            Some(ty) => match &ty.resolved {
                Some(tm) => self.to_java_value_type(tm, &self.spec.java_package),
                None => void_type.to_string(),
            },
        }
    }

    // --- field_type ---

    pub fn field_type_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_java_value_type(tm, &None)
    }

    pub fn field_type_from_typeref(&self, ty: &TypeRef) -> String {
        match &ty.resolved {
            Some(tm) => self.to_java_value_type(tm, &None),
            None => String::new(),
        }
    }

    // --- nullity annotations ---

    pub fn nullity_annotation_opt(&self, ty: &Option<TypeRef>) -> Option<String> {
        match ty {
            Some(t) => self.nullity_annotation(t),
            None => None,
        }
    }

    pub fn nullity_annotation(&self, ty: &TypeRef) -> Option<String> {
        match &ty.resolved {
            Some(resolved) => match &resolved.base {
                Meta::MOptional => self.java_nullable_annotation.clone(),
                Meta::MPrimitive(_) => None,
                Meta::MDef(d) => match d.def_type {
                    DefType::Interface => self.interface_nullity_annotation.clone(),
                    DefType::Enum | DefType::Record => self.java_nonnull_annotation.clone(),
                },
                Meta::MExtern(e) => match e.def_type {
                    DefType::Interface => self.interface_nullity_annotation.clone(),
                    DefType::Record => {
                        if e.java.reference {
                            self.java_nonnull_annotation.clone()
                        } else {
                            None
                        }
                    }
                    DefType::Enum => self.java_nonnull_annotation.clone(),
                },
                _ => self.java_nonnull_annotation.clone(),
            },
            None => None,
        }
    }

    // --- references (for imports) ---

    pub fn references(&self, m: &Meta) -> Vec<SymbolReference> {
        match m {
            Meta::MList => vec![SymbolReference::ImportRef("java.util.ArrayList".into())],
            Meta::MSet => vec![SymbolReference::ImportRef("java.util.HashSet".into())],
            Meta::MMap => vec![SymbolReference::ImportRef("java.util.HashMap".into())],
            Meta::MDate => vec![SymbolReference::ImportRef("java.util.Date".into())],
            Meta::MProtobuf(p) => {
                let pkg = &p.body.java.pkg;
                vec![SymbolReference::ImportRef(
                    self.with_package(&Some(pkg.clone()), &p.name),
                )]
            }
            Meta::MDef(d) => {
                if is_enum_flags_typedef(&d.body) {
                    vec![SymbolReference::ImportRef("java.util.EnumSet".into())]
                } else {
                    vec![]
                }
            }
            Meta::MExtern(e) => {
                if is_enum_flags_typedef(&e.body) {
                    vec![SymbolReference::ImportRef("java.util.EnumSet".into())]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }

    // --- extends record ---

    pub fn extends_record(&self, idl: &[TypeDecl], r: &Record) -> String {
        match &r.base_record {
            None => String::new(),
            Some(base_name) => {
                if let Some(td) = idl.iter().find(|td| td.ident().name == *base_name) {
                    let name = self.typename_from_name(&td.ident().name, td.body());
                    if self.kotlin {
                        format!(" : {}", name)
                    } else {
                        format!(" extends {}", name)
                    }
                } else {
                    String::new()
                }
            }
        }
    }

    // --- private helpers ---

    fn to_java_value_type(&self, tm: &MExpr, package_name: &Option<String>) -> String {
        let name = self.to_java_type(tm, package_name);
        if self.is_enum_flags_mexpr(tm) {
            format!("EnumSet<{}>", name)
        } else {
            name
        }
    }

    fn to_java_type(&self, tm: &MExpr, package_name: &Option<String>) -> String {
        self.to_java_type_inner(tm, package_name, false)
    }

    fn to_java_type_inner(
        &self,
        tm: &MExpr,
        package_name: &Option<String>,
        need_ref: bool,
    ) -> String {
        let args_str = if tm.args.is_empty() {
            String::new()
        } else {
            let inner: Vec<String> = tm
                .args
                .iter()
                .map(|a| self.to_java_type_inner(a, package_name, true))
                .collect();
            format!("<{}>", inner.join(", "))
        };

        match &tm.base {
            Meta::MOptional => {
                assert_eq!(tm.args.len(), 1);
                let arg = &tm.args[0];
                match &arg.base {
                    Meta::MPrimitive(p) => {
                        if self.kotlin {
                            format!("{}?", p.k_name)
                        } else {
                            p.j_boxed.clone()
                        }
                    }
                    Meta::MOptional => panic!("nested optional?"),
                    _ => {
                        let inner = self.to_java_type_inner(arg, package_name, true);
                        if self.kotlin {
                            format!("{}?", inner)
                        } else {
                            inner
                        }
                    }
                }
            }
            Meta::MArray => {
                let inner = self.to_java_type_inner(&tm.args[0], package_name, false);
                format!("{}[]", inner)
            }
            Meta::MExtern(e) if self.kotlin => {
                let base = e.java.boxed.clone();
                if e.java.generic {
                    format!("{}{}", base, args_str)
                } else {
                    base
                }
            }
            Meta::MExtern(e) => {
                let base = if need_ref {
                    e.java.boxed.clone()
                } else {
                    e.java.typename.clone()
                };
                if e.java.generic {
                    format!("{}{}", base, args_str)
                } else {
                    base
                }
            }
            Meta::MProtobuf(p) => p.name.clone(),
            Meta::MPrimitive(p) if self.kotlin => p.k_name.clone(),
            Meta::MPrimitive(p) => {
                if need_ref {
                    p.j_boxed.clone()
                } else {
                    p.j_name.clone()
                }
            }
            Meta::MString => "String".into(),
            Meta::MDate => "Date".into(),
            Meta::MBinary => {
                if self.kotlin {
                    "ByteArray".into()
                } else {
                    "byte[]".into()
                }
            }
            Meta::MList => format!("ArrayList{}", args_str),
            Meta::MSet => format!("HashSet{}", args_str),
            Meta::MMap => format!("HashMap{}", args_str),
            Meta::MDef(d) => {
                let base = self.with_package(
                    package_name,
                    &(self.spec.java_ident_style.ty)(&d.name),
                );
                format!("{}{}", base, args_str)
            }
            Meta::MParam(p) => (self.spec.java_ident_style.type_param)(&p.name),
            Meta::MVoid => "Void".into(),
        }
    }

    fn with_package(&self, package_name: &Option<String>, t: &str) -> String {
        match package_name {
            Some(pkg) => format!("{}.{}", pkg, t),
            None => t.to_string(),
        }
    }

    fn is_enum_flags_mexpr(&self, tm: &MExpr) -> bool {
        match &tm.base {
            Meta::MOptional => {
                if !tm.args.is_empty() {
                    self.is_enum_flags_meta(&tm.args[0].base)
                } else {
                    false
                }
            }
            _ => self.is_enum_flags_meta(&tm.base),
        }
    }

    fn is_enum_flags_meta(&self, m: &Meta) -> bool {
        match m {
            Meta::MDef(d) => is_enum_flags_typedef(&d.body),
            Meta::MExtern(e) => is_enum_flags_typedef(&e.body),
            _ => false,
        }
    }
}

pub fn is_enum_flags_typedef(td: &TypeDef) -> bool {
    if let TypeDef::Enum(e) = td {
        e.flags
    } else {
        false
    }
}
