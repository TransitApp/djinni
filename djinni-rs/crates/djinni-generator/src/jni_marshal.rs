// vinsg
// JNIMarshal: JNI type marshalling, translated from JNIMarshal.scala

use djinni_ast::ast::*;
use djinni_ast::meta::*;
use djinni_ast::spec::{q, Spec};

use crate::cpp_marshal::SymbolReference;
use crate::gen::with_ns;
use crate::java_marshal::is_enum_flags_typedef;

pub struct JniMarshal<'a> {
    pub spec: &'a Spec,
}

impl<'a> JniMarshal<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        JniMarshal { spec }
    }

    /// JNI typename: mangled Java type signature for field/method signatures
    pub fn typename_from_mexpr(&self, tm: &MExpr) -> String {
        self.java_type_signature(tm)
    }

    pub fn typename_from_name(&self, name: &str, ty: &TypeDef) -> String {
        match ty {
            TypeDef::Enum(e) if e.flags => "Ljava/util/EnumSet;".into(),
            _ => format!("L{};", self.undecorated_typename(name, ty)),
        }
    }

    pub fn fq_typename_from_name(&self, name: &str, ty: &TypeDef) -> String {
        self.typename_from_name(name, ty)
    }

    /// JNI param type (C++ JNI type like jint, jstring, jobject...)
    pub fn param_type_from_mexpr(&self, tm: &MExpr) -> String {
        self.to_jni_type(tm, false)
    }

    pub fn param_type_from_typeref(&self, ty: &TypeRef) -> String {
        match &ty.resolved {
            Some(tm) => self.to_jni_type(tm, false),
            None => "jobject".into(),
        }
    }

    /// JNI return type
    pub fn return_type(&self, ret: &Option<TypeRef>) -> String {
        match ret {
            None => "void".into(),
            Some(ty) => self.param_type_from_typeref(ty),
        }
    }

    pub fn fq_return_type(&self, ret: &Option<TypeRef>) -> String {
        self.return_type(ret)
    }

    /// Field type = param type for JNI
    pub fn field_type_from_mexpr(&self, tm: &MExpr) -> String {
        self.param_type_from_mexpr(tm)
    }

    /// toCpp conversion expression
    pub fn to_cpp(&self, tm: &MExpr, expr: &str) -> String {
        format!("{}::toCpp(jniEnv, {})", self.helper_class_from_mexpr(tm), expr)
    }

    pub fn to_cpp_typeref(&self, ty: &TypeRef, expr: &str) -> String {
        match &ty.resolved {
            Some(tm) => self.to_cpp(tm, expr),
            None => expr.to_string(),
        }
    }

    /// fromCpp conversion expression
    pub fn from_cpp(&self, tm: &MExpr, expr: &str) -> String {
        format!("{}::fromCpp(jniEnv, {})", self.helper_class_from_mexpr(tm), expr)
    }

    pub fn from_cpp_typeref(&self, ty: &TypeRef, expr: &str) -> String {
        match &ty.resolved {
            Some(tm) => self.from_cpp(tm, expr),
            None => expr.to_string(),
        }
    }

    /// Helper class name from an ident string
    pub fn helper_class(&self, name: &str) -> String {
        (self.spec.jni_class_ident_style)(name)
    }

    fn helper_class_from_mexpr(&self, tm: &MExpr) -> String {
        format!("{}{}", self.helper_name(tm), self.helper_templates(tm))
    }

    /// Include directive for a JNI helper
    pub fn include(&self, ident: &str) -> String {
        q(&format!(
            "{}{}{}",
            self.spec.jni_include_prefix,
            (self.spec.jni_file_ident_style)(ident),
            format!(".{}", self.spec.cpp_header_ext)
        ))
    }

    /// References for includes
    pub fn references(&self, m: &Meta, _exclude: &str) -> Vec<SymbolReference> {
        match m {
            Meta::MPrimitive(_)
            | Meta::MString
            | Meta::MDate
            | Meta::MBinary
            | Meta::MOptional
            | Meta::MList
            | Meta::MSet
            | Meta::MMap
            | Meta::MArray
            | Meta::MVoid => {
                vec![SymbolReference::ImportRef(q(&format!(
                    "{}Marshal.hpp",
                    self.spec.jni_base_lib_include_prefix
                )))]
            }
            Meta::MProtobuf(p) => {
                let mut refs = vec![SymbolReference::ImportRef(q(&format!(
                    "{}Marshal.hpp",
                    self.spec.jni_base_lib_include_prefix
                )))];
                if let Some(ref jni_header) = p.body.java.jni_header {
                    refs.push(SymbolReference::ImportRef(jni_header.clone()));
                }
                refs
            }
            Meta::MDef(d) => {
                vec![SymbolReference::ImportRef(self.include(&d.name))]
            }
            Meta::MExtern(e) => {
                vec![SymbolReference::ImportRef(self.resolve_ext_jni_hdr(&e.jni.header))]
            }
            _ => vec![],
        }
    }

    pub fn resolve_ext_jni_hdr(&self, path: &str) -> String {
        path.replace('$', &self.spec.jni_base_lib_include_prefix)
    }

    /// The undecorated Java typename (e.g. "com/dropbox/djinni/test/Color")
    pub fn undecorated_typename(&self, name: &str, _ty: &TypeDef) -> String {
        let java_class_name = (self.spec.java_ident_style.ty)(name);
        match &self.spec.java_package {
            Some(pkg) => format!("{}/{}", pkg.replace('.', "/"), java_class_name),
            None => java_class_name,
        }
    }

    /// Java method signature for JNI (e.g. "(IJLjava/lang/String;)V")
    pub fn java_method_signature(&self, params: &[Field], ret: &Option<TypeRef>) -> String {
        let param_sigs: String = params
            .iter()
            .map(|f| self.typename_from_typeref(&f.ty))
            .collect::<Vec<_>>()
            .join("");
        let ret_sig = match ret {
            None => "V".to_string(),
            Some(ty) => self.typename_from_typeref(ty),
        };
        format!("({}){}", param_sigs, ret_sig)
    }

    pub fn typename_from_typeref(&self, ty: &TypeRef) -> String {
        match &ty.resolved {
            Some(tm) => self.java_type_signature(tm),
            None => String::new(),
        }
    }

    pub fn fq_typename_from_typeref(&self, ty: &TypeRef) -> String {
        self.typename_from_typeref(ty)
    }

    /// Helper class name for Java class reflection (no L...;)
    pub fn java_class_name_as_cpp_type(&self, fq_java_class: &str) -> String {
        let class_name_chars: Vec<String> = fq_java_class.chars().map(|c| format!("'{}'", c)).collect();
        format!("::djinni::JavaClassName<{}>", class_name_chars.join(","))
    }

    /// Check if a TypeRef represents a Java heap object (not a primitive)
    pub fn is_java_heap_object(&self, m: &Meta) -> bool {
        !matches!(m, Meta::MPrimitive(_))
    }

    pub fn is_java_heap_object_typeref(&self, ty: &TypeRef) -> bool {
        match &ty.resolved {
            Some(tm) => self.is_java_heap_object(&tm.base),
            None => true,
        }
    }

    // --- private helpers ---

    fn to_jni_type(&self, tm: &MExpr, need_ref: bool) -> String {
        match &tm.base {
            Meta::MPrimitive(p) => {
                if need_ref {
                    "jobject".into()
                } else {
                    p.jni_name.clone()
                }
            }
            Meta::MString => "jstring".into(),
            Meta::MOptional => self.to_jni_type(&tm.args[0], true),
            Meta::MBinary => "jbyteArray".into(),
            Meta::MParam(p) => format!("{}::JniType", self.helper_class(&p.name)),
            Meta::MExtern(_) => {
                let hc = self.helper_class_from_mexpr(tm);
                if need_ref {
                    format!("{}::Boxed::JniType", hc)
                } else {
                    format!("{}::JniType", hc)
                }
            }
            _ => "jobject".into(),
        }
    }

    fn java_type_signature(&self, tm: &MExpr) -> String {
        match &tm.base {
            Meta::MPrimitive(p) => p.j_sig.clone(),
            Meta::MString => "Ljava/lang/String;".into(),
            Meta::MDate => "Ljava/util/Date;".into(),
            Meta::MBinary => "[B".into(),
            Meta::MOptional => {
                let head = &tm.args[0];
                match &head.base {
                    Meta::MPrimitive(p) => format!("Ljava/lang/{};", p.j_boxed),
                    Meta::MOptional => panic!("nested optional?"),
                    _ => self.java_type_signature(head),
                }
            }
            Meta::MList => "Ljava/util/ArrayList;".into(),
            Meta::MSet => "Ljava/util/HashSet;".into(),
            Meta::MMap => "Ljava/util/HashMap;".into(),
            Meta::MArray => format!("[{}", self.java_type_signature(&tm.args[0])),
            Meta::MVoid => "Ljava/lang/Void;".into(),
            Meta::MExtern(e) => e.jni.type_signature.clone(),
            Meta::MParam(_) => "Ljava/lang/Object;".into(),
            Meta::MDef(d) => {
                if is_enum_flags_typedef(&d.body) {
                    "Ljava/util/EnumSet;".into()
                } else {
                    format!("L{};", self.undecorated_typename(&d.name, &d.body))
                }
            }
            Meta::MProtobuf(p) => {
                let prefix = p.body.java.pkg.replace('.', "/");
                format!("L{}${};", prefix, p.name)
            }
        }
    }

    /// Extract class name for FindClass (strip L...;)
    fn java_class_name_for_find_class(&self, tm: &MExpr) -> String {
        let sig = self.java_type_signature(tm);
        if sig.starts_with('L') && sig.ends_with(';') {
            sig[1..sig.len() - 1].to_string()
        } else {
            sig
        }
    }

    pub fn helper_name(&self, tm: &MExpr) -> String {
        match &tm.base {
            Meta::MDef(d) => {
                with_ns(Some(&self.spec.jni_namespace), &self.helper_class(&d.name))
            }
            Meta::MExtern(e) => e.jni.translator.clone(),
            Meta::MPrimitive(p) => {
                let name = match p.idl_name.as_str() {
                    "i8" => "I8",
                    "i16" => "I16",
                    "i32" => "I32",
                    "i64" => "I64",
                    "f32" => "F32",
                    "f64" => "F64",
                    "bool" => "Bool",
                    _ => panic!("Unknown primitive: {}", p.idl_name),
                };
                with_ns(Some("djinni"), name)
            }
            other => {
                let name = match other {
                    Meta::MOptional => "Optional",
                    Meta::MBinary => "Binary",
                    Meta::MString => {
                        if self.spec.cpp_use_wide_strings {
                            "WString"
                        } else {
                            "String"
                        }
                    }
                    Meta::MDate => "Date",
                    Meta::MList => "List",
                    Meta::MSet => "Set",
                    Meta::MMap => "Map",
                    Meta::MProtobuf(_) => "Protobuf",
                    Meta::MArray => "Array",
                    Meta::MVoid => "Void",
                    _ => panic!("Unexpected meta type for helper_name"),
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
                let inner: Vec<String> = tm.args.iter().map(|a| self.helper_class_from_mexpr(a)).collect();
                format!("<{}>", inner.join(", "))
            }
        };

        match &tm.base {
            Meta::MOptional => {
                assert_eq!(tm.args.len(), 1);
                let arg_helper = self.helper_class_from_mexpr(&tm.args[0]);
                format!("<{}, {}>", self.spec.cpp_optional_template, arg_helper)
            }
            Meta::MList | Meta::MSet => {
                assert_eq!(tm.args.len(), 1);
                f()
            }
            Meta::MMap => {
                assert_eq!(tm.args.len(), 2);
                f()
            }
            Meta::MProtobuf(p) => {
                assert!(tm.args.is_empty());
                let fq_java_proto_class =
                    format!("{}${}", p.body.java.pkg.replace('.', "/"), p.name);
                let serializer = match &p.body.java.jni_class {
                    Some(s) => format!(", {}", s),
                    None => String::new(),
                };
                format!(
                    "<{}, {}{}>",
                    with_ns(Some(&p.body.cpp.ns), &p.name),
                    self.java_class_name_as_cpp_type(&fq_java_proto_class),
                    serializer
                )
            }
            Meta::MArray => {
                assert_eq!(tm.args.len(), 1);
                let arg_helper = self.helper_class_from_mexpr(&tm.args[0]);
                let class_name = self.java_class_name_for_find_class(&tm.args[0]);
                format!("<{}, {}>", arg_helper, self.java_class_name_as_cpp_type(&class_name))
            }
            _ => f(),
        }
    }
}

/// Convert a JNI field access to the correct typed call (e.g. GetBooleanField, GetObjectField)
pub fn to_jni_call<F>(ty: &TypeRef, f: F) -> String
where
    F: Fn(&str) -> String,
{
    match &ty.resolved {
        Some(tm) => to_jni_call_mexpr(tm, &f, false),
        None => f("Object"),
    }
}

fn to_jni_call_mexpr<F>(tm: &MExpr, f: &F, need_ref: bool) -> String
where
    F: Fn(&str) -> String,
{
    match &tm.base {
        Meta::MPrimitive(p) => {
            if need_ref {
                f("Object")
            } else {
                f(&camel_upper_j(&p.j_name))
            }
        }
        Meta::MString => format!("(jstring){}", f("Object")),
        Meta::MOptional => to_jni_call_mexpr(&tm.args[0], f, true),
        Meta::MBinary => format!("(jbyteArray){}", f("Object")),
        _ => f("Object"),
    }
}

fn camel_upper_j(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => {
            let upper = c.to_uppercase().to_string();
            format!("{}{}", upper, chars.as_str())
        }
    }
}
