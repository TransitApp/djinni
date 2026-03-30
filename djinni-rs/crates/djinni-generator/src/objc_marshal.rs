// vinsg
// ObjcMarshal: Objective-C type marshalling, translated from ObjcMarshal.scala

use djinni_ast::ast::*;
use djinni_ast::meta::*;
use djinni_ast::spec::{q, Spec};

use crate::cpp_marshal::SymbolReference;

pub struct ObjcMarshal<'a> {
    pub spec: &'a Spec,
}

impl<'a> ObjcMarshal<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        ObjcMarshal { spec }
    }

    fn id_objc_ty(&self, name: &str) -> String {
        format!("{}{}", self.spec.objc_type_prefix, (self.spec.objc_ident_style.ty)(name))
    }

    #[allow(dead_code)]
    fn id_objc_file(&self, name: &str) -> String {
        format!("{}{}", self.spec.objc_type_prefix, (self.spec.objc_file_ident_style)(name))
    }

    // --- typename ---

    pub fn typename_from_mexpr(&self, tm: &MExpr) -> String {
        let (name, _) = self.to_objc_type(tm, false);
        name
    }

    pub fn typename_from_name(&self, _name: &str, _ty: &TypeDef) -> String {
        self.id_objc_ty(_name)
    }

    pub fn fq_typename_from_mexpr(&self, tm: &MExpr) -> String {
        self.typename_from_mexpr(tm)
    }

    pub fn fq_typename_from_name(&self, name: &str, ty: &TypeDef) -> String {
        self.typename_from_name(name, ty)
    }

    // --- nullability ---

    pub fn nullability(&self, tm: &MExpr) -> Option<String> {
        let nonnull = Some("nonnull".to_string());
        let nullable = Some("nullable".to_string());
        let interface_nullity = if self.spec.cpp_nn_type.is_some() {
            nonnull.clone()
        } else {
            nullable.clone()
        };
        match &tm.base {
            Meta::MOptional => nullable,
            Meta::MPrimitive(_) => None,
            Meta::MDef(d) => match d.def_type {
                DefType::Enum => None,
                DefType::Interface => interface_nullity,
                DefType::Record => nonnull,
            },
            Meta::MExtern(e) => match e.def_type {
                DefType::Enum => None,
                DefType::Interface => interface_nullity,
                DefType::Record => {
                    if e.objc.pointer {
                        nonnull
                    } else {
                        None
                    }
                }
            },
            Meta::MProtobuf(p) => {
                if p.body.objc.is_none() {
                    None
                } else {
                    nonnull
                }
            }
            _ => nonnull,
        }
    }

    // --- cpp_proto_type ---

    fn cpp_proto_type(&self, tm: &MExpr) -> Option<String> {
        if let Meta::MProtobuf(p) = &tm.base {
            if p.body.objc.is_none() {
                return Some(format!("{}::{}", p.body.cpp.ns, p.name));
            }
        }
        None
    }

    // --- param_type ---

    pub fn param_type(&self, ty: &TypeRef) -> String {
        let tm = ty.resolved.as_ref().expect("TypeRef not resolved");
        self.param_type_mexpr(tm)
    }

    pub fn param_type_mexpr(&self, tm: &MExpr) -> String {
        if let Some(t) = self.cpp_proto_type(tm) {
            return format!("const {} & ", t);
        }
        let null_prefix = self.nullability(tm)
            .map(|n| format!("{} ", n))
            .unwrap_or_default();
        format!("{}{}", null_prefix, self.to_objc_param_type(tm))
    }

    pub fn fq_param_type(&self, tm: &MExpr) -> String {
        self.param_type_mexpr(tm)
    }

    // --- return_type ---

    pub fn return_type(&self, ret: Option<&TypeRef>) -> String {
        match ret {
            None => "void".to_string(),
            Some(ty) => {
                let tm = ty.resolved.as_ref().expect("TypeRef not resolved");
                if let Some(t) = self.cpp_proto_type(tm) {
                    return t;
                }
                let null_prefix = self.nullability(tm)
                    .map(|n| format!("{} ", n))
                    .unwrap_or_default();
                format!("{}{}", null_prefix, self.to_objc_param_type(tm))
            }
        }
    }

    pub fn fq_return_type(&self, ret: Option<&TypeRef>) -> String {
        self.return_type(ret)
    }

    // --- field_type ---

    pub fn field_type(&self, ty: &TypeRef) -> String {
        let tm = ty.resolved.as_ref().expect("TypeRef not resolved");
        self.field_type_mexpr(tm)
    }

    pub fn field_type_mexpr(&self, tm: &MExpr) -> String {
        if let Some(t) = self.cpp_proto_type(tm) {
            return t;
        }
        self.to_objc_param_type(tm)
    }

    pub fn fq_field_type(&self, tm: &MExpr) -> String {
        self.field_type_mexpr(tm)
    }

    // --- references ---

    pub fn references(&self, m: &Meta) -> Vec<SymbolReference> {
        match m {
            Meta::MPrimitive(_) | Meta::MString | Meta::MDate | Meta::MBinary
            | Meta::MOptional | Meta::MList | Meta::MSet | Meta::MMap | Meta::MArray
            | Meta::MVoid => {
                vec![SymbolReference::ImportRef("<Foundation/Foundation.h>".into())]
            }
            Meta::MDef(d) => match &d.body {
                TypeDef::Enum(_) => {
                    vec![SymbolReference::ImportRef(self.include(&d.name))]
                }
                TypeDef::Interface(i) => {
                    let type_name = self.typename_from_name(&d.name, &d.body);
                    if !use_protocol(&i.ext, self.spec) {
                        vec![
                            SymbolReference::ImportRef("<Foundation/Foundation.h>".into()),
                            SymbolReference::DeclRef {
                                decl: format!("@class {};", type_name),
                                namespace: None,
                            },
                        ]
                    } else {
                        vec![
                            SymbolReference::ImportRef("<Foundation/Foundation.h>".into()),
                            SymbolReference::DeclRef {
                                decl: format!("@protocol {};", type_name),
                                namespace: None,
                            },
                        ]
                    }
                }
                TypeDef::Record(r) => {
                    let prefix = if r.ext.objc {
                        &self.spec.objc_extended_record_include_prefix
                    } else {
                        &self.spec.objc_include_prefix
                    };
                    vec![SymbolReference::ImportRef(q(&format!(
                        "{}{}",
                        prefix,
                        self.header_name(&d.name)
                    )))]
                }
                TypeDef::ProtobufMessage(_) => vec![],
            },
            Meta::MExtern(e) => {
                vec![SymbolReference::ImportRef(self.resolve_ext_objc_hdr(&e.objc.header))]
            }
            Meta::MProtobuf(p) => match &p.body.objc {
                Some(o) => vec![SymbolReference::ImportRef(o.header.clone())],
                None => vec![SymbolReference::ImportRef(p.body.cpp.header.clone())],
            },
            Meta::MParam(_) => vec![],
        }
    }

    pub fn resolve_ext_objc_hdr(&self, path: &str) -> String {
        path.replace('$', &self.spec.objc_base_lib_include_prefix)
    }

    pub fn impl_header_name(&self, ident: &str) -> String {
        format!("{}+Impl.{}", self.id_objc_ty(ident), self.spec.objc_header_ext)
    }

    pub fn header_name(&self, ident: &str) -> String {
        format!("{}.{}", self.id_objc_ty(ident), self.spec.objc_header_ext)
    }

    pub fn include(&self, ident: &str) -> String {
        q(&format!("{}{}", self.spec.objc_include_prefix, self.header_name(ident)))
    }

    pub fn is_pointer(&self, td: &TypeDef) -> bool {
        match td {
            TypeDef::Interface(_) => true,
            TypeDef::Record(_) => true,
            TypeDef::Enum(_) => false,
            TypeDef::ProtobufMessage(_) => true,
        }
    }

    pub fn boxed_typename(&self, name: &str, td: &TypeDef) -> String {
        match td {
            TypeDef::Enum(_) => "NSNumber".to_string(),
            _ => self.typename_from_name(name, td),
        }
    }

    // --- toObjcType ---

    pub fn to_objc_type(&self, tm: &MExpr, need_ref: bool) -> (String, bool) {
        self.to_objc_type_inner(tm, need_ref)
    }

    pub fn to_objc_type_from_typeref(&self, ty: &TypeRef, need_ref: bool) -> (String, bool) {
        let tm = ty.resolved.as_ref().expect("TypeRef not resolved");
        self.to_objc_type(tm, need_ref)
    }

    fn to_objc_type_inner(&self, tm: &MExpr, need_ref: bool) -> (String, bool) {
        let args_str = |tm: &MExpr| -> String {
            if tm.args.is_empty() {
                String::new()
            } else {
                let parts: Vec<String> = tm.args.iter().map(|a| self.to_boxed_param_type(a)).collect();
                format!("<{}>", parts.join(", "))
            }
        };

        match &tm.base {
            Meta::MOptional => {
                assert_eq!(tm.args.len(), 1);
                let arg = &tm.args[0];
                if matches!(&arg.base, Meta::MOptional) {
                    panic!("nested optional?");
                }
                self.to_objc_type_inner(arg, true)
            }
            other => {
                match other {
                    Meta::MPrimitive(p) => {
                        if need_ref {
                            (p.objc_boxed.clone(), true)
                        } else {
                            (p.objc_name.clone(), false)
                        }
                    }
                    Meta::MString => ("NSString".to_string(), true),
                    Meta::MDate => ("NSDate".to_string(), true),
                    Meta::MBinary => ("NSData".to_string(), true),
                    Meta::MList | Meta::MArray => {
                        (format!("NSArray{}", args_str(tm)), true)
                    }
                    Meta::MSet => {
                        (format!("NSSet{}", args_str(tm)), true)
                    }
                    Meta::MMap => {
                        (format!("NSDictionary{}", args_str(tm)), true)
                    }
                    Meta::MDef(d) => match d.def_type {
                        DefType::Enum => {
                            if need_ref {
                                ("NSNumber".to_string(), true)
                            } else {
                                (self.id_objc_ty(&d.name), false)
                            }
                        }
                        DefType::Record => (self.id_objc_ty(&d.name), true),
                        DefType::Interface => {
                            if let TypeDef::Interface(iface) = &d.body {
                                if !use_protocol(&iface.ext, self.spec) {
                                    (self.id_objc_ty(&d.name), true)
                                } else {
                                    (format!("id<{}>", self.id_objc_ty(&d.name)), false)
                                }
                            } else {
                                (self.id_objc_ty(&d.name), true)
                            }
                        }
                    },
                    Meta::MExtern(e) => match &e.body {
                        TypeDef::Interface(i) => {
                            if i.ext.objc || e.objc.protocol {
                                (format!("id<{}>", e.objc.typename), false)
                            } else {
                                (e.objc.typename.clone(), true)
                            }
                        }
                        _ => {
                            if e.objc.generic {
                                (format!("{}{}", e.objc.typename, args_str(tm)), e.objc.pointer)
                            } else if need_ref {
                                (e.objc.boxed.clone(), true)
                            } else {
                                (e.objc.typename.clone(), e.objc.pointer)
                            }
                        }
                    },
                    Meta::MProtobuf(p) => match &p.body.objc {
                        Some(o) => (format!("{}{}", o.prefix, p.name), true),
                        None => (format!("{}::{}", p.body.cpp.ns, p.name), true),
                    },
                    Meta::MParam(_) => panic!("Parameter should not happen at Obj-C top level"),
                    Meta::MVoid => ("NSNull".to_string(), true),
                    Meta::MOptional => panic!("optional should have been special cased"),
                }
            }
        }
    }

    pub fn to_boxed_param_type(&self, tm: &MExpr) -> String {
        if let Meta::MProtobuf(p) = &tm.base {
            if p.body.objc.is_none() {
                panic!("C++ proto types are not compatible with generics");
            }
        }
        let (name, need_ref) = self.to_objc_type(tm, true);
        if need_ref {
            format!("{} *", name)
        } else {
            name
        }
    }

    pub fn to_objc_param_type(&self, tm: &MExpr) -> String {
        let (name, need_ref) = self.to_objc_type(tm, false);
        if need_ref {
            format!("{} *", name)
        } else {
            name
        }
    }

    pub fn can_be_const_variable(&self, c: &Const) -> bool {
        let tm = c.ty.resolved.as_ref().expect("TypeRef not resolved");
        match &tm.base {
            Meta::MPrimitive(_) => true,
            Meta::MString => true,
            Meta::MOptional => {
                assert_eq!(tm.args.len(), 1);
                matches!(&tm.args[0].base, Meta::MString)
            }
            _ => false,
        }
    }
}

pub fn use_protocol(ext: &Ext, spec: &Spec) -> bool {
    ext.objc || spec.objc_gen_protocol
}
