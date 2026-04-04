// vinsg
// WasmGenerator: generates WASM/Emscripten C++ bridge files (.hpp/.cpp), translated from WasmGenerator.scala

use std::collections::BTreeSet;
use std::path::Path;

use djinni_ast::ast::*;
use djinni_ast::meta::*;
use djinni_ast::spec::{q, Spec};

use crate::cpp_marshal::{CppMarshal, SymbolReference};
use crate::gen::*;
use crate::writer::IndentWriter;

pub fn generate_wasm(ctx: &mut GeneratorContext, idl: &[TypeDecl]) {
    let wasm_out = match &ctx.spec.wasm_out_folder {
        Some(f) => f.clone(),
        None => return,
    };

    let intern_decls: Vec<_> = idl
        .iter()
        .filter_map(|td| {
            if let TypeDecl::Intern { ident, params, body, doc, origin } = td {
                Some((ident.clone(), params.clone(), body.clone(), doc.clone(), origin.clone()))
            } else {
                None
            }
        })
        .collect();

    for (ident, params, body, _doc, origin) in &intern_decls {
        match body {
            TypeDef::Enum(e) => {
                assert!(params.is_empty());
                generate_enum(ctx, &wasm_out, origin, ident, e, idl);
            }
            TypeDef::Record(r) => {
                generate_record(ctx, &wasm_out, origin, ident, params, r, idl);
            }
            TypeDef::Interface(i) => {
                generate_interface(ctx, &wasm_out, origin, ident, params, i, idl);
            }
            TypeDef::ProtobufMessage(_) => {}
        }
    }
}

// --- WasmRefs ---

struct WasmRefs {
    hpp: BTreeSet<String>,
    cpp: BTreeSet<String>,
}

impl WasmRefs {
    fn new(spec: &Spec, name: &str, cpp_prefix_override: Option<&str>) -> Self {
        let mut hpp = BTreeSet::new();
        let cpp = BTreeSet::new();

        let cpp_prefix = cpp_prefix_override.unwrap_or(&spec.wasm_include_cpp_prefix);
        hpp.insert(format!(
            "#include {}",
            q(&format!(
                "{}{}.{}",
                cpp_prefix,
                (spec.cpp_file_ident_style)(name),
                spec.cpp_header_ext
            ))
        ));
        hpp.insert(format!(
            "#include {}",
            q(&format!("{}djinni_wasm.hpp", spec.wasm_base_lib_include_prefix))
        ));
        if let Some(ref nn_hdr) = spec.cpp_nn_header {
            hpp.insert(format!("#include {}", nn_hdr));
        }

        WasmRefs { hpp, cpp }
    }

    fn find_type_ref(&mut self, ty: &TypeRef, spec: &Spec) {
        if let Some(resolved) = &ty.resolved {
            self.find_mexpr(resolved, spec);
        }
    }

    fn find_mexpr(&mut self, tm: &MExpr, spec: &Spec) {
        for arg in &tm.args {
            self.find_mexpr(arg, spec);
        }
        self.find_meta(&tm.base, spec);
    }

    fn find_meta(&mut self, m: &Meta, spec: &Spec) {
        for r in wasm_references(m, "", spec) {
            if let SymbolReference::ImportRef(arg) = r {
                self.cpp.insert(format!("#include {}", arg));
            }
        }
    }
}

fn wasm_include(ident: &str, spec: &Spec) -> String {
    q(&format!(
        "{}{}.{}",
        spec.wasm_include_prefix,
        wasm_filename_style(ident, spec),
        spec.cpp_header_ext
    ))
}

fn wasm_references(m: &Meta, _exclude: &str, spec: &Spec) -> Vec<SymbolReference> {
    match m {
        Meta::MDef(d) => vec![SymbolReference::ImportRef(wasm_include(&d.name, spec))],
        Meta::MExtern(e) => vec![SymbolReference::ImportRef(resolve_ext_wasm_hdr(&e.wasm.header, spec))],
        _ => vec![],
    }
}

fn resolve_ext_wasm_hdr(path: &str, spec: &Spec) -> String {
    path.replace('$', &spec.wasm_base_lib_include_prefix)
}

fn wasm_filename_style(name: &str, spec: &Spec) -> String {
    (spec.jni_file_ident_style)(name)
}

fn helper_class_name(name: &str, spec: &Spec) -> String {
    (spec.jni_class_ident_style)(name)
}

fn helper_namespace(spec: &Spec) -> String {
    spec.jni_namespace.clone()
}

fn helper_class_mexpr(tm: &MExpr, spec: &Spec) -> String {
    format!("{}{}", helper_name(tm, spec), helper_templates(tm, spec))
}

