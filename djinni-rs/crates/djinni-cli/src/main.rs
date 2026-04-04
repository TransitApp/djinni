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

    #[arg(long = "cpp-header-out")]
    cpp_header_out: Option<String>,

    #[arg(long = "cpp-base-lib-include-prefix")]
    cpp_base_lib_include_prefix: Option<String>,

    #[arg(long = "cpp-nn-header")]
    cpp_nn_header: Option<String>,

    #[arg(long = "cpp-nn-type")]
    cpp_nn_type: Option<String>,

    #[arg(long = "cpp-nn-check-expression")]
    cpp_nn_check_expression: Option<String>,

    #[arg(long = "cpp-ext")]
    cpp_ext: Option<String>,

    #[arg(long = "hpp-ext")]
    hpp_ext: Option<String>,

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

    #[arg(long = "java-class-access-modifier")]
    java_class_access_modifier: Option<String>,

    #[arg(long = "java-cpp-exception")]
    java_cpp_exception: Option<String>,

    #[arg(long = "java-annotation")]
    java_annotation: Option<String>,

    #[arg(long = "kotlin-out")]
    kotlin_out: Option<String>,

    #[arg(long = "jni-out")]
    jni_out: Option<String>,

    #[arg(long = "jni-use-on-load-initializer")]
    jni_use_on_load_initializer: Option<bool>,

    #[arg(long = "jni-function-prologue-file")]
    jni_function_prologue_file: Option<String>,

    #[arg(long = "jni-header-out")]
    jni_header_out: Option<String>,

    #[arg(long = "jni-include-prefix")]
    jni_include_prefix: Option<String>,

    #[arg(long = "jni-include-cpp-prefix")]
    jni_include_cpp_prefix: Option<String>,

    #[arg(long = "jni-namespace")]
    jni_namespace: Option<String>,

    #[arg(long = "jni-base-lib-include-prefix")]
    jni_base_lib_include_prefix: Option<String>,

    #[arg(long = "objc-out")]
    objc_out: Option<String>,

    #[arg(long = "objcpp-out")]
    objcpp_out: Option<String>,

    #[arg(long = "objc-type-prefix")]
    objc_type_prefix: Option<String>,

    #[arg(long = "objcpp-function-prologue-file")]
    objcpp_function_prologue_file: Option<String>,

    #[arg(long = "objc-include-prefix")]
    objc_include_prefix: Option<String>,

    #[arg(long = "objc-extended-record-include-prefix")]
    objc_extended_record_include_prefix: Option<String>,

    #[arg(long = "objc-swift-bridging-header")]
    objc_swift_bridging_header: Option<String>,

    #[arg(long = "objc-h-ext")]
    objc_h_ext: Option<String>,

    #[arg(long = "objc-gen-protocol")]
    objc_gen_protocol: Option<bool>,

    #[arg(long = "objc-disable-class-ctor")]
    objc_disable_class_ctor: Option<bool>,

    #[arg(long = "objc-closed-enums")]
    objc_closed_enums: Option<bool>,

    #[arg(long = "objc-strict-protocols")]
    objc_strict_protocols: Option<bool>,

    #[arg(long = "objcpp-include-prefix")]
    objcpp_include_prefix: Option<String>,

    #[arg(long = "objcpp-include-cpp-prefix")]
    objcpp_include_cpp_prefix: Option<String>,

    #[arg(long = "objcpp-include-objc-prefix")]
    objcpp_include_objc_prefix: Option<String>,

    #[arg(long = "objcpp-namespace")]
    objcpp_namespace: Option<String>,

    #[arg(long = "objcpp-disable-exception-translation")]
    objcpp_disable_exception_translation: Option<bool>,

    #[arg(long = "wasm-out")]
    wasm_out: Option<String>,

    #[arg(long = "wasm-namespace")]
    wasm_namespace: Option<String>,

    #[arg(long = "wasm-include-prefix")]
    wasm_include_prefix: Option<String>,

    #[arg(long = "wasm-include-cpp-prefix")]
    wasm_include_cpp_prefix: Option<String>,

    #[arg(long = "wasm-base-lib-include-prefix")]
    wasm_base_lib_include_prefix: Option<String>,

    #[arg(long = "wasm-omit-constants")]
    wasm_omit_constants: Option<bool>,

    #[arg(long = "wasm-omit-namespace-alias")]
    wasm_omit_namespace_alias: Option<bool>,

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

    #[arg(long = "ident-java-enum")]
    ident_java_enum: Option<String>,

    #[arg(long = "ident-cpp-type")]
    ident_cpp_type: Option<String>,

    #[arg(long = "ident-cpp-method")]
    ident_cpp_method: Option<String>,

    #[arg(long = "ident-cpp-field")]
    ident_cpp_field: Option<String>,

    #[arg(long = "ident-cpp-local")]
    ident_cpp_local: Option<String>,

    #[arg(long = "ident-cpp-enum")]
    ident_cpp_enum: Option<String>,

    #[arg(long = "ident-cpp-type-param")]
    ident_cpp_type_param: Option<String>,

    #[arg(long = "ident-objc-field")]
    ident_objc_field: Option<String>,

    #[arg(long = "ident-objc-method")]
    ident_objc_method: Option<String>,

    #[arg(long = "ident-objc-type-param")]
    ident_objc_type_param: Option<String>,

    #[arg(long = "ident-objc-local")]
    ident_objc_local: Option<String>,

    #[arg(long = "ident-objc-file")]
    ident_objc_file: Option<String>,

    #[arg(long = "skip-generation")]
    skip_generation: Option<bool>,
}

