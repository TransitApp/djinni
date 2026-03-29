// vinsg
// Djinni code generator - Rust implementation

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "djinni", about = "Djinni cross-platform code generator")]
struct Cli {
    /// Input IDL file(s)
    #[arg(long = "idl")]
    idl: Vec<String>,

    /// IDL include paths
    #[arg(long = "idl-include-path")]
    idl_include_path: Vec<String>,

    /// C++ output directory
    #[arg(long = "cpp-out")]
    cpp_out: Option<String>,

    /// C++ namespace
    #[arg(long = "cpp-namespace")]
    cpp_namespace: Option<String>,

    /// C++ optional template
    #[arg(long = "cpp-optional-template")]
    cpp_optional_template: Option<String>,

    /// C++ optional header
    #[arg(long = "cpp-optional-header")]
    cpp_optional_header: Option<String>,

    /// C++ extended record include prefix
    #[arg(long = "cpp-extended-record-include-prefix")]
    cpp_extended_record_include_prefix: Option<String>,

    /// C++ use wide strings
    #[arg(long = "cpp-use-wide-strings")]
    cpp_use_wide_strings: Option<bool>,

    /// C++ struct constructor generation
    #[arg(long = "cpp-struct-constructor")]
    cpp_struct_constructor: Option<bool>,

    /// Java output directory
    #[arg(long = "java-out")]
    java_out: Option<String>,

    /// Java package
    #[arg(long = "java-package")]
    java_package: Option<String>,

    /// Java nullable annotation
    #[arg(long = "java-nullable-annotation")]
    java_nullable_annotation: Option<String>,

    /// Java nonnull annotation
    #[arg(long = "java-nonnull-annotation")]
    java_nonnull_annotation: Option<String>,

    /// Java use final for record
    #[arg(long = "java-use-final-for-record")]
    java_use_final_for_record: Option<bool>,

    /// Java implement android.os.Parcelable
    #[arg(long = "java-implement-android-os-parcelable")]
    java_implement_android_os_parcelable: Option<bool>,

    /// Java generate interface
    #[arg(long = "java-gen-interface")]
    java_gen_interface: Option<bool>,

    /// Kotlin output directory
    #[arg(long = "kotlin-out")]
    kotlin_out: Option<String>,

    /// JNI output directory
    #[arg(long = "jni-out")]
    jni_out: Option<String>,

    /// JNI use on load initializer
    #[arg(long = "jni-use-on-load-initializer")]
    jni_use_on_load_initializer: Option<bool>,

    /// JNI function prologue file
    #[arg(long = "jni-function-prologue-file")]
    jni_function_prologue_file: Option<String>,

    /// Objective-C output directory
    #[arg(long = "objc-out")]
    objc_out: Option<String>,

    /// Objective-C++ output directory
    #[arg(long = "objcpp-out")]
    objcpp_out: Option<String>,

    /// Objective-C type prefix
    #[arg(long = "objc-type-prefix")]
    objc_type_prefix: Option<String>,

    /// Objective-C++ function prologue file
    #[arg(long = "objcpp-function-prologue-file")]
    objcpp_function_prologue_file: Option<String>,

    /// WASM output directory
    #[arg(long = "wasm-out")]
    wasm_out: Option<String>,

    /// WASM namespace
    #[arg(long = "wasm-namespace")]
    wasm_namespace: Option<String>,

    /// TypeScript output directory
    #[arg(long = "ts-out")]
    ts_out: Option<String>,

    /// TypeScript module name
    #[arg(long = "ts-module")]
    ts_module: Option<String>,

    /// YAML output directory
    #[arg(long = "yaml-out")]
    yaml_out: Option<String>,

    /// YAML output filename
    #[arg(long = "yaml-out-file")]
    yaml_out_file: Option<String>,

    /// YAML prefix
    #[arg(long = "yaml-prefix")]
    yaml_prefix: Option<String>,

    /// List input files
    #[arg(long = "list-in-files")]
    list_in_files: Option<String>,

    /// List output files
    #[arg(long = "list-out-files")]
    list_out_files: Option<String>,

    // Identifier style options
    #[arg(long = "ident-java-field")]
    ident_java_field: Option<String>,

    #[arg(long = "ident-java-type")]
    ident_java_type: Option<String>,

    #[arg(long = "ident-cpp-enum-type")]
    ident_cpp_enum_type: Option<String>,

    #[arg(long = "ident-cpp-file")]
    ident_cpp_file: Option<String>,

    #[arg(long = "ident-jni-class")]
    ident_jni_class: Option<String>,

    #[arg(long = "ident-jni-file")]
    ident_jni_file: Option<String>,

    #[arg(long = "ident-objc-type")]
    ident_objc_type: Option<String>,

    #[arg(long = "ident-objc-enum")]
    ident_objc_enum: Option<String>,

    #[arg(long = "ident-objc-const")]
    ident_objc_const: Option<String>,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    eprintln!("Parsing...");
    // TODO: implement parsing
    eprintln!("Resolving...");
    // TODO: implement resolution
    eprintln!("Generating...");
    // TODO: implement generation

    Ok(())
}