fn helper_name(tm: &MExpr, spec: &Spec) -> String {
    match &tm.base {
        Meta::MDef(d) => with_ns(Some(&helper_namespace(spec)), &helper_class_name(&d.name, spec)),
        Meta::MExtern(e) => e.wasm.translator.clone(),
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
                    _ => panic!("Unknown primitive: {}", p.idl_name),
                },
                Meta::MOptional => "Optional",
                Meta::MBinary => "Binary",
                Meta::MString => {
                    if spec.cpp_use_wide_strings {
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
                _ => panic!("Unexpected meta type in helper_name"),
            };
            with_ns(Some("djinni"), name)
        }
    }
}

fn helper_templates(tm: &MExpr, spec: &Spec) -> String {
    let f = || -> String {
        if tm.args.is_empty() {
            String::new()
        } else {
            let parts: Vec<String> = tm.args.iter().map(|a| helper_class_mexpr(a, spec)).collect();
            format!("<{}>", parts.join(", "))
        }
    };
    match &tm.base {
        Meta::MOptional => {
            assert!(tm.args.len() == 1);
            let arg_helper = helper_class_mexpr(&tm.args[0], spec);
            format!("<{}, {}>", spec.cpp_optional_template, arg_helper)
        }
        Meta::MList | Meta::MSet => {
            assert!(tm.args.len() == 1);
            f()
        }
        Meta::MMap => {
            assert!(tm.args.len() == 2);
            f()
        }
        Meta::MProtobuf(pb) => {
            assert!(tm.args.is_empty());
            let ts = pb.body.ts.as_ref();
            match ts {
                Some(ts_info) => {
                    let tsname = if ts_info.ns.is_empty() {
                        pb.name.clone()
                    } else {
                        format!("{}.{}", ts_info.ns, pb.name)
                    };
                    format!(
                        "<{}, {}>",
                        with_ns(Some(&pb.body.cpp.ns), &pb.name),
                        js_class_name_as_cpp_type(&tsname)
                    )
                }
                None => f(),
            }
        }
        Meta::MArray => {
            assert!(tm.args.len() == 1);
            format!("<{}>", helper_class_mexpr(&tm.args[0], spec))
        }
        _ => f(),
    }
}

fn js_class_name_as_cpp_type(js_class: &str) -> String {
    let chars: Vec<String> = js_class.chars().map(|c| format!("'{}'", c)).collect();
    format!("::djinni::JsClassName<{}>", chars.join(","))
}

fn wasm_type_mexpr(tm: &MExpr, spec: &Spec) -> String {
    match &tm.base {
        Meta::MPrimitive(p) => p.c_name.clone(),
        Meta::MString => {
            if spec.cpp_use_wide_strings {
                "std::wstring".into()
            } else {
                "std::string".into()
            }
        }
        Meta::MDef(d) => match d.def_type {
            DefType::Enum => "int32_t".into(),
            _ => "em::val".into(),
        },
        Meta::MExtern(e) => e.wasm.typename.clone(),
        _ => "em::val".into(),
    }
}

fn wasm_type_ref(ty: &TypeRef, spec: &Spec) -> String {
    wasm_type_mexpr(ty.resolved.as_ref().expect("TypeRef not resolved"), spec)
}

fn stub_ret_type(m: &Method, spec: &Spec) -> String {
    match &m.ret {
        None => "void".into(),
        Some(ty) => wasm_type_ref(ty, spec),
    }
}

fn stub_param_type(ty: &TypeRef, spec: &Spec) -> String {
    let tm = ty.resolved.as_ref().expect("TypeRef not resolved");
    match &tm.base {
        Meta::MPrimitive(p) => p.c_name.clone(),
        Meta::MString => {
            if spec.cpp_use_wide_strings {
                "const std::wstring&".into()
            } else {
                "const std::string&".into()
            }
        }
        Meta::MDef(d) => match d.def_type {
            DefType::Enum => "int32_t".into(),
            _ => "const em::val&".into(),
        },
        Meta::MExtern(e) => match e.def_type {
            DefType::Enum => e.wasm.typename.clone(),
            _ => format!("const {}&", e.wasm.typename),
        },
        _ => "const em::val&".into(),
    }
}

fn stub_param_name(name: &str, spec: &Spec) -> String {
    format!("w_{}", (spec.cpp_ident_style.local)(name))
}

fn with_wasm_namespace(name: &str, spec: &Spec) -> String {
    with_wasm_namespace_sep(name, spec, "_")
}

fn with_wasm_namespace_sep(name: &str, spec: &Spec, sep: &str) -> String {
    match &spec.wasm_namespace {
        Some(p) => format!("{}{}{}", p.replace('.', sep), sep, name),
        None => name.to_string(),
    }
}

fn with_cpp_namespace(name: &str, spec: &Spec) -> String {
    with_cpp_namespace_sep(name, spec, "_")
}

fn with_cpp_namespace_sep(name: &str, spec: &Spec, sep: &str) -> String {
    format!("{}{}{}", spec.cpp_namespace.replace("::", sep), sep, name)
}

