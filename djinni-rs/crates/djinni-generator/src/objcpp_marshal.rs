// vinsg
// ObjcppMarshal: Objective-C++ type marshalling, translated from ObjcppMarshal.scala

use djinni_ast::ast::*;
use djinni_ast::meta::*;
use djinni_ast::spec::{q, Spec};

use crate::cpp_marshal::{CppMarshal, SymbolReference};
use crate::gen::with_ns;
use crate::objc_marshal::ObjcMarshal;

pub struct ObjcppMarshal<'a> {
    pub spec: &'a Spec,
}

impl<'a> ObjcppMarshal<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        ObjcppMarshal { spec }
    }

    fn id_objc_ty(&self, name: &str) -> String {
        format!(
            "{}{}",
            self.spec.objc_type_prefix,
            (self.spec.objc_ident_style.ty)(name)
        )
    }

    fn id_cpp_ty(&self, name: &str) -> String {
        (self.spec.cpp_ident_style.ty)(name)
    }

    // --- toCpp / fromCpp ---

    pub fn to_cpp(&self, ty: &TypeRef, expr: &str) -> String {
        let tm = ty.resolved.as_ref().expect("TypeRef not resolved");
        self.to_cpp_mexpr(tm, expr)
    }

    pub fn to_cpp_mexpr(&self, tm: &MExpr, expr: &str) -> String {
        format!("{}::toCpp({})", self.helper_class(tm), expr)
    }

    pub fn from_cpp(&self, ty: &TypeRef, expr: &str) -> String {
        let tm = ty.resolved.as_ref().expect("TypeRef not resolved");
        self.from_cpp_mexpr(tm, expr)
    }

    pub fn from_cpp_mexpr(&self, tm: &MExpr, expr: &str) -> String {
        format!("{}::fromCpp({})", self.helper_class(tm), expr)
    }

    // --- references ---

    pub fn references(&self, m: &Meta) -> Vec<SymbolReference> {
        match m {
            Meta::MPrimitive(_) | Meta::MString | Meta::MDate | Meta::MBinary
            | Meta::MOptional | Meta::MList | Meta::MSet | Meta::MMap | Meta::MArray
            | Meta::MVoid => {
                vec![SymbolReference::ImportRef(q(&format!(
                    "{}DJIMarshal+Private.h",
                    self.spec.objc_base_lib_include_prefix
                )))]
            }
            Meta::MProtobuf(p) => {
                let mut refs = vec![SymbolReference::ImportRef(q(&format!(
                    "{}DJIMarshal+Private.h",
                    self.spec.objc_base_lib_include_prefix
                )))];
                if let Some(o) = &p.body.objc {
                    refs.push(SymbolReference::ImportRef(o.header.clone()));
                }
                refs
            }
            Meta::MDef(d) => match &d.body {
                TypeDef::Enum(_) | TypeDef::Interface(_) => {
                    vec![SymbolReference::ImportRef(self.include(m))]
                }
                TypeDef::Record(r) => {
                    let objc_name = if r.ext.objc {
                        format!("{}_base", d.name)
                    } else {
                        d.name.clone()
                    };
                    vec![SymbolReference::ImportRef(q(&format!(
                        "{}{}",
                        self.spec.objcpp_include_prefix,
                        self.private_header_name(&objc_name)
                    )))]
                }
                TypeDef::ProtobufMessage(_) => vec![],
            },
            Meta::MExtern(e) => {
                vec![SymbolReference::ImportRef(
                    self.resolve_ext_objcpp_hdr(&e.objcpp.header),
                )]
            }
            Meta::MParam(_) => vec![],
        }
    }

    pub fn resolve_ext_objcpp_hdr(&self, path: &str) -> String {
        path.replace('$', &self.spec.objc_base_lib_include_prefix)
    }

    pub fn include(&self, m: &Meta) -> String {
        match m {
            Meta::MDef(d) => q(&format!(
                "{}{}",
                self.spec.objcpp_include_prefix,
                self.private_header_name(&d.name)
            )),
            _ => panic!("not applicable"),
        }
    }

    pub fn helper_class_name(&self, name: &str) -> String {
        self.id_cpp_ty(name)
    }

    pub fn helper_class_with_ns(&self, name: &str) -> String {
        with_ns(
            Some(&self.spec.objcpp_namespace),
            &self.helper_class_name(name),
        )
    }

    pub fn private_header_name(&self, ident: &str) -> String {
        format!(
            "{}+Private.{}",
            self.id_objc_ty(ident),
            self.spec.objc_header_ext
        )
    }

    fn helper_class(&self, tm: &MExpr) -> String {
        format!("{}{}", self.helper_name(tm), self.helper_templates(tm))
    }

    fn helper_name(&self, tm: &MExpr) -> String {
        let cpp_marshal = CppMarshal::new(self.spec);
        let objc_marshal = ObjcMarshal::new(self.spec);

        match &tm.base {
            Meta::MDef(d) => match d.def_type {
                DefType::Enum => {
                    with_ns(
                        Some("djinni"),
                        &format!(
                            "Enum<{}, {}>",
                            cpp_marshal.fq_typename_from_mexpr(tm),
                            objc_marshal.fq_typename_from_mexpr(tm)
                        ),
                    )
                }
                _ => with_ns(
                    Some(&self.spec.objcpp_namespace),
                    &self.helper_class_name(&d.name),
                ),
            },
            Meta::MExtern(e) => e.objcpp.translator.clone(),
            Meta::MProtobuf(p) => match &p.body.objc {
                Some(o) => {
                    format!(
                        "{}<{}, {}{}>",
                        with_ns(Some("djinni"), "Protobuf"),
                        with_ns(Some(&p.body.cpp.ns), &p.name),
                        o.prefix,
                        p.name
                    )
                }
                None => {
                    format!(
                        "{}<{}>",
                        with_ns(Some("djinni"), "ProtobufPassthrough"),
                        with_ns(Some(&p.body.cpp.ns), &p.name)
                    )
                }
            },
            other => {
                let name = match other {
                    Meta::MPrimitive(p) => match p.idl_name.as_str() {
                        "i8" => "I8",
                        "i16" => "I16",
                        "i32" => "I32",
                        "i64" => "I64",
                        "f32" => "F32",
                        "f64" => "F64",
                        "bool" => "Bool",
                        _ => panic!("unknown primitive {}", p.idl_name),
                    },
                    Meta::MOptional => "Optional",
                    Meta::MBinary => "Binary",
                    Meta::MDate => "Date",
                    Meta::MString => {
                        if self.spec.cpp_use_wide_strings {
                            "WString"
                        } else {
                            "String"
                        }
                    }
                    Meta::MList => "List",
                    Meta::MSet => "Set",
                    Meta::MMap => "Map",
                    Meta::MArray => "Array",
                    Meta::MVoid => "Void",
                    _ => panic!("unreachable in helper_name"),
                };
                with_ns(Some("djinni"), name)
            }
        }
    }

    fn helper_templates(&self, tm: &MExpr) -> String {
        let f = || {
            if tm.args.is_empty() {
                String::new()
            } else {
                let parts: Vec<String> = tm.args.iter().map(|a| self.helper_class(a)).collect();
                format!("<{}>", parts.join(", "))
            }
        };

        match &tm.base {
            Meta::MOptional => {
                assert_eq!(tm.args.len(), 1);
                let arg_helper = self.helper_class(&tm.args[0]);
                format!("<{}, {}>", self.spec.cpp_optional_template, arg_helper)
            }
            Meta::MList | Meta::MSet | Meta::MArray => {
                assert_eq!(tm.args.len(), 1);
                f()
            }
            _ => f(),
        }
    }
}