/// Infer an ident converter that may include prefix/suffix (returns IdentConverter).
/// Falls back to a plain fn pointer wrapped in Box.
fn infer_ident_converter(opt: &Option<String>, default: fn(&str) -> String) -> IdentConverter {
    match opt {
        Some(s) => ident_style::infer(s).unwrap_or_else(|| Box::new(default)),
        None => Box::new(default),
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
                "foo_bar!" => style.enum_type = ident_style::under_lower_strict,
                "FooBar!" => style.enum_type = ident_style::camel_upper_strict,
                _ => {}
            }
        }
        style
    };

    let cpp_file_ident_style: IdentConverter = infer_ident_converter(&cli.ident_cpp_file, ident_style::under_lower);

    Spec {
        java_out_folder: cli.java_out.as_ref().map(PathBuf::from),
        java_package: cli.java_package.clone(),
        java_class_access_modifier: match cli.java_class_access_modifier.as_deref() {
            Some("package") => JavaAccessModifier::Package,
            _ => JavaAccessModifier::Public,
        },
        java_ident_style: {
            let mut style = ident_style::java_default();
            if let Some(ref s) = cli.ident_java_field {
                if let Some(conv) = ident_style::infer(s) {
                    style.field = conv;
                }
            }
            if let Some(ref s) = cli.ident_java_type {
                style.ty = infer_ident_converter(&Some(s.clone()), ident_style::camel_upper);
            }
            style
        },
        java_cpp_exception: cli.java_cpp_exception.clone(),
        java_annotation: cli.java_annotation.clone(),
        java_nullable_annotation: cli.java_nullable_annotation.clone(),
        java_nonnull_annotation: cli.java_nonnull_annotation.clone(),
        java_implement_android_os_parcelable: cli
            .java_implement_android_os_parcelable
            .unwrap_or(false),
        java_use_final_for_record: cli.java_use_final_for_record.unwrap_or(true),
        java_gen_interface: cli.java_gen_interface.unwrap_or(false),
        kotlin_out_folder: cli.kotlin_out.as_ref().map(PathBuf::from),
        cpp_out_folder: cli.cpp_out.as_ref().map(PathBuf::from),
        cpp_header_out_folder: cli.cpp_header_out.as_ref().or(cli.cpp_out.as_ref()).map(PathBuf::from),
        cpp_include_prefix: cli.cpp_include_prefix.clone().unwrap_or_default(),
        cpp_extended_record_include_prefix: cli
            .cpp_extended_record_include_prefix
            .clone()
            .unwrap_or_default(),
        cpp_namespace: cli.cpp_namespace.clone().unwrap_or_default(),
        cpp_ident_style,
        cpp_file_ident_style,
        cpp_base_lib_include_prefix: cli.cpp_base_lib_include_prefix.clone().unwrap_or_default(),
        cpp_optional_template: cli
            .cpp_optional_template
            .clone()
            .unwrap_or_else(|| "std::optional".into()),
        cpp_optional_header: cli
            .cpp_optional_header
            .clone()
            .unwrap_or_else(|| "<optional>".into()),
        cpp_enum_hash_workaround: cli.cpp_enum_hash_workaround.unwrap_or(true),
        cpp_nn_header: cli.cpp_nn_header.clone(),
        cpp_nn_type: cli.cpp_nn_type.clone(),
        cpp_nn_check_expression: cli.cpp_nn_check_expression.clone(),
        cpp_use_wide_strings: cli.cpp_use_wide_strings.unwrap_or(false),
        cpp_struct_constructor: cli.cpp_struct_constructor.unwrap_or(true),
        jni_out_folder: cli.jni_out.as_ref().map(PathBuf::from),
        jni_header_out_folder: cli.jni_header_out.as_ref().or(cli.jni_out.as_ref()).map(PathBuf::from),
        jni_include_prefix: cli.jni_include_prefix.clone().unwrap_or_default(),
        jni_include_cpp_prefix: cli.jni_include_cpp_prefix.clone().unwrap_or_default(),
        jni_namespace: cli.jni_namespace.clone().unwrap_or_else(|| "djinni_generated".into()),
        jni_class_ident_style: infer_ident_converter(&cli.ident_jni_class, ident_style::camel_upper),
        jni_file_ident_style: infer_ident_converter(&cli.ident_jni_file, ident_style::camel_upper),
        jni_base_lib_include_prefix: cli.jni_base_lib_include_prefix.clone().unwrap_or_default(),
        jni_use_on_load: cli.jni_use_on_load_initializer.unwrap_or(false),
        jni_function_prologue_file: cli.jni_function_prologue_file.clone(),
        cpp_ext: cli.cpp_ext.clone().unwrap_or_else(|| "cpp".into()),
        cpp_header_ext: cli.hpp_ext.clone().unwrap_or_else(|| "hpp".into()),
        objc_out_folder: cli.objc_out.as_ref().map(PathBuf::from),
        objcpp_out_folder: cli.objcpp_out.as_ref().map(PathBuf::from),
        objc_ident_style: {
            let mut style = ident_style::objc_default();
            if let Some(ref s) = cli.ident_objc_type {
                style.ty = infer_ident_converter(&Some(s.clone()), ident_style::camel_upper);
            }
            if let Some(ref s) = cli.ident_objc_enum {
                style.enum_ = infer_ident_converter(&Some(s.clone()), ident_style::camel_upper);
            }
            if let Some(ref s) = cli.ident_objc_const {
                style.const_ = infer_ident_converter(&Some(s.clone()), ident_style::camel_upper);
            }
            style
        },
        objc_file_ident_style: infer_ident_converter(&cli.ident_objc_file, ident_style::camel_upper),
        objcpp_ext: "mm".into(),
        objc_header_ext: cli.objc_h_ext.clone().unwrap_or_else(|| "h".into()),
        objc_include_prefix: cli.objc_include_prefix.clone().unwrap_or_default(),
        objc_extended_record_include_prefix: cli.objc_extended_record_include_prefix.clone().unwrap_or_default(),
        objcpp_include_prefix: cli.objcpp_include_prefix.clone().unwrap_or_default(),
        objcpp_include_cpp_prefix: cli.objcpp_include_cpp_prefix.clone().unwrap_or_default(),
        objcpp_include_objc_prefix: cli.objcpp_include_objc_prefix.clone().unwrap_or_default(),
        objcpp_namespace: cli.objcpp_namespace.clone().unwrap_or_else(|| "djinni_generated".into()),
        objcpp_function_prologue_file: cli.objcpp_function_prologue_file.clone(),
        objcpp_disable_exception_translation: cli.objcpp_disable_exception_translation.unwrap_or(false),
        objc_base_lib_include_prefix: String::new(),
        objc_swift_bridging_header_name: cli.objc_swift_bridging_header.clone(),
        objc_gen_protocol: cli.objc_gen_protocol.unwrap_or(false),
        objc_disable_class_ctor: cli.objc_disable_class_ctor.unwrap_or(false),
        objc_closed_enums: cli.objc_closed_enums.unwrap_or(false),
        objc_strict_protocol: cli.objc_strict_protocols.unwrap_or(true),
        objc_type_prefix: cli.objc_type_prefix.clone().unwrap_or_default(),
        wasm_out_folder: cli.wasm_out.as_ref().map(PathBuf::from),
        wasm_include_prefix: cli.wasm_include_prefix.clone().unwrap_or_default(),
        wasm_include_cpp_prefix: cli.wasm_include_cpp_prefix.clone().unwrap_or_default(),
        wasm_base_lib_include_prefix: cli.wasm_base_lib_include_prefix.clone().unwrap_or_default(),
        wasm_omit_constants: cli.wasm_omit_constants.unwrap_or(false),
        wasm_namespace: cli.wasm_namespace.clone(),
        wasm_omit_ns_alias: cli.wasm_omit_namespace_alias.unwrap_or(false),
        js_ident_style: ident_style::js_default(),
        ts_out_folder: cli.ts_out.as_ref().map(PathBuf::from),
        ts_module: cli.ts_module.clone().unwrap_or_else(|| "module".into()),
        list_out_files: cli.list_out_files.as_ref().map(PathBuf::from),
        list_in_files: cli.list_in_files.as_ref().map(PathBuf::from),
        skip_generation: cli.skip_generation.unwrap_or(false),
        yaml_out_folder: cli.yaml_out.as_ref().map(PathBuf::from),
        yaml_out_file: cli.yaml_out_file.clone(),
        yaml_prefix: cli.yaml_prefix.clone().unwrap_or_default(),
        module_name: cli.idl.first().map(|p| {
            PathBuf::from(p)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
        }).unwrap_or_default(),
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
    let mut in_files = Vec::new();

    for idl_path in &cli.idl {
        let path = PathBuf::from(idl_path);
        let (types, _flags) = parser_ctx.parse_file(&path, &mut in_files)?;
        all_types.extend(types);
    }

    // Resolve types
    eprintln!("Resolving...");
    let defaults = djinni_ast::meta::defaults();
    resolve(&defaults, &mut all_types).map_err(|e| anyhow::anyhow!("{}", e))?;

    // Create output directories
    if let Some(ref dir) = spec.cpp_out_folder {
        fs::create_dir_all(dir)?;
    }
    if let Some(ref dir) = spec.objc_out_folder {
        fs::create_dir_all(dir)?;
    }
    if let Some(ref dir) = spec.objcpp_out_folder {
        fs::create_dir_all(dir)?;
    }
    if let Some(ref dir) = spec.java_out_folder {
        fs::create_dir_all(dir)?;
    }
    if let Some(ref dir) = spec.kotlin_out_folder {
        fs::create_dir_all(dir)?;
    }
    if let Some(ref dir) = spec.jni_out_folder {
        fs::create_dir_all(dir)?;
    }
    if let Some(ref dir) = spec.yaml_out_folder {
        fs::create_dir_all(dir)?;
    }
    if let Some(ref dir) = spec.ts_out_folder {
        fs::create_dir_all(dir)?;
    }
    if let Some(ref dir) = spec.wasm_out_folder {
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
    djinni_generator::objc_gen::generate_objc(&mut gen_ctx, &all_types);
    djinni_generator::objcpp_gen::generate_objcpp(&mut gen_ctx, &all_types);
    djinni_generator::kotlin_gen::generate_kotlin(&mut gen_ctx, &all_types);
    djinni_generator::jni_gen::generate_jni(&mut gen_ctx, &all_types);
    djinni_generator::java_gen::generate_java(&mut gen_ctx, &all_types);
    djinni_generator::yaml_gen::generate_yaml(&mut gen_ctx, &all_types);
    djinni_generator::ts_gen::generate_ts(&mut gen_ctx, &all_types);
    djinni_generator::wasm_gen::generate_wasm(&mut gen_ctx, &all_types);

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
