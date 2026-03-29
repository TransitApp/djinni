// vinsg
// Meta type system for resolved types, translated from meta.scala

use std::collections::HashMap;

use crate::ast::{ProtobufMessage, TypeDef};

#[derive(Debug, Clone)]
pub struct MExpr {
    pub base: Meta,
    pub args: Vec<MExpr>,
}

#[derive(Debug, Clone)]
pub enum Meta {
    MParam(MParam),
    MDef(MDef),
    MExtern(Box<MExtern>),
    MProtobuf(MProtobuf),
    MPrimitive(MPrimitive),
    MString,
    MDate,
    MBinary,
    MOptional,
    MList,
    MSet,
    MMap,
    MArray,
    MVoid,
}

impl Meta {
    pub fn num_params(&self) -> usize {
        match self {
            Meta::MParam(_) => 0,
            Meta::MDef(d) => d.num_params,
            Meta::MExtern(e) => e.num_params,
            Meta::MProtobuf(_) => 0,
            Meta::MPrimitive(_) => 0,
            Meta::MString | Meta::MDate | Meta::MBinary | Meta::MVoid => 0,
            Meta::MOptional | Meta::MList | Meta::MSet | Meta::MArray => 1,
            Meta::MMap => 2,
        }
    }

    pub fn idl_name(&self) -> &str {
        match self {
            Meta::MPrimitive(p) => &p.idl_name,
            Meta::MString => "string",
            Meta::MDate => "date",
            Meta::MBinary => "binary",
            Meta::MOptional => "optional",
            Meta::MList => "list",
            Meta::MSet => "set",
            Meta::MMap => "map",
            Meta::MArray => "array",
            Meta::MVoid => "void",
            _ => "",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MParam {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefType {
    Enum,
    Interface,
    Record,
}

#[derive(Debug, Clone)]
pub struct MDef {
    pub name: String,
    pub num_params: usize,
    pub def_type: DefType,
    pub body: TypeDef,
}

#[derive(Debug, Clone)]
pub struct MExtern {
    pub name: String,
    pub num_params: usize,
    pub def_type: DefType,
    pub body: TypeDef,
    pub cpp: ExternCpp,
    pub objc: ExternObjc,
    pub objcpp: ExternObjcpp,
    pub java: ExternJava,
    pub jni: ExternJni,
    pub wasm: ExternWasm,
    pub ts: ExternTs,
}

#[derive(Debug, Clone)]
pub struct ExternCpp {
    pub typename: String,
    pub header: String,
    pub by_value: bool,
    pub move_only: bool,
}

#[derive(Debug, Clone)]
pub struct ExternObjc {
    pub typename: String,
    pub header: String,
    pub boxed: String,
    pub pointer: bool,
    pub generic: bool,
    pub hash: String,
    pub equal: String,
    pub protocol: bool,
}

#[derive(Debug, Clone)]
pub struct ExternObjcpp {
    pub translator: String,
    pub header: String,
}

#[derive(Debug, Clone)]
pub struct ExternJava {
    pub typename: String,
    pub boxed: String,
    pub reference: bool,
    pub generic: bool,
    pub hash: String,
    pub write_to_parcel: String,
    pub read_from_parcel: String,
}

#[derive(Debug, Clone)]
pub struct ExternJni {
    pub translator: String,
    pub header: String,
    pub typename: String,
    pub type_signature: String,
}

#[derive(Debug, Clone)]
pub struct ExternWasm {
    pub typename: String,
    pub translator: String,
    pub header: String,
}

#[derive(Debug, Clone)]
pub struct ExternTs {
    pub typename: String,
    pub module: String,
    pub generic: bool,
}

#[derive(Debug, Clone)]
pub struct MProtobuf {
    pub name: String,
    pub num_params: usize,
    pub body: ProtobufMessage,
}

#[derive(Debug, Clone)]
pub struct MPrimitive {
    pub idl_name: String,
    pub j_name: String,
    pub jni_name: String,
    pub c_name: String,
    pub j_boxed: String,
    pub j_sig: String,
    pub objc_name: String,
    pub objc_boxed: String,
    pub k_name: String,
}

pub fn is_interface(ty: &MExpr) -> bool {
    match &ty.base {
        Meta::MDef(d) => d.def_type == DefType::Interface,
        Meta::MExtern(e) => e.def_type == DefType::Interface,
        _ => false,
    }
}

pub fn is_optional(ty: &MExpr) -> bool {
    matches!(&ty.base, Meta::MOptional) && ty.args.len() == 1
}

pub fn is_optional_interface(ty: &MExpr) -> bool {
    is_optional(ty) && is_interface(&ty.args[0])
}

pub fn defaults() -> HashMap<String, Meta> {
    let mut m = HashMap::new();
    m.insert("i8".into(), Meta::MPrimitive(MPrimitive {
        idl_name: "i8".into(), j_name: "byte".into(), jni_name: "jbyte".into(),
        c_name: "int8_t".into(), j_boxed: "Byte".into(), j_sig: "B".into(),
        objc_name: "int8_t".into(), objc_boxed: "NSNumber".into(), k_name: "Byte".into(),
    }));
    m.insert("i16".into(), Meta::MPrimitive(MPrimitive {
        idl_name: "i16".into(), j_name: "short".into(), jni_name: "jshort".into(),
        c_name: "int16_t".into(), j_boxed: "Short".into(), j_sig: "S".into(),
        objc_name: "int16_t".into(), objc_boxed: "NSNumber".into(), k_name: "Short".into(),
    }));
    m.insert("i32".into(), Meta::MPrimitive(MPrimitive {
        idl_name: "i32".into(), j_name: "int".into(), jni_name: "jint".into(),
        c_name: "int32_t".into(), j_boxed: "Integer".into(), j_sig: "I".into(),
        objc_name: "int32_t".into(), objc_boxed: "NSNumber".into(), k_name: "Int".into(),
    }));
    m.insert("i64".into(), Meta::MPrimitive(MPrimitive {
        idl_name: "i64".into(), j_name: "long".into(), jni_name: "jlong".into(),
        c_name: "int64_t".into(), j_boxed: "Long".into(), j_sig: "J".into(),
        objc_name: "int64_t".into(), objc_boxed: "NSNumber".into(), k_name: "Long".into(),
    }));
    m.insert("f32".into(), Meta::MPrimitive(MPrimitive {
        idl_name: "f32".into(), j_name: "float".into(), jni_name: "jfloat".into(),
        c_name: "float".into(), j_boxed: "Float".into(), j_sig: "F".into(),
        objc_name: "float".into(), objc_boxed: "NSNumber".into(), k_name: "Float".into(),
    }));
    m.insert("f64".into(), Meta::MPrimitive(MPrimitive {
        idl_name: "f64".into(), j_name: "double".into(), jni_name: "jdouble".into(),
        c_name: "double".into(), j_boxed: "Double".into(), j_sig: "D".into(),
        objc_name: "double".into(), objc_boxed: "NSNumber".into(), k_name: "Double".into(),
    }));
    m.insert("bool".into(), Meta::MPrimitive(MPrimitive {
        idl_name: "bool".into(), j_name: "boolean".into(), jni_name: "jboolean".into(),
        c_name: "bool".into(), j_boxed: "Boolean".into(), j_sig: "Z".into(),
        objc_name: "BOOL".into(), objc_boxed: "NSNumber".into(), k_name: "Boolean".into(),
    }));
    m.insert("string".into(), Meta::MString);
    m.insert("binary".into(), Meta::MBinary);
    m.insert("optional".into(), Meta::MOptional);
    m.insert("date".into(), Meta::MDate);
    m.insert("list".into(), Meta::MList);
    m.insert("set".into(), Meta::MSet);
    m.insert("map".into(), Meta::MMap);
    m.insert("array".into(), Meta::MArray);
    m.insert("void".into(), Meta::MVoid);
    m
}
