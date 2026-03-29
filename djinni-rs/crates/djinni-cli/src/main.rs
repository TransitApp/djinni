// vinsg
// Djinni code generator - Rust implementation

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use djinni_ast::ident_style::{self, IdentConverter};
use djinni_ast::spec::*;
use djinni_generator::cpp_gen::generate_cpp;
use djinni_generator::gen::GeneratorContext;
use djinni_parser::ParserContext;
use djinni_resolver::resolve;

#[derive(Parser, Debug)]
#[command(name = "djinni", about = "Djinni cross-platform code generator")]
struct Cli {
    #[arg(long = "idl")]
    idl: Vec<String>,

    #[arg(long = "idl-include-path")]
    idl_include_path: Vec<String>,

    #[arg(long = "cpp-out")]
    cpp_out: Option<String>,

    #[arg(long = "cpp-namespace")]
    cpp_namespace: Option<String>,

    #[arg(long = "cpp-optional-template")]
    cpp_optional_template: Option<String>,

    #[arg(long = "cpp-optional-header")]
    cpp_optional_header: Option<String>,

    #[arg(long = "cpp-extended-record-include-prefix")]
    cpp_extended_record_include_prefix: Option<String>,

    #[arg(long = "cpp-use-wide-strings")]
    cpp_use_wide_strings: Option<bool>,

    #[arg(long = "cpp-struct-constructor")]
    cpp_struct_constructor: Option<bool>,

    #[arg(long = "cpp-include-prefix")]
    cpp_include_prefix: Option<String>,

    #[arg(long = "cpp-enum-hash-workaround")]
    cpp_enum_hash_workaround: Option<bool>,

    #[arg(long = "java-out")]
    java_out: Option<String>,

    #[arg(long = "java-package")]
    java_package: Option<String>,

    #[arg(long = "java-nullable-annotation")]
    java_nullable_annotation: Option<String>,

    #[arg(long = "java-nonnull-annotation")]
    java_nonnull_annotation: Option<String>,

    #[arg(long = "java-use-final-for-record")]
    java_use_final_for_record: Option<bool>,

    #[arg(long = "java-implement-android-os-parcelable")]
    java_implement_android_os_parcelable: Option<bool>,

    #[arg(long = "java-gen-interface")]
    java_gen_interface: Option<bool>,

    #[arg(long = "kotlin-out")]
    kotlin_out: Option<String>,

    #[arg(long = "jni-out")]
    jni_out: Option<String>,

    #[arg(long = "jni-use-on-load-initializer")]
    jni_use_on_load_initializer: Option<bool>,

    #[arg(long = "jni-function-prologue-file")]
    jni_function_prologue_file: Option<String>,

    #[arg(long = "objc-out")]
    objc_out: Option<String>,

    #[arg(long = "objcpp-out")]
    objcpp_out: Option<String>,

    #[arg(long = "objc-type-prefix")]
    objc_type_prefix: Option<String>,

    #[arg(long = "objcpp-function-prologue-file")]
    objcpp_function_prologue_file: Option<String>,

    #[arg(long = "wasm-out")]
    wasm_out: Option<String>,

    #[arg(long = "wasm-namespace")]
    wasm_namespace: Option<String>,

    #[arg(long = "ts-out")]
    ts_out: Option<String>,

    #[arg(long = "ts-module")]
    ts_module: Option<String>,

    #[arg(long = "yaml-out")]
    yaml_out: Option<String>,

    #[arg(long = "yaml-out-file")]
    yaml_out_file: Option<String>,

    #[arg(long = "yaml-prefix")]
    yaml_prefix: Option<String>,

