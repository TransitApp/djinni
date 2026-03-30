// vinsg
// Configuration specification for code generation, translated from generator.scala

use std::path::PathBuf;

use crate::ident_style::{
    CppIdentStyle, IdentConverter, JavaIdentStyle, JsIdentStyle, ObjcIdentStyle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JavaAccessModifier {
    Public,
    Package,
}

impl JavaAccessModifier {
    pub fn code_generation_string(&self) -> &str {
        match self {
            JavaAccessModifier::Public => "public ",
            JavaAccessModifier::Package => "",
        }
    }
}

pub struct Spec {
    // Java
    pub java_out_folder: Option<PathBuf>,
    pub java_package: Option<String>,
    pub java_class_access_modifier: JavaAccessModifier,
    pub java_ident_style: JavaIdentStyle,
    pub java_cpp_exception: Option<String>,
    pub java_annotation: Option<String>,
    pub java_nullable_annotation: Option<String>,
    pub java_nonnull_annotation: Option<String>,
    pub java_implement_android_os_parcelable: bool,
    pub java_use_final_for_record: bool,
    pub java_gen_interface: bool,

    // Kotlin
    pub kotlin_out_folder: Option<PathBuf>,

    // C++
    pub cpp_out_folder: Option<PathBuf>,
    pub cpp_header_out_folder: Option<PathBuf>,
    pub cpp_include_prefix: String,
    pub cpp_extended_record_include_prefix: String,
    pub cpp_namespace: String,
    pub cpp_ident_style: CppIdentStyle,
    pub cpp_file_ident_style: IdentConverter,
    pub cpp_base_lib_include_prefix: String,
    pub cpp_optional_template: String,
    pub cpp_optional_header: String,
    pub cpp_enum_hash_workaround: bool,
    pub cpp_nn_header: Option<String>,
    pub cpp_nn_type: Option<String>,
    pub cpp_nn_check_expression: Option<String>,
    pub cpp_use_wide_strings: bool,
    pub cpp_struct_constructor: bool,

    // JNI
    pub jni_out_folder: Option<PathBuf>,
    pub jni_header_out_folder: Option<PathBuf>,
    pub jni_include_prefix: String,
    pub jni_include_cpp_prefix: String,
    pub jni_namespace: String,
    pub jni_class_ident_style: IdentConverter,
    pub jni_file_ident_style: IdentConverter,
    pub jni_base_lib_include_prefix: String,
    pub jni_use_on_load: bool,
    pub jni_function_prologue_file: Option<String>,

    // C++ file extensions
    pub cpp_ext: String,
    pub cpp_header_ext: String,

    // Objective-C
    pub objc_out_folder: Option<PathBuf>,
    pub objcpp_out_folder: Option<PathBuf>,
    pub objc_ident_style: ObjcIdentStyle,
    pub objc_file_ident_style: IdentConverter,
    pub objcpp_ext: String,
    pub objc_header_ext: String,
    pub objc_include_prefix: String,
    pub objc_extended_record_include_prefix: String,
    pub objcpp_include_prefix: String,
    pub objcpp_include_cpp_prefix: String,
    pub objcpp_include_objc_prefix: String,
    pub objcpp_namespace: String,
    pub objcpp_function_prologue_file: Option<String>,
    pub objcpp_disable_exception_translation: bool,
    pub objc_base_lib_include_prefix: String,
    pub objc_swift_bridging_header_name: Option<String>,
    pub objc_gen_protocol: bool,
    pub objc_disable_class_ctor: bool,
    pub objc_closed_enums: bool,
    pub objc_strict_protocol: bool,
    pub objc_type_prefix: String,

    // WASM
    pub wasm_out_folder: Option<PathBuf>,
    pub wasm_include_prefix: String,
    pub wasm_include_cpp_prefix: String,
    pub wasm_base_lib_include_prefix: String,
    pub wasm_omit_constants: bool,
    pub wasm_namespace: Option<String>,
    pub wasm_omit_ns_alias: bool,

    // JS/TS
    pub js_ident_style: JsIdentStyle,
    pub ts_out_folder: Option<PathBuf>,
    pub ts_module: String,

    // Output file lists
    pub list_out_files: Option<PathBuf>,
    pub list_in_files: Option<PathBuf>,

    // Generation control
    pub skip_generation: bool,

    // YAML
    pub yaml_out_folder: Option<PathBuf>,
    pub yaml_out_file: Option<String>,
    pub yaml_prefix: String,

    // Module
    pub module_name: String,
}

pub fn pre_comma(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        format!(", {}", s)
    }
}

pub fn q(s: &str) -> String {
    format!("\"{}\"", s)
}