// --- Enum Generation ---

fn generate_enum(
    ctx: &mut GeneratorContext,
    folder: &Path,
    origin: &str,
    ident: &Ident,
    e: &Enum,
    _idl: &[TypeDecl],
) {
    let (spec, out_files, written_files) = ctx.split_borrow();
    let cpp_marshal = CppMarshal::new(spec);
    let refs = WasmRefs::new(spec, &ident.name, None);

    let _cls = cpp_marshal.fq_typename_from_name(&ident.name, &TypeDef::Enum(e.clone()));
    let helper = helper_class_name(&ident.name, spec);
    let cls = with_ns(Some(&spec.cpp_namespace), &(spec.cpp_ident_style.enum_type)(&ident.name));
    let fully_qualified_name = with_wasm_namespace(&(spec.js_ident_style.ty)(&ident.name), spec);
    let helper_ns = helper_namespace(spec);
    let wasm_file_style = |name: &str| -> String { wasm_filename_style(name, spec) };

    // HPP
    let hpp_includes: Vec<String> = refs.hpp.iter().cloned().collect();
    let hpp_name = format!("{}.{}", wasm_file_style(&ident.name), spec.cpp_header_ext);
    let cpp_header_ext = spec.cpp_header_ext.clone();
    let cpp_ext = spec.cpp_ext.clone();
    let wasm_include_prefix = spec.wasm_include_prefix.clone();
    let wasm_omit_constants = spec.wasm_omit_constants;
    let wasm_omit_ns_alias = spec.wasm_omit_ns_alias;
    let wasm_namespace = spec.wasm_namespace.clone();

    create_file_from_parts(out_files, written_files, folder, &hpp_name, |w| {
        w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
        w.wl(&format!("// This file was generated by Djinni from {}", origin));
        w.wl_empty();
        w.wl("#pragma once");
        w.wl_empty();
        for inc in &hpp_includes {
            w.wl(inc);
        }
        w.wl_empty();
        wrap_namespace(w, &helper_ns, |w| {
            w.w(&format!("struct {}: ::djinni::WasmEnum<{}>", helper, cls));
            w.braced_semi(|w| {
                if !wasm_omit_constants {
                    w.wl("static void staticInitializeConstants();");
                }
            });
        });
    });

    // CPP (only if constants are not omitted)
    if !wasm_omit_constants {
        let (spec, out_files, written_files) = ctx.split_borrow();
        let mut cpp_includes = refs.cpp.clone();
        cpp_includes.insert("#include <mutex>".to_string());

        let cpp_name = format!("{}.{}", wasm_filename_style(&ident.name, spec), cpp_ext);
        let my_header = format!(
            "\"{}{}\"",
            wasm_include_prefix,
            format!("{}.{}", wasm_filename_style(&ident.name, spec), cpp_header_ext)
        );
        let my_header_include = format!("#include {}", my_header);
        let id_js = &spec.js_ident_style;
        let cpp_ns_name = with_cpp_namespace(&ident.name, spec);
        let e_clone = e.clone();

        create_file_from_parts(out_files, written_files, folder, &cpp_name, |w| {
            w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
            w.wl(&format!("// This file was generated by Djinni from {}", origin));
            w.wl_empty();
            w.wl(&format!("#include {}  // my header", my_header));
            for inc in &cpp_includes {
                if *inc != my_header_include {
                    w.wl(inc);
                }
            }
            w.wl_empty();
            wrap_namespace(w, &helper_ns, |w| {
                w.w("namespace");
                w.braced(|w| {
                    w.wl(&format!("EM_JS(void, djinni_init_{}_consts, (), {{", cpp_ns_name));
                    w.nested(|w| {
                        w.w(&format!("Module.{} = ", fully_qualified_name));
                        w.braced(|w| {
                            write_enum_option_none(w, &e_clone, &|name| (id_js.enum_)(name), ":");
                            write_enum_options(w, &e_clone, &|name| (id_js.enum_)(name), ":");
                            write_enum_option_all(w, &e_clone, &|name| (id_js.enum_)(name), ":");
                        });
                    });
                    w.wl("})");
                });
                w.wl_empty();
                w.w(&format!("void {}::staticInitializeConstants()", helper));
                w.braced(|w| {
                    w.wl("static std::once_flag initOnce;");
                    w.wl("std::call_once(initOnce, [] {");
                    w.wl(&format!("    djinni_init_{}_consts();", cpp_ns_name));
                    if !wasm_omit_ns_alias && wasm_namespace.is_some() {
                        let ns = wasm_namespace.as_ref().unwrap();
                        w.wl(&format!(
                            "    ::djinni::djinni_register_name_in_ns(\"{}\", \"{}.{}\");",
                            fully_qualified_name,
                            ns,
                            (id_js.ty)(&ident.name)
                        ));
                    }
                    w.wl("});");
                });
                w.wl_empty();
                w.w(&format!("EMSCRIPTEN_BINDINGS({})", cpp_ns_name));
                w.braced(|w| {
                    w.wl(&format!("{}::staticInitializeConstants();", helper));
                });
            });
        });
    }
}