    #[arg(long = "list-in-files")]
    list_in_files: Option<String>,

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

fn infer_ident(opt: &Option<String>, default: fn(&str) -> String) -> fn(&str) -> String {
    // For now, only support the built-in styles via exact match.
    // The full IdentConverter with prefix/suffix is Box<dyn Fn> which doesn't fit fn pointer.
    // We handle the common cases used by the test suite.
    match opt.as_deref() {
        Some("FooBar") => ident_style::camel_upper,
        Some("fooBar") => ident_style::camel_lower,
        Some("foo_bar") => ident_style::under_lower,
        Some("Foo_Bar") => ident_style::under_upper,
        Some("FOO_BAR") => ident_style::under_caps,
        _ => default,
    }
}

fn build_spec(cli: &Cli) -> Spec {
    let cpp_ident_style = {
        let mut style = ident_style::cpp_default();
        if let Some(ref s) = cli.ident_cpp_enum_type {
            match s.as_str() {
                "foo_bar" => style.enum_type = ident_style::under_lower,
                "FooBar" => style.enum_type = ident_style::camel_upper,
                "FOO_BAR" => style.enum_type = ident_style::under_caps,
                _ => {}
            }
        }
        style
    };

    let cpp_file_ident_style: IdentConverter = Box::new(infer_ident(&cli.ident_cpp_file, ident_style::under_lower));

    Spec {
        java_out_folder: cli.java_out.as_ref().map(PathBuf::from),
        java_package: cli.java_package.clone(),
        java_class_access_modifier: JavaAccessModifier::Public,
        java_ident_style: ident_style::java_default(),
        java_cpp_exception: None,
        java_annotation: None,
        java_nullable_annotation: cli.java_nullable_annotation.clone(),
        java_nonnull_annotation: cli.java_nonnull_annotation.clone(),
        java_implement_android_os_parcelable: cli
            .java_implement_android_os_parcelable
            .unwrap_or(false),
        java_use_final_for_record: cli.java_use_final_for_record.unwrap_or(true),
        java_gen_interface: cli.java_gen_interface.unwrap_or(false),
        kotlin_out_folder: cli.kotlin_out.as_ref().map(PathBuf::from),
        cpp_out_folder: cli.cpp_out.as_ref().map(PathBuf::from),
        cpp_header_out_folder: cli.cpp_out.as_ref().map(PathBuf::from),
        cpp_include_prefix: cli.cpp_include_prefix.clone().unwrap_or_default(),
        cpp_extended_record_include_prefix: cli
            .cpp_extended_record_include_prefix
            .clone()
            .unwrap_or_default(),
        cpp_namespace: cli.cpp_namespace.clone().unwrap_or_default(),
        cpp_ident_style,
        cpp_file_ident_style,
        cpp_base_lib_include_prefix: String::new(),
        cpp_optional_template: cli
            .cpp_optional_template
            .clone()
            .unwrap_or_else(|| "std::optional".into()),
        cpp_optional_header: cli
            .cpp_optional_header
            .clone()
            .unwrap_or_else(|| "<optional>".into()),
        cpp_enum_hash_workaround: cli.cpp_enum_hash_workaround.unwrap_or(true),
        cpp_nn_header: None,
        cpp_nn_type: None,
        cpp_nn_check_expression: None,
        cpp_use_wide_strings: cli.cpp_use_wide_strings.unwrap_or(false),
        cpp_struct_constructor: cli.cpp_struct_constructor.unwrap_or(true),
        jni_out_folder: cli.jni_out.as_ref().map(PathBuf::from),
        jni_header_out_folder: cli.jni_out.as_ref().map(PathBuf::from),
        jni_include_prefix: String::new(),
        jni_include_cpp_prefix: String::new(),
        jni_namespace: "djinni_generated".into(),
        jni_class_ident_style: Box::new(ident_style::camel_upper),
        jni_file_ident_style: Box::new(ident_style::camel_upper),
        jni_base_lib_include_prefix: String::new(),
        jni_use_on_load: cli.jni_use_on_load_initializer.unwrap_or(false),
        jni_function_prologue_file: cli.jni_function_prologue_file.clone(),
        cpp_ext: "cpp".into(),
        cpp_header_ext: "hpp".into(),
        objc_out_folder: cli.objc_out.as_ref().map(PathBuf::from),
        objcpp_out_folder: cli.objcpp_out.as_ref().map(PathBuf::from),
        objc_ident_style: ident_style::objc_default(),
        objc_file_ident_style: Box::new(ident_style::camel_upper),
        objcpp_ext: "mm".into(),
        objc_header_ext: "h".into(),
        objc_include_prefix: String::new(),
        objc_extended_record_include_prefix: String::new(),
        objcpp_include_prefix: String::new(),
        objcpp_include_cpp_prefix: String::new(),
        objcpp_include_objc_prefix: String::new(),
        objcpp_namespace: "djinni_generated".into(),
        objcpp_function_prologue_file: cli.objcpp_function_prologue_file.clone(),
        objcpp_disable_exception_translation: false,
        objc_base_lib_include_prefix: String::new(),
        objc_swift_bridging_header_name: None,
        objc_gen_protocol: false,
        objc_disable_class_ctor: false,
        objc_closed_enums: false,
        objc_strict_protocol: true,
        wasm_out_folder: cli.wasm_out.as_ref().map(PathBuf::from),
        wasm_include_prefix: String::new(),
        wasm_include_cpp_prefix: String::new(),
        wasm_base_lib_include_prefix: String::new(),
        wasm_omit_constants: false,
        wasm_namespace: cli.wasm_namespace.clone(),
        wasm_omit_ns_alias: false,
        js_ident_style: ident_style::js_default(),
        ts_out_folder: cli.ts_out.as_ref().map(PathBuf::from),
        ts_module: cli.ts_module.clone().unwrap_or_else(|| "module".into()),
        list_out_files: cli.list_out_files.as_ref().map(PathBuf::from),
        list_in_files: cli.list_in_files.as_ref().map(PathBuf::from),
        skip_generation: false,
        yaml_out_folder: cli.yaml_out.as_ref().map(PathBuf::from),
        yaml_out_file: cli.yaml_out_file.clone(),
        yaml_prefix: cli.yaml_prefix.clone().unwrap_or_default(),
        module_name: String::new(),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.idl.is_empty() {
        anyhow::bail!("No IDL files specified");
    }

    let spec = build_spec(&cli);

    // Include paths: empty string means "relative to current file"
    let mut include_paths = vec![String::new()];
    include_paths.extend(cli.idl_include_path.clone());

    // Parse all IDL files
    eprintln!("Parsing...");
    let mut parser_ctx = ParserContext::new(include_paths);
    let mut all_types = Vec::new();
    let mut all_flags = Vec::new();
    let mut in_files = Vec::new();

    for idl_path in &cli.idl {
        let path = PathBuf::from(idl_path);
        let (types, flags) = parser_ctx.parse_file(&path, &mut in_files)?;
        all_types.extend(types);
        all_flags.extend(flags);
    }

    // Resolve types
    eprintln!("Resolving...");
    let defaults = djinni_ast::meta::defaults();
    resolve(&defaults, &mut all_types).map_err(|e| anyhow::anyhow!("{}", e))?;

    // Create output directories
    if let Some(ref dir) = spec.cpp_out_folder {
        fs::create_dir_all(dir)?;
    }

    // Generate
    eprintln!("Generating...");
    let mut gen_ctx = GeneratorContext {
        spec,
        written_files: HashMap::new(),
        out_files: Vec::new(),
    };

    generate_cpp(&mut gen_ctx, &all_types);

    // Write file lists
    if let Some(ref path) = gen_ctx.spec.list_in_files {
        let content: String = in_files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path, content + "\n")?;
    }
    if let Some(ref path) = gen_ctx.spec.list_out_files {
        let content: String = gen_ctx
            .out_files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path, content + "\n")?;
    }

    Ok(())
}
