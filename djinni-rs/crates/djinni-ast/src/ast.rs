// vinsg
// AST types for the Djinni IDL, translated from ast.scala

use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::meta::MExpr;

#[derive(Debug, Clone)]
pub struct Loc {
    pub file: PathBuf,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}.{})", self.file.display(), self.line, self.col)
    }
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub struct TypeParam {
    pub ident: Ident,
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub lines: Vec<String>,
}

// File references
#[derive(Debug, Clone)]
pub enum FileRef {
    Idl(PathBuf),
    Extern(PathBuf),
    Protobuf(PathBuf),
}

impl FileRef {
    pub fn file(&self) -> &PathBuf {
        match self {
            FileRef::Idl(f) | FileRef::Extern(f) | FileRef::Protobuf(f) => f,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdlFile {
    pub imports: Vec<FileRef>,
    pub type_decls: Vec<TypeDecl>,
    pub flags: Vec<String>,
}

// Type declarations
#[derive(Debug, Clone)]
pub enum TypeDecl {
    Intern {
        ident: Ident,
        params: Vec<TypeParam>,
        body: TypeDef,
        doc: Doc,
        origin: String,
    },
    Extern {
        ident: Ident,
        params: Vec<TypeParam>,
        body: TypeDef,
        properties: std::collections::HashMap<String, serde_yaml::Value>,
        origin: String,
    },
    Protobuf {
        ident: Ident,
        params: Vec<TypeParam>,
        body: TypeDef,
        origin: String,
    },
}

impl TypeDecl {
    pub fn ident(&self) -> &Ident {
        match self {
            TypeDecl::Intern { ident, .. }
            | TypeDecl::Extern { ident, .. }
            | TypeDecl::Protobuf { ident, .. } => ident,
        }
    }

    pub fn params(&self) -> &[TypeParam] {
        match self {
            TypeDecl::Intern { params, .. }
            | TypeDecl::Extern { params, .. }
            | TypeDecl::Protobuf { params, .. } => params,
        }
    }

    pub fn body(&self) -> &TypeDef {
        match self {
            TypeDecl::Intern { body, .. }
            | TypeDecl::Extern { body, .. }
            | TypeDecl::Protobuf { body, .. } => body,
        }
    }

    pub fn origin(&self) -> &str {
        match self {
            TypeDecl::Intern { origin, .. }
            | TypeDecl::Extern { origin, .. }
            | TypeDecl::Protobuf { origin, .. } => origin,
        }
    }

    pub fn body_mut(&mut self) -> &mut TypeDef {
        match self {
            TypeDecl::Intern { body, .. }
            | TypeDecl::Extern { body, .. }
            | TypeDecl::Protobuf { body, .. } => body,
        }
    }
}

// Language extension flags
#[derive(Debug, Clone, Default)]
pub struct Ext {
    pub java: bool,
    pub cpp: bool,
    pub objc: bool,
    pub js: bool,
}

impl Ext {
    pub fn any(&self) -> bool {
        self.java || self.cpp || self.objc || self.js
    }
}

// Type references (resolved during type resolution)
#[derive(Debug, Clone)]
pub struct TypeRef {
    pub expr: TypeExpr,
    pub resolved: Option<MExpr>,
}

#[derive(Debug, Clone)]
pub struct TypeExpr {
    pub ident: Ident,
    pub args: Vec<TypeExpr>,
}

// Type definitions
#[derive(Debug, Clone)]
pub enum TypeDef {
    Enum(Enum),
    Record(Record),
    Interface(Interface),
    ProtobufMessage(ProtobufMessage),
}

// Constants
#[derive(Debug, Clone)]
pub struct Const {
    pub ident: Ident,
    pub ty: TypeRef,
    pub value: ConstValue,
    pub doc: Doc,
}

#[derive(Debug, Clone)]
pub enum ConstValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    EnumValue { ty: String, value: String },
    Composite(Vec<(String, ConstValue)>),
}

// Enum definition
#[derive(Debug, Clone)]
pub struct Enum {
    pub options: Vec<EnumOption>,
    pub flags: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialFlag {
    NoFlags,
    AllFlags,
}

#[derive(Debug, Clone)]
pub struct EnumOption {
    pub ident: Ident,
    pub doc: Doc,
    pub special_flag: Option<SpecialFlag>,
}

// Record definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DerivingType {
    Ord,
    AndroidParcelable,
    NSCopying,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub ext: Ext,
    pub fields: Vec<Field>,
    pub consts: Vec<Const>,
    pub deriving_types: BTreeSet<DerivingType>,
    pub base_record: Option<String>,
}

// Interface definition
#[derive(Debug, Clone)]
pub struct Interface {
    pub ext: Ext,
    pub methods: Vec<Method>,
    pub consts: Vec<Const>,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub ident: Ident,
    pub params: Vec<Field>,
    pub ret: Option<TypeRef>,
    pub doc: Doc,
    pub is_static: bool,
    pub is_const: bool,
    pub lang: Ext,
}

// Field definition
#[derive(Debug, Clone)]
pub struct Field {
    pub ident: Ident,
    pub ty: TypeRef,
    pub default_value: String,
    pub doc: Doc,
}

// Super record (for inheritance)
#[derive(Debug, Clone)]
pub struct SuperRecord {
    pub ident: Ident,
    pub record: Record,
    pub fields: Vec<Field>,
}

// Protobuf message
#[derive(Debug, Clone)]
pub struct ProtobufMessage {
    pub cpp: ProtobufCpp,
    pub java: ProtobufJava,
    pub objc: Option<ProtobufObjc>,
    pub ts: Option<ProtobufTs>,
}

#[derive(Debug, Clone)]
pub struct ProtobufCpp {
    pub header: String,
    pub ns: String,
}

#[derive(Debug, Clone)]
pub struct ProtobufJava {
    pub pkg: String,
    pub jni_class: Option<String>,
    pub jni_header: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProtobufObjc {
    pub header: String,
    pub prefix: String,
}

#[derive(Debug, Clone)]
pub struct ProtobufTs {
    pub module: String,
    pub ns: String,
}