// --- Record Generation ---

fn generate_record(
    ctx: &mut GeneratorContext,
    folder: &Path,
    origin: &str,
    ident: &Ident,
    _params: &[TypeParam],
    r: &Record,
    _idl: &[TypeDecl],
) {
    let (spec, out_files, written_files) = ctx.split_borrow();

    let mut refs = WasmRefs::new(spec, &ident.name, None);
    for f in &r.fields {
        refs.find_type_ref(&f.ty, spec);
    }
    for c in &r.consts {
        refs.find_type_ref(&c.ty, spec);
    }

    let cls = with_ns(Some(&spec.cpp_namespace), &(spec.cpp_ident_style.ty)(&ident.name));
    let helper = helper_class_name(&ident.name, spec);
    let helper_ns = helper_namespace(spec);
    let wasm_omit_constants = spec.wasm_omit_constants;

    // HPP
    let hpp_includes: Vec<String> = refs.hpp.iter().cloned().collect();
    let hpp_name = format!("{}.{}", wasm_filename_style(&ident.name, spec), spec.cpp_header_ext);

    create_file_from_parts(out_files, written_files, folder, &hpp_name, |w| {
        w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
        w.wl(&format!("// This file was generated by Djinni from {}", origin));
        w.wl_empty();
        w.wl("#pragma once");
        w.wl_empty();
        for inc in &hpp_includes {
            w.wl(inc);
        }
        w.wl_empty();
        wrap_namespace(w, &helper_ns, |w| {
            w.wl(&format!("struct {}", helper));
            w.braced_semi(|w| {
                w.wl(&format!("using CppType = {};", cls));
                w.wl("using JsType = em::val;");
                w.wl(&format!("using Boxed = {};", helper));
                w.wl_empty();
                w.wl("static CppType toCpp(const JsType& j);");
                w.wl("static JsType fromCpp(const CppType& c);");
                if !wasm_omit_constants && !r.consts.is_empty() {
                    w.wl("static void staticInitializeConstants();");
                }
            });
        });
    });

    // CPP
    let (spec, out_files, written_files) = ctx.split_borrow();
    let cpp_marshal = CppMarshal::new(spec);
    let id_cpp = &spec.cpp_ident_style;
    let id_js = &spec.js_ident_style;
    let helper_ns = helper_namespace(spec);

    let cpp_name = format!("{}.{}", wasm_filename_style(&ident.name, spec), spec.cpp_ext);
    let my_header = format!(
        "\"{}{}\"",
        spec.wasm_include_prefix,
        format!("{}.{}", wasm_filename_style(&ident.name, spec), spec.cpp_header_ext)
    );
    let my_header_include = format!("#include {}", my_header);
    let cpp_includes: Vec<String> = refs.cpp.iter().cloned().collect();

    let r_fields = r.fields.clone();
    let r_consts = r.consts.clone();
    let wasm_omit_constants = spec.wasm_omit_constants;
    let ident_name = ident.name.clone();

    create_file_from_parts(out_files, written_files, folder, &cpp_name, |w| {
        w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
        w.wl(&format!("// This file was generated by Djinni from {}", origin));
        w.wl_empty();
        w.wl(&format!("#include {}  // my header", my_header));
        for inc in &cpp_includes {
            if *inc != my_header_include {
                w.wl(inc);
            }
        }
        w.wl_empty();
        wrap_namespace(w, &helper_ns, |w| {
            // toCpp
            w.w(&format!("auto {}::toCpp(const JsType& j) -> CppType", helper));
            w.braced(|w| {
                write_aligned_call(
                    w,
                    "return {",
                    &r_fields,
                    ",",
                    "}",
                    |f| {
                        let js_field = (id_js.field)(&f.ident.name);
                        let resolved = f.ty.resolved.as_ref().unwrap();
                        format!(
                            "{}::Boxed::toCpp(j[\"{}\"])",
                            helper_class_mexpr(resolved, spec),
                            js_field
                        )
                    },
                );
                w.wl(";");
            });
            // fromCpp
            w.w(&format!("auto {}::fromCpp(const CppType& c) -> JsType", helper));
            w.braced(|w| {
                w.wl("em::val js = em::val::object();");
                for f in &r_fields {
                    let js_field = (id_js.field)(&f.ident.name);
                    let cpp_field = (id_cpp.field)(&f.ident.name);
                    let resolved = f.ty.resolved.as_ref().unwrap();
                    let move_expr = cpp_marshal.maybe_move(&format!("c.{}", cpp_field), &f.ty);
                    w.wl(&format!(
                        "js.set(\"{}\", {}::Boxed::fromCpp({}));",
                        js_field,
                        helper_class_mexpr(resolved, spec),
                        move_expr
                    ));
                }
                w.wl("return js;");
            });
            // constants
            if !wasm_omit_constants && !r_consts.is_empty() {
                generate_wasm_constants(
                    w,
                    &ident_name,
                    &r_consts,
                    spec,
                    &helper,
                );
            }
        });
    });
}

// --- Interface Generation ---

fn generate_interface(
    ctx: &mut GeneratorContext,
    folder: &Path,
    origin: &str,
    ident: &Ident,
    _params: &[TypeParam],
    i: &Interface,
    _idl: &[TypeDecl],
) {
    let (spec, out_files, written_files) = ctx.split_borrow();
    let cpp_marshal = CppMarshal::new(spec);

    let mut refs = WasmRefs::new(spec, &ident.name, None);
    for c in &i.consts {
        refs.find_type_ref(&c.ty, spec);
    }
    for m in &i.methods {
        for p in &m.params {
            refs.find_type_ref(&p.ty, spec);
        }
        if let Some(ref ret) = m.ret {
            refs.find_type_ref(ret, spec);
        }
    }

    let cls = with_ns(Some(&spec.cpp_namespace), &(spec.cpp_ident_style.ty)(&ident.name));
    let helper = helper_class_name(&ident.name, spec);
    let helper_ns = helper_namespace(spec);
    let id_cpp = &spec.cpp_ident_style;
    let wasm_omit_constants = spec.wasm_omit_constants;

    // HPP
    let hpp_includes: Vec<String> = refs.hpp.iter().cloned().collect();
    let hpp_name = format!("{}.{}", wasm_filename_style(&ident.name, spec), spec.cpp_header_ext);

    let i_ext_cpp = i.ext.cpp;
    let i_ext_js = i.ext.js;
    let methods = i.methods.clone();
    let consts = i.consts.clone();

    create_file_from_parts(out_files, written_files, folder, &hpp_name, |w| {
        w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
        w.wl(&format!("// This file was generated by Djinni from {}", origin));
        w.wl_empty();
        w.wl("#pragma once");
        w.wl_empty();
        for inc in &hpp_includes {
            w.wl(inc);
        }
        w.wl_empty();
        wrap_namespace(w, &helper_ns, |w| {
            w.w(&format!("struct {} : ::djinni::JsInterface<{}, {}>", helper, cls, helper));
            w.braced_semi(|w| {
                // types
                w.wl(&format!("using CppType = std::shared_ptr<{}>;", cls));
                w.wl(&format!("using CppOptType = std::shared_ptr<{}>;", cls));
                w.wl("using JsType = em::val;");
                w.wl(&format!("using Boxed = {};", helper));
                w.wl_empty();
                // marshalling
                w.wl("static CppType toCpp(JsType j) { return _fromJs(j); }");
                w.wl("static JsType fromCppOpt(const CppOptType& c) { return {_toJs(c)}; }");
                w.w("static JsType fromCpp(const CppType& c)");
                w.braced(|w| {
                    if spec.cpp_nn_type.is_none() {
                        w.wl(&format!(
                            "::djinni::checkForNull(c.get(), \"{}::fromCpp\");",
                            helper
                        ));
                    }
                    w.wl("return fromCppOpt(c);");
                });
                w.wl_empty();
                // method list
                if i_ext_cpp {
                    w.wl("static em::val cppProxyMethods();");
                }
                w.wl_empty();
                // stubs
                if i_ext_cpp {
                    for m in &methods {
                        if !m.is_static || m.lang.js {
                            let self_ref = if m.is_static {
                                "".to_string()
                            } else if m.params.is_empty() {
                                "const CppType& self".to_string()
                            } else {
                                "const CppType& self, ".to_string()
                            };
                            let params_str: Vec<String> = m.params.iter().map(|p| {
                                format!(
                                    "{} {}",
                                    stub_param_type(&p.ty, spec),
                                    stub_param_name(&(id_cpp.local)(&p.ident.name), spec)
                                )
                            }).collect();
                            w.wl(&format!(
                                "static {} {}({}{});",
                                stub_ret_type(m, spec),
                                (id_cpp.method)(&m.ident.name),
                                self_ref,
                                params_str.join(",")
                            ));
                        }
                    }
                    w.wl_empty();
                }
                // js proxy
                if i_ext_js {
                    w.w(&format!(
                        "struct JsProxy: ::djinni::JsProxyBase, {}, ::djinni::InstanceTracker<JsProxy>",
                        cls
                    ));
                    w.braced_semi(|w| {
                        w.wl("JsProxy(const em::val& v) : JsProxyBase(v) {}");
                        for m in &methods {
                            if !m.is_static {
                                let ret_type = cpp_marshal.fq_return_type(m.ret.as_ref());
                                let params_str: Vec<String> = m.params.iter().map(|p| {
                                    format!(
                                        "{} {}",
                                        cpp_marshal.fq_param_type_from_typeref(&p.ty),
                                        (id_cpp.local)(&p.ident.name)
                                    )
                                }).collect();
                                let const_mod = if m.is_const { " const" } else { "" };
                                w.wl(&format!(
                                    "{} {}({}){} override;",
                                    ret_type,
                                    (id_cpp.method)(&m.ident.name),
                                    params_str.join(","),
                                    const_mod
                                ));
                            }
                        }
                    });
                }
                // init consts
                if !wasm_omit_constants && !consts.is_empty() {
                    w.wl("static void staticInitializeConstants();");
                }
            });
        });
    });

    // CPP
    let (spec, out_files, written_files) = ctx.split_borrow();
    let cpp_marshal = CppMarshal::new(spec);
    let id_cpp = &spec.cpp_ident_style;
    let id_js = &spec.js_ident_style;
    let helper_ns = helper_namespace(spec);

    let cpp_name = format!("{}.{}", wasm_filename_style(&ident.name, spec), spec.cpp_ext);
    let my_header = format!(
        "\"{}{}\"",
        spec.wasm_include_prefix,
        format!("{}.{}", wasm_filename_style(&ident.name, spec), spec.cpp_header_ext)
    );
    let my_header_include = format!("#include {}", my_header);
    let cpp_includes: Vec<String> = refs.cpp.iter().cloned().collect();

    let wasm_omit_constants = spec.wasm_omit_constants;
    let wasm_omit_ns_alias = spec.wasm_omit_ns_alias;
    let wasm_namespace = spec.wasm_namespace.clone();
    let ident_name = ident.name.clone();

    create_file_from_parts(out_files, written_files, folder, &cpp_name, |w| {
        w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
        w.wl(&format!("// This file was generated by Djinni from {}", origin));
        w.wl_empty();
        w.wl(&format!("#include {}  // my header", my_header));
        for inc in &cpp_includes {
            if *inc != my_header_include {
                w.wl(inc);
            }
        }
        w.wl_empty();
        wrap_namespace(w, &helper_ns, |w| {
            // method list
            if i_ext_cpp {
                w.w(&format!("em::val {}::cppProxyMethods()", helper));
                w.braced(|w| {
                    w.w("static const em::val methods = em::val::array(std::vector<std::string>");
                    w.braced_end(");", |w| {
                        for m in &methods {
                            if !m.is_static {
                                w.wl(&format!("\"{}\",", (id_js.method)(&m.ident.name)));
                            }
                        }
                    });
                    w.wl("return methods;");
                });
            }
            w.wl_empty();
            // stub methods
            if i_ext_cpp {
                for m in &methods {
                    if !m.is_static || m.lang.js {
                        let self_ref = if m.is_static {
                            "".to_string()
                        } else if m.params.is_empty() {
                            "const CppType& self".to_string()
                        } else {
                            "const CppType& self, ".to_string()
                        };
                        let params_str: Vec<String> = m.params.iter().map(|p| {
                            format!(
                                "{} {}",
                                stub_param_type(&p.ty, spec),
                                stub_param_name(&p.ident.name, spec)
                            )
                        }).collect();
                        w.w(&format!(
                            "{} {}::{}({}{})",
                            stub_ret_type(m, spec),
                            helper,
                            (id_cpp.method)(&m.ident.name),
                            self_ref,
                            params_str.join(",")
                        ));
                        w.braced(|w| {
                            w.w("try");
                            w.braced(|w| {
                                if m.ret.is_some() {
                                    w.w("auto r = ");
                                }
                                if m.is_static {
                                    w.w(&format!("{}::", cls));
                                } else {
                                    w.w("self->");
                                }
                                write_aligned_call(
                                    w,
                                    &format!("{}(", (id_cpp.method)(&m.ident.name)),
                                    &m.params,
                                    ",",
                                    ")",
                                    |p| {
                                        let resolved = p.ty.resolved.as_ref().unwrap();
                                        format!(
                                            "{}::toCpp({})",
                                            helper_class_mexpr(resolved, spec),
                                            stub_param_name(&p.ident.name, spec)
                                        )
                                    },
                                );
                                w.wl(";");
                                if let Some(ref ret) = m.ret {
                                    let resolved = ret.resolved.as_ref().unwrap();
                                    let move_expr = cpp_marshal.maybe_move("r", ret);
                                    w.wl(&format!(
                                        "return {}::fromCpp({});",
                                        helper_class_mexpr(resolved, spec),
                                        move_expr
                                    ));
                                }
                            });
                            w.w("catch(const std::exception& e)");
                            w.braced(|w| {
                                let helper_type = if m.ret.is_some() {
                                    let resolved = m.ret.as_ref().unwrap().resolved.as_ref().unwrap();
                                    helper_class_mexpr(resolved, spec)
                                } else {
                                    "void".to_string()
                                };
                                w.wl(&format!(
                                    "return ::djinni::ExceptionHandlingTraits<{}>::handleNativeException(e);",
                                    helper_type
                                ));
                            });
                        });
                    }
                }
                w.wl_empty();
            }
            // js proxy methods
            if i_ext_js {
                for m in &methods {
                    if !m.is_static {
                        let const_mod = if m.is_const { " const" } else { "" };
                        let ret_type = cpp_marshal.fq_return_type(m.ret.as_ref());
                        let params_str: Vec<String> = m.params.iter().map(|p| {
                            format!(
                                "{} {}",
                                cpp_marshal.fq_param_type_from_typeref(&p.ty),
                                (id_cpp.local)(&p.ident.name)
                            )
                        }).collect();
                        w.w(&format!(
                            "{} {}::JsProxy::{}({}){}",
                            ret_type,
                            helper,
                            (id_cpp.method)(&m.ident.name),
                            params_str.join(","),
                            const_mod
                        ));
                        w.braced(|w| {
                            let method_name_arg = if m.params.is_empty() {
                                q(&(id_js.method)(&m.ident.name))
                            } else {
                                format!("{}, ", q(&(id_js.method)(&m.ident.name)))
                            };
                            write_aligned_call(
                                w,
                                &format!("auto ret = callMethod({}", method_name_arg),
                                &m.params,
                                ",",
                                ")",
                                |p| {
                                    let resolved = p.ty.resolved.as_ref().unwrap();
                                    let move_expr = cpp_marshal.maybe_move(
                                        &(id_cpp.local)(&p.ident.name),
                                        &p.ty,
                                    );
                                    format!("{}::fromCpp({})", helper_class_mexpr(resolved, spec), move_expr)
                                },
                            );
                            w.wl(";");
                            w.wl("checkError(ret);");
                            let srt = stub_ret_type(m, spec);
                            match srt.as_str() {
                                "void" => {}
                                "em::val" => {
                                    let resolved = m.ret.as_ref().unwrap().resolved.as_ref().unwrap();
                                    w.wl(&format!(
                                        "return {}::toCpp(ret);",
                                        helper_class_mexpr(resolved, spec)
                                    ));
                                }
                                _ => {
                                    let resolved = m.ret.as_ref().unwrap().resolved.as_ref().unwrap();
                                    w.wl(&format!(
                                        "return {}::toCpp(ret.as<{}>());",
                                        helper_class_mexpr(resolved, spec),
                                        srt
                                    ));
                                }
                            }
                        });
                        w.wl_empty();
                    }
                }
            }

            let fully_qualified_name = with_cpp_namespace(&ident_name, spec);
            let fully_qualified_js_name = with_wasm_namespace(&(id_js.ty)(&ident_name), spec);

            // embind
            w.w(&format!("EMSCRIPTEN_BINDINGS({})", fully_qualified_name));
            w.braced(|w| {
                let class_register = if !wasm_omit_ns_alias && wasm_namespace.is_some() {
                    let ns = wasm_namespace.as_ref().unwrap();
                    format!(
                        "::djinni::DjinniClass_<{}>(\"{}\", \"{}.{}\")",
                        cls,
                        fully_qualified_js_name,
                        ns,
                        (id_js.ty)(&ident_name)
                    )
                } else {
                    format!("em::class_<{}>(\"{}\")", cls, fully_qualified_js_name)
                };

                w.wl(&class_register);
                w.nested(|w| {
                    w.wl(&format!(
                        ".smart_ptr<std::shared_ptr<{}>>(\"{}\")",
                        cls,
                        fully_qualified_js_name
                    ));
                    w.wl(&format!(
                        ".function(\"{}\", &{}::nativeDestroy)",
                        (id_js.method)("native_destroy"),
                        helper
                    ));
                    if i_ext_cpp {
                        for m in &methods {
                            if !m.is_static || m.lang.js {
                                let func_type = if m.is_static {
                                    "class_function"
                                } else {
                                    "function"
                                };
                                w.wl(&format!(
                                    ".{}(\"{}\", {}::{})",
                                    func_type,
                                    (id_js.method)(&m.ident.name),
                                    helper,
                                    (id_cpp.method)(&m.ident.name)
                                ));
                            }
                        }
                    }
                    w.wl(";");
                });
            });
            // constants
            if !wasm_omit_constants && !consts.is_empty() {
                generate_wasm_constants(
                    w,
                    &ident_name,
                    &consts,
                    spec,
                    &helper,
                );
            }
        });
    });
}

// --- Constants Generation ---

fn generate_wasm_constants(
    w: &mut IndentWriter,
    ident_name: &str,
    consts: &[Const],
    spec: &Spec,
    helper: &str,
) {
    let id_js = &spec.js_ident_style;
    let fully_qualified_name = with_wasm_namespace(&(id_js.ty)(ident_name), spec);
    let cpp_ns_name = with_cpp_namespace(ident_name, spec);
    let wasm_omit_ns_alias = spec.wasm_omit_ns_alias;
    let wasm_namespace = &spec.wasm_namespace;

    let mut dependent_types = BTreeSet::new();

    w.wl_empty();
    w.w("namespace");
    w.braced(|w| {
        w.wl(&format!("EM_JS(void, djinni_init_{}_consts, (), {{", cpp_ns_name));
        w.nested(|w| {
            w.w(&format!("if (!('{}' in Module))", fully_qualified_name));
            w.braced(|w| {
                w.wl(&format!("Module.{} = {{}};", fully_qualified_name));
            });
            for c in consts {
                w.w(&format!(
                    "Module.{}.{} = ",
                    fully_qualified_name,
                    (id_js.const_)(&c.ident.name)
                ));
                write_js_const(w, &c.ty, &c.value, spec, ident_name, &mut dependent_types);
                w.wl(";");
            }
        });
        w.wl("})");
    });
    w.w(&format!("void {}::staticInitializeConstants()", helper));
    w.braced(|w| {
        w.wl("static std::once_flag initOnce;");
        w.wl("std::call_once(initOnce, [] {");
        w.wl(&format!("    djinni_init_{}_consts();", cpp_ns_name));
        if !wasm_omit_ns_alias && wasm_namespace.is_some() {
            let ns = wasm_namespace.as_ref().unwrap();
            let id_js = &spec.js_ident_style;
            w.wl(&format!(
                "    ::djinni::djinni_register_name_in_ns(\"{}\", \"{}.{}\");",
                fully_qualified_name,
                ns,
                (id_js.ty)(ident_name)
            ));
        }
        w.wl("});");
    });
    w.wl_empty();
    w.w(&format!("EMSCRIPTEN_BINDINGS({}_consts)", cpp_ns_name));
    w.braced(|w| {
        for d in &dependent_types {
            if *d != helper {
                w.wl(&format!("{}::staticInitializeConstants();", d));
            }
        }
        w.wl(&format!("{}::staticInitializeConstants();", helper));
    });
}

fn write_js_const(
    w: &mut IndentWriter,
    ty: &TypeRef,
    value: &ConstValue,
    spec: &Spec,
    ident_name: &str,
    dependent_types: &mut BTreeSet<String>,
) {
    let id_js = &spec.js_ident_style;
    match value {
        ConstValue::Int(l) => {
            let wt = wasm_type_ref(ty, spec);
            if wt.eq_ignore_ascii_case("int64_t") {
                w.w(&format!("BigInt(\"{}\")", l));
            } else {
                w.w(&format!("{}", l));
            }
        }
        ConstValue::Float(d) => {
            // Ensure floats always have a decimal point (e.g. 5.0 not 5)
            let s = format!("{}", d);
            if s.contains('.') {
                w.w(&s);
            } else {
                w.w(&format!("{}.0", s));
            }
        }
        ConstValue::Bool(b) => {
            w.w(if *b { "true" } else { "false" });
        }
        ConstValue::String(s) => {
            w.w(&format!("\"{}\"", s));
        }
        ConstValue::EnumValue { ty: _enum_ty, value: enum_val } => {
            let js_ty = (id_js.ty)(&ty.expr.ident.name);
            let js_enum = (id_js.enum_)(enum_val);
            w.w(&format!(
                "Module.{}.{}",
                with_wasm_namespace(&js_ty, spec),
                js_enum
            ));
            dependent_types.insert(helper_class_name(&ty.expr.ident.name, spec));
        }
        ConstValue::ConstRef(v) => {
            w.w(&format!(
                "Module.{}.{}",
                with_wasm_namespace(&(id_js.ty)(ident_name), spec),
                (id_js.const_)(v)
            ));
        }
        ConstValue::Composite(fields) => {
            // Value is record - get the record definition to iterate fields in order
            let resolved = ty.resolved.as_ref().unwrap();
            if let Meta::MDef(mdef) = &resolved.base {
                if let TypeDef::Record(record) = &mdef.body {
                    w.w("");
                    w.braced(|w| {
                        let mut first = true;
                        for f in &record.fields {
                            if let Some((_, val)) = fields.iter().find(|(k, _)| *k == f.ident.name) {
                                if !first {
                                    w.wl(",");
                                }
                                first = false;
                                w.w(&format!("{}: ", (id_js.field)(&f.ident.name)));
                                write_js_const(w, &f.ty, val, spec, ident_name, dependent_types);
                            }
                        }
                        w.wl_empty();
                    });
                }
            }
        }
    }
}
