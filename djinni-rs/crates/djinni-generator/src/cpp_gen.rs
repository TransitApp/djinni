// vinsg
// CppGenerator: generates C++ header and source files, translated from CppGenerator.scala

use std::collections::BTreeSet;
use std::path::Path;

use djinni_ast::ast::*;
use djinni_ast::meta::*;
use djinni_ast::spec::{q, Spec};

use crate::cpp_marshal::{CppMarshal, SymbolReference};
use crate::gen::*;
use crate::writer::IndentWriter;

/// Format a float like Scala's Double.toString: always includes decimal point
fn fmt_float(d: f64) -> String {
    let s = d.to_string();
    if s.contains('.') || s.contains('E') || s.contains('e') {
        s
    } else {
        format!("{}.0", s)
    }
}

pub fn generate_cpp(ctx: &mut GeneratorContext, idl: &[TypeDecl]) {
    let cpp_out = match &ctx.spec.cpp_out_folder {
        Some(f) => f.clone(),
        None => return,
    };
    let cpp_header_out = ctx.spec.cpp_header_out_folder.as_ref().unwrap_or(&cpp_out).clone();

    // Collect intern type declarations to process
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

    for (ident, params, body, doc, origin) in &intern_decls {
        match body {
            TypeDef::Enum(e) => {
                assert!(params.is_empty());
                generate_enum(ctx, &cpp_header_out, origin, ident, &doc, e);
            }
            TypeDef::Record(r) => {
                generate_record(
                    ctx, &cpp_out, &cpp_header_out, origin, ident, &doc, params, r,
                    idl,
                );
            }
            TypeDef::Interface(i) => {
                generate_interface(
                    ctx, &cpp_out, &cpp_header_out, origin, ident, &doc, params, i,
                );
            }
            TypeDef::ProtobufMessage(_) => {}
        }
    }
}

// --- CppRefs: collects include references ---

struct CppRefs {
    name: String,
    hpp: BTreeSet<String>,
    hpp_fwds: BTreeSet<String>,
    cpp: BTreeSet<String>,
}

impl CppRefs {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            hpp: BTreeSet::new(),
            hpp_fwds: BTreeSet::new(),
            cpp: BTreeSet::new(),
        }
    }

    fn find_type_ref(&mut self, ty: &TypeRef, marshal: &CppMarshal, forward_declare_only: bool) {
        if let Some(resolved) = &ty.resolved {
            self.find_mexpr(resolved, marshal, forward_declare_only);
        }
    }

    fn find_mexpr(&mut self, tm: &MExpr, marshal: &CppMarshal, forward_declare_only: bool) {
        for arg in &tm.args {
            self.find_mexpr(arg, marshal, forward_declare_only);
        }
        self.find_meta(&tm.base, marshal, forward_declare_only);
    }

    fn find_meta(&mut self, m: &Meta, marshal: &CppMarshal, forward_declare_only: bool) {
        let cpp_ns = &marshal.spec.cpp_namespace;
        for r in marshal.hpp_references(m, &self.name, forward_declare_only) {
            match r {
                SymbolReference::ImportRef(arg) => {
                    for part in arg.split(',') {
                        self.hpp.insert(format!("#include {}", part.trim()));
                    }
                }
                SymbolReference::DeclRef { decl, namespace } => {
                    if namespace.as_deref() == Some(cpp_ns) {
                        self.hpp_fwds.insert(decl);
                    }
                }
            }
        }
        for r in marshal.cpp_references(m, &self.name, forward_declare_only) {
            match r {
                SymbolReference::ImportRef(arg) => {
                    self.cpp.insert(format!("#include {}", arg));
                }
                SymbolReference::DeclRef { .. } => {}
            }
        }
    }
}

// --- Enum Generation ---

fn generate_enum(
    ctx: &mut GeneratorContext,
    hpp_folder: &Path,
    origin: &str,
    ident: &Ident,
    _doc: &Doc,
    e: &Enum,
) {
    let marshal = CppMarshal::new(&ctx.spec);
    let mut refs = CppRefs::new(&ident.name);
    let self_name = marshal.typename_from_name(&ident.name, &TypeDef::Enum(e.clone()));

    if ctx.spec.cpp_enum_hash_workaround {
        refs.hpp.insert("#include <functional>".into());
    }

    let underlying_type = if e.flags { "int32_t" } else { "int" };

    let file_ident = (ctx.spec.cpp_file_ident_style)(&ident.name);
    let hpp_name = format!("{}.{}", file_ident, ctx.spec.cpp_header_ext);

    // HPP
    let hpp_includes: Vec<String> = refs.hpp.iter().cloned().collect();
    let hpp_fwds: Vec<String> = refs.hpp_fwds.iter().cloned().collect();
    let fq_self = marshal.fq_typename_from_name(&ident.name, &TypeDef::Enum(e.clone()));
    let enum_hash_workaround = ctx.spec.cpp_enum_hash_workaround;
    let cpp_namespace = ctx.spec.cpp_namespace.clone();
    let id_enum = ctx.spec.cpp_ident_style.enum_;
    let id_method = ctx.spec.cpp_ident_style.method;
    drop(marshal);

    ctx.create_file(hpp_folder, &hpp_name, |w| {
        w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
        w.wl(&format!("// This file was generated by Djinni from {}", origin));
        w.wl_empty();
        w.wl("#pragma once");
        if !hpp_includes.is_empty() {
            w.wl_empty();
            for inc in &hpp_includes {
                w.wl(inc);
            }
        }
        w.wl_empty();
        wrap_namespace(w, &cpp_namespace, |w| {
            if !hpp_fwds.is_empty() {
                for fwd in &hpp_fwds {
                    w.wl(fwd);
                }
                w.wl_empty();
            }
            // enum class
            w.w(&format!("enum class {} : {}", self_name, underlying_type));
            w.braced_semi(|w| {
                write_enum_option_none(w, e, id_enum, "=");
                write_enum_options(w, e, id_enum, "=");
                write_enum_option_all(w, e, id_enum, "=");
            });

            if e.flags {
                // Bitwise operators for flags
                for op in &["|", "&", "^"] {
                    w.w(&format!(
                        "constexpr {} operator{}({} lhs, {} rhs) noexcept",
                        self_name, op, self_name, self_name
                    ));
                    w.braced(|w| {
                        w.wl(&format!(
                            "return static_cast<{}>(static_cast<int32_t>(lhs) {} static_cast<int32_t>(rhs));",
                            self_name, op
                        ));
                    });
                    w.w(&format!(
                        "constexpr {}& operator{}=({}& lhs, {} rhs) noexcept",
                        self_name, op, self_name, self_name
                    ));
                    w.braced(|w| {
                        w.wl(&format!("return lhs = lhs {} rhs;", op));
                    });
                }
                w.w(&format!(
                    "constexpr {} operator~({} x) noexcept",
                    self_name, self_name
                ));
                w.braced(|w| {
                    w.wl(&format!(
                        "return static_cast<{}>(~static_cast<int32_t>(x));",
                        self_name
                    ));
                });
            } else {
                w.wl_empty();
                // to_string function
                w.w(&format!(
                    "constexpr const char* {}({} e) noexcept",
                    id_method("to_string"),
                    self_name
                ));
                w.braced(|w| {
                    w.w("constexpr const char* names[] =");
                    w.braced_semi(|w| {
                        for o in &e.options {
                            w.wl(&format!("\"{}\",", o.ident.name));
                        }
                    });
                    w.wl(&format!(
                        "return names[static_cast<{}>(e)];",
                        underlying_type
                    ));
                });
            }
        });

        // Hash specialization outside namespace
        if enum_hash_workaround {
            w.wl_empty();
            wrap_namespace(w, "std", |w| {
                w.wl("template <>");
                w.w(&format!("struct hash<{}>", fq_self));
                w.braced_semi(|w| {
                    w.w(&format!("size_t operator()({} type) const", fq_self));
                    w.braced(|w| {
                        w.wl(&format!(
                            "return std::hash<{}>()(static_cast<{}>(type));",
                            underlying_type, underlying_type
                        ));
                    });
                });
            });
        }
    });
}

// --- Record Generation ---

fn generate_record(
    ctx: &mut GeneratorContext,
    cpp_folder: &Path,
    hpp_folder: &Path,
    origin: &str,
    ident: &Ident,
    doc: &Doc,
    params: &[TypeParam],
    r: &Record,
    idl: &[TypeDecl],
) {
    let (spec, out_files, written_files) = ctx.split_borrow();
    let marshal = CppMarshal::new(spec);
    let mut refs = CppRefs::new(&ident.name);
    for f in &r.fields {
        refs.find_type_ref(&f.ty, &marshal, false);
    }
    for c in &r.consts {
        refs.find_type_ref(&c.ty, &marshal, false);
    }
    refs.hpp.insert("#include <utility>".into());
    refs.hpp.insert("#include <sstream>".into());
    refs.cpp.insert("#include \"BuildConstants.h\"".into());

    let self_name = marshal.typename_from_name(&ident.name, &TypeDef::Record(r.clone()));
    let is_record_inherited = is_inherited(idl, &ident.name);

    let super_record = get_super_record(idl, r);

    if let Some(ref sr) = super_record {
        refs.hpp.insert(format!(
            "#include {}",
            q(&format!(
                "{}{}{}",
                spec.cpp_extended_record_include_prefix,
                (spec.cpp_file_ident_style)(&sr.ident.name),
                format!(".{}", spec.cpp_header_ext)
            ))
        ));
    }

    let super_fields: Vec<Field> = super_record
        .as_ref()
        .map(|sr| sr.fields.clone())
        .unwrap_or_default();

    let (cpp_name, cpp_final) = if r.ext.cpp {
        (format!("{}_base", ident.name), "".to_string())
    } else if !is_record_inherited && super_record.is_none() {
        (ident.name.clone(), " final".to_string())
    } else {
        (ident.name.clone(), String::new())
    };

    let actual_self = marshal.typename_from_name(&cpp_name, &TypeDef::Record(r.clone()));

    if r.ext.cpp {
        refs.cpp.insert(format!(
            "#include {}",
            q(&format!(
                "{}{}{}",
                spec.cpp_extended_record_include_prefix,
                (spec.cpp_file_ident_style)(&ident.name),
                format!(".{}", spec.cpp_header_ext)
            ))
        ));
    }

    let id_cpp = &spec.cpp_ident_style;
    let all_fields: Vec<Field> = [super_fields.clone(), r.fields.clone()].concat();

    // --- HPP ---
    let hpp_includes: Vec<String> = refs.hpp.iter().cloned().collect();
    let hpp_fwds: Vec<String> = refs.hpp_fwds.iter().cloned().collect();
    let file_ident = (spec.cpp_file_ident_style)(&cpp_name);
    let hpp_name = format!("{}.{}", file_ident, spec.cpp_header_ext);
    let cpp_namespace = spec.cpp_namespace.clone();

    create_file_from_parts(out_files, written_files, hpp_folder, &hpp_name, |w| {
        w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
        w.wl(&format!("// This file was generated by Djinni from {}", origin));
        w.wl_empty();
        w.wl("#pragma once");
        if !hpp_includes.is_empty() {
            w.wl_empty();
            for inc in &hpp_includes {
                w.wl(inc);
            }
        }
        w.wl_empty();
        wrap_namespace(w, &cpp_namespace, |w| {
            if !hpp_fwds.is_empty() {
                for fwd in &hpp_fwds {
                    w.wl(fwd);
                }
                w.wl_empty();
            }

            // Forward declare extended type
            if r.ext.cpp {
                w.wl(&format!("struct {}; // Requiring extended class", self_name));
                w.wl_empty();
            }

            // Doc comment (filter out @test-representation annotations)
            let filtered_doc = Doc {
                lines: doc
                    .lines
                    .iter()
                    .filter(|l| !l.contains("@test-representation-"))
                    .cloned()
                    .collect(),
            };
            write_doc(w, &filtered_doc);

            // Type params
            write_cpp_type_params(w, params, id_cpp);

            // Extends clause
            let extends = match &super_record {
                Some(sr) => {
                    let parent_name =
                        marshal.typename_from_name(&sr.ident.name, &TypeDef::Record(sr.record.clone()));
                    marshal.extends_record(&parent_name)
                }
                None => String::new(),
            };

            w.w(&format!("struct {}{}{}", actual_self, extends, cpp_final));
            w.braced_semi(|w| {
                // Constants
                generate_hpp_constants(w, &r.consts, &marshal, id_cpp);

                // Fields
                for f in &r.fields {
                    write_doc(w, &f.doc);
                    let default_val = if f.default_value.is_empty() {
                        String::new()
                    } else {
                        format!(" = {}", f.default_value)
                    };
                    w.wl(&format!(
                        "{} {}{};",
                        marshal.field_type_from_typeref(&f.ty, &[]),
                        (id_cpp.field)(&f.ident.name),
                        default_val
                    ));
                }

                // operator== / operator!=
                w.wl_empty();
                w.wl(&format!(
                    "friend bool operator==(const {}& lhs, const {}& rhs);",
                    actual_self, actual_self
                ));
                w.wl(&format!(
                    "friend bool operator!=(const {}& lhs, const {}& rhs);",
                    actual_self, actual_self
                ));

                // getTestRepresentation
                if !all_fields.is_empty() {
                    w.wl_empty();
                    let virtual_prefix = if is_record_inherited && super_record.is_none() {
                        "virtual "
                    } else {
                        ""
                    };
                    let override_suffix = if super_record.is_some() {
                        " override"
                    } else {
                        ""
                    };
                    w.wl(&format!(
                        "{}std::string getTestRepresentation(const std::string& indentation) const{};",
                        virtual_prefix, override_suffix
                    ));
                }

                // Ord operators
                if r.deriving_types.contains(&DerivingType::Ord) {
                    w.wl_empty();
                    w.wl(&format!(
                        "friend bool operator<(const {}& lhs, const {}& rhs);",
                        actual_self, actual_self
                    ));
                    w.wl(&format!(
                        "friend bool operator>(const {}& lhs, const {}& rhs);",
                        actual_self, actual_self
                    ));
                    w.wl_empty();
                    w.wl(&format!(
                        "friend bool operator<=(const {}& lhs, const {}& rhs);",
                        actual_self, actual_self
                    ));
                    w.wl(&format!(
                        "friend bool operator>=(const {}& lhs, const {}& rhs);",
                        actual_self, actual_self
                    ));
                }

                // Virtual destructor for inherited records
                if is_record_inherited {
                    w.wl_empty();
                    w.wl(&format!("virtual ~{}(){{}};", actual_self));
                }

                // Constructor
                if !r.fields.is_empty() && spec.cpp_struct_constructor {
                    w.wl_empty();
                    if r.fields.len() == 1 && super_fields.is_empty() {
                        w.wl("//NOLINTNEXTLINE(google-explicit-constructor)");
                    }
                    // Constructor signature
                    let ctor_params: Vec<Field> = [super_fields.clone(), r.fields.clone()].concat();
                    write_aligned_call(
                        w,
                        &format!("{}(", actual_self),
                        &ctor_params,
                        ",",
                        ")",
                        |f| {
                            format!(
                                "{} {}_",
                                marshal.field_type_from_typeref(&f.ty, &[]),
                                (id_cpp.local)(&f.ident.name)
                            )
                        },
                    );
                    w.wl_empty();

                    // Initializer list
                    let init = |f: &Field| -> String {
                        format!(
                            "{}(std::move({}_))",
                            (id_cpp.field)(&f.ident.name),
                            (id_cpp.local)(&f.ident.name)
                        )
                    };

                    match &super_record {
                        None => {
                            w.wl(&format!(": {}", init(&r.fields[0])));
                        }
                        Some(sr) => {
                            w.wl(": ");
                            let parent_name = marshal.typename_from_name(
                                &sr.ident.name,
                                &TypeDef::Record(sr.record.clone()),
                            );
                            write_aligned_call(
                                w,
                                &format!("{}(", parent_name),
                                &super_fields,
                                ",",
                                ")",
                                |f| format!(" {}_", (id_cpp.local)(&f.ident.name)),
                            );
                            w.w(&format!(", {}", init(&r.fields[0])));
                        }
                    }

                    for f in r.fields.iter().skip(1) {
                        w.wl(&format!(", {}", init(f)));
                    }
                    w.wl("{}");
                }

                // Extended record base class virtual dtor and copy/move
                if r.ext.cpp {
                    w.wl_empty();
                    w.wl(&format!("virtual ~{}() = default;", actual_self));
                    w.wl_empty();
                    w.wl_outdent("protected:");
                    w.wl(&format!(
                        "{}(const {}&) = default;",
                        actual_self, actual_self
                    ));
                    w.wl(&format!(
                        "{}({}&&) = default;",
                        actual_self, actual_self
                    ));
                    w.wl(&format!(
                        "{}& operator=(const {}&) = default;",
                        actual_self, actual_self
                    ));
                    w.wl(&format!(
                        "{}& operator=({}&&) = default;",
                        actual_self, actual_self
                    ));
                }
            });
        });
    });

    // --- CPP ---
    let cpp_includes: Vec<String> = refs.cpp.iter().cloned().collect();
    let cpp_name_file = format!("{}.{}", file_ident, spec.cpp_ext);
    let my_header = format!(
        "\"{}{}\"",
        spec.cpp_include_prefix,
        format!("{}.{}", file_ident, spec.cpp_header_ext)
    );
    let my_header_include = format!("#include {}", my_header);

    create_file_from_parts(out_files, written_files, cpp_folder, &cpp_name_file, |w| {
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
        wrap_namespace(w, &cpp_namespace, |w| {
            generate_cpp_constants(w, &r.consts, &actual_self, &marshal, id_cpp);

            // operator==
            w.wl_empty();
            w.w(&format!(
                "bool operator==(const {}& lhs, const {}& rhs)",
                actual_self, actual_self
            ));
            w.braced(|w| {
                if !all_fields.is_empty() {
                    write_aligned_call(w, "return ", &all_fields, " &&", "", |f| {
                        format!(
                            "lhs.{} == rhs.{}",
                            (id_cpp.field)(&f.ident.name),
                            (id_cpp.field)(&f.ident.name)
                        )
                    });
                    w.wl(";");
                } else {
                    w.wl("return true;");
                }
            });

            // operator!=
            w.wl_empty();
            w.w(&format!(
                "bool operator!=(const {}& lhs, const {}& rhs)",
                actual_self, actual_self
            ));
            w.braced(|w| {
                w.wl("return !(lhs == rhs);");
            });

            // getTestRepresentation
            write_cpp_get_test_representation(w, &actual_self, &all_fields, &r.fields, &super_record, doc, &marshal, id_cpp);

            // Ord operators
            if r.deriving_types.contains(&DerivingType::Ord) {
                w.wl_empty();
                w.w(&format!(
                    "bool operator<(const {}& lhs, const {}& rhs)",
                    actual_self, actual_self
                ));
                w.braced(|w| {
                    for f in &all_fields {
                        let fname = (id_cpp.field)(&f.ident.name);
                        w.w(&format!("if (lhs.{} < rhs.{})", fname, fname));
                        w.braced(|w| {
                            w.wl("return true;");
                        });
                        w.w(&format!("if (rhs.{} < lhs.{})", fname, fname));
                        w.braced(|w| {
                            w.wl("return false;");
                        });
                    }
                    w.wl("return false;");
                });
                w.wl_empty();
                w.w(&format!(
                    "bool operator>(const {}& lhs, const {}& rhs)",
                    actual_self, actual_self
                ));
                w.braced(|w| {
                    w.wl("return rhs < lhs;");
                });
                w.wl_empty();
                w.w(&format!(
                    "bool operator<=(const {}& lhs, const {}& rhs)",
                    actual_self, actual_self
                ));
                w.braced(|w| {
                    w.wl("return !(rhs < lhs);");
                });
                w.wl_empty();
                w.w(&format!(
                    "bool operator>=(const {}& lhs, const {}& rhs)",
                    actual_self, actual_self
                ));
                w.braced(|w| {
                    w.wl("return !(lhs < rhs);");
                });
            }
        });
    });
}

// --- Interface Generation ---

fn generate_interface(
    ctx: &mut GeneratorContext,
    cpp_folder: &Path,
    hpp_folder: &Path,
    origin: &str,
    ident: &Ident,
    doc: &Doc,
    params: &[TypeParam],
    i: &Interface,
) {
    let (spec, out_files, written_files) = ctx.split_borrow();
    let marshal = CppMarshal::new(spec);
    let mut refs = CppRefs::new(&ident.name);
    for m in &i.methods {
        for p in &m.params {
            refs.find_type_ref(&p.ty, &marshal, true);
        }
        if let Some(ref ret) = m.ret {
            refs.find_type_ref(ret, &marshal, true);
        }
    }
    for c in &i.consts {
        refs.find_type_ref(&c.ty, &marshal, true);
    }

    let self_name = marshal.typename_from_name(&ident.name, &TypeDef::Interface(i.clone()));
    let method_names: Vec<String> = i
        .methods
        .iter()
        .map(|m| (spec.cpp_ident_style.method)(&m.ident.name))
        .collect();
    let id_cpp = &spec.cpp_ident_style;

    // HPP
    let hpp_includes: Vec<String> = refs.hpp.iter().cloned().collect();
    let hpp_fwds: Vec<String> = refs.hpp_fwds.iter().cloned().collect();
    let file_ident = (spec.cpp_file_ident_style)(&ident.name);
    let hpp_name = format!("{}.{}", file_ident, spec.cpp_header_ext);
    let cpp_namespace = spec.cpp_namespace.clone();

    create_file_from_parts(out_files, written_files, hpp_folder, &hpp_name, |w| {
        w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!");
        w.wl(&format!("// This file was generated by Djinni from {}", origin));
        w.wl_empty();
        w.wl("#pragma once");
        if !hpp_includes.is_empty() {
            w.wl_empty();
            for inc in &hpp_includes {
                w.wl(inc);
            }
        }
        w.wl_empty();
        wrap_namespace(w, &cpp_namespace, |w| {
            if !hpp_fwds.is_empty() {
                for fwd in &hpp_fwds {
                    w.wl(fwd);
                }
                w.wl_empty();
            }

            write_doc(w, doc);
            write_cpp_type_params(w, params, id_cpp);
            w.w(&format!("class {}", self_name));
            w.braced_semi(|w| {
                w.wl_outdent("public:");
                w.wl(&format!("virtual ~{}() = default;", self_name));

                // Constants
                generate_hpp_constants(w, &i.consts, &marshal, id_cpp);

                // Methods
                for m in &i.methods {
                    w.wl_empty();
                    write_method_doc(w, &m.doc, &m.params, id_cpp.local);
                    let ret = marshal.return_type_scoped(m.ret.as_ref(), &method_names);
                    let params_str: Vec<String> = m
                        .params
                        .iter()
                        .map(|p| {
                            format!(
                                "{} {}",
                                marshal.param_type_from_typeref(&p.ty, &method_names),
                                (id_cpp.local)(&p.ident.name)
                            )
                        })
                        .collect();
                    let params_joined = params_str.join(", ");
                    if m.is_static {
                        w.wl(&format!(
                            "static {} {}({});",
                            ret,
                            (id_cpp.method)(&m.ident.name),
                            params_joined
                        ));
                    } else {
                        let const_flag = if m.is_const { " const" } else { "" };
                        w.wl(&format!(
                            "virtual {} {}({}){} = 0;",
                            ret,
                            (id_cpp.method)(&m.ident.name),
                            params_joined,
                            const_flag
                        ));
                    }
                }
            });
        });
    });

    // CPP (only if constants exist)
    if !i.consts.is_empty() {
        let cpp_includes: Vec<String> = refs.cpp.iter().cloned().collect();
        let cpp_name_file = format!("{}.{}", file_ident, spec.cpp_ext);
        let my_header = format!(
            "\"{}{}\"",
            spec.cpp_include_prefix,
            format!("{}.{}", file_ident, spec.cpp_header_ext)
        );
        let my_header_include = format!("#include {}", my_header);

        create_file_from_parts(out_files, written_files, cpp_folder, &cpp_name_file, |w| {
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
            wrap_namespace(w, &cpp_namespace, |w| {
                generate_cpp_constants(w, &i.consts, &self_name, &marshal, id_cpp);
            });
        });
    }
}

// --- Helper functions ---

fn write_cpp_type_params(
    w: &mut IndentWriter,
    params: &[TypeParam],
    id_cpp: &djinni_ast::ident_style::CppIdentStyle,
) {
    if params.is_empty() {
        return;
    }
    let parts: Vec<String> = params
        .iter()
        .map(|p| format!("typename {}", (id_cpp.type_param)(&p.ident.name)))
        .collect();
    w.wl(&format!("template <{}>", parts.join(", ")));
}

fn should_constexpr(c: &Const, marshal: &CppMarshal) -> bool {
    if let Some(ref resolved) = c.ty.resolved {
        matches!(&resolved.base, Meta::MPrimitive(_)) && !matches!(&resolved.base, Meta::MOptional)
    } else {
        false
    }
}

fn generate_hpp_constants(
    w: &mut IndentWriter,
    consts: &[Const],
    marshal: &CppMarshal,
    id_cpp: &djinni_ast::ident_style::CppIdentStyle,
) {
    for c in consts {
        let is_constexpr = should_constexpr(c, marshal);
        let const_value = if is_constexpr {
            match &c.value {
                ConstValue::Int(l) => format!(" = {};", l),
                ConstValue::Float(d) => {
                    let field_type = marshal.field_type_from_typeref(&c.ty, &[]);
                    if field_type == "float" {
                        format!(" = {}f;", fmt_float(*d))
                    } else {
                        format!(" = {};", fmt_float(*d))
                    }
                }
                ConstValue::Bool(b) => {
                    if *b {
                        " = true;".to_string()
                    } else {
                        " = false;".to_string()
                    }
                }
                _ => ";".to_string(),
            }
        } else {
            ";".to_string()
        };

        let field_type = marshal.field_type_from_typeref(&c.ty, &[]);
        let const_field_type = if is_constexpr {
            format!("constexpr {}", field_type)
        } else {
            format!("{} const", field_type)
        };

        w.wl_empty();
        write_doc(w, &c.doc);
        w.wl(&format!(
            "static {} {}{}",
            const_field_type,
            (id_cpp.const_)(&c.ident.name),
            const_value
        ));
    }
}

fn generate_cpp_constants(
    w: &mut IndentWriter,
    consts: &[Const],
    self_name: &str,
    marshal: &CppMarshal,
    id_cpp: &djinni_ast::ident_style::CppIdentStyle,
) {
    let mut first = true;
    for c in consts {
        if !should_constexpr(c, marshal) {
            if !first {
                w.wl_empty();
            }
            first = false;
            let field_type = marshal.field_type_from_typeref(&c.ty, &[]);
            w.w(&format!(
                "{} const {}::{} = ",
                field_type,
                self_name,
                (id_cpp.const_)(&c.ident.name)
            ));
            write_cpp_const_value(w, &c.ty, &c.value, marshal, id_cpp, self_name);
            w.wl(";");
        }
    }
}

fn write_cpp_const_value(
    w: &mut IndentWriter,
    ty: &TypeRef,
    value: &ConstValue,
    marshal: &CppMarshal,
    id_cpp: &djinni_ast::ident_style::CppIdentStyle,
    self_name: &str,
) {
    match value {
        ConstValue::Int(l) => {
            w.w(&l.to_string());
        }
        ConstValue::Float(d) => {
            let field_type = marshal.field_type_from_typeref(ty, &[]);
            if field_type == "float" {
                w.w(&format!("{}f", fmt_float(*d)));
            } else {
                w.w(&fmt_float(*d));
            }
        }
        ConstValue::Bool(b) => {
            w.w(if *b { "true" } else { "false" });
        }
        ConstValue::String(s) => {
            w.w(&format!("{{\"{}\"}}",s));
        }
        ConstValue::EnumValue { ty: enum_ty, value: val } => {
            if enum_ty.is_empty() {
                // ConstRef
                w.w(&format!("{}::{}", self_name, (id_cpp.const_)(val)));
            } else {
                // Actual enum value
                let type_name = marshal.field_type_from_typeref(ty, &[]);
                w.w(&format!("{}::{}", type_name, (id_cpp.enum_)(val)));
            }
        }
        ConstValue::Composite(fields) => {
            if let Some(ref resolved) = ty.resolved {
                if let Meta::MDef(d) = &resolved.base {
                    if let TypeDef::Record(rec) = &d.body {
                        let type_name = marshal.field_type_from_typeref(ty, &[]);
                        w.wl(&format!("{}(", type_name));
                        w.increase();
                        let mut first = true;
                        for f in &rec.fields {
                            if !first {
                                w.wl(",");
                            }
                            first = false;
                            if let Some((_, val)) = fields.iter().find(|(k, _)| *k == f.ident.name) {
                                write_cpp_const_value(w, &f.ty, val, marshal, id_cpp, self_name);
                                w.w(&format!(" /* {} */ ", (id_cpp.field)(&f.ident.name)));
                            }
                        }
                        w.w(")");
                        w.decrease();
                    }
                }
            }
        }
    }
}

fn is_ptr_type(base: &Meta) -> bool {
    if let Meta::MExtern(e) = base {
        e.cpp.typename.contains("shared_ptr") && !e.cpp.typename.contains("vector")
    } else {
        false
    }
}

fn is_list_of_ptr_type(base: &Meta) -> bool {
    if let Meta::MExtern(e) = base {
        e.cpp.typename.contains("shared_ptr") && e.cpp.typename.contains("vector")
    } else {
        false
    }
}

fn is_list_field(f: &Field) -> bool {
    if let Some(ref resolved) = f.ty.resolved {
        matches!(&resolved.base, Meta::MList) || is_list_of_ptr_type(&resolved.base)
    } else {
        false
    }
}

fn write_cpp_get_test_representation(
    w: &mut IndentWriter,
    actual_self: &str,
    fields: &[Field],
    own_fields: &[Field],
    super_record: &Option<SuperRecord>,
    doc: &Doc,
    marshal: &CppMarshal,
    id_cpp: &djinni_ast::ident_style::CppIdentStyle,
) {
    let is_inline = doc.lines.iter().any(|l| l.contains("@test-representation-inline"));

    if fields.is_empty() {
        return;
    }

    w.wl_empty();
    w.w(&format!(
        "std::string {}::getTestRepresentation(const std::string& textIndentation) const",
        actual_self
    ));
    w.braced(|w| {
        w.w("if constexpr (BuildConstants::UnitTests)");
        w.braced(|w| {
            w.wl("std::ostringstream ss;");
            w.wl("auto childIndentation = textIndentation + \"   \";");
            w.wl(&format!("ss << \"{} {{\";", actual_self));

            let mut is_first_output = true;

            // Parent getTestRepresentation
            if let Some(sr) = super_record {
                let parent_name = marshal.typename_from_name(
                    &sr.ident.name,
                    &TypeDef::Record(sr.record.clone()),
                );
                w.wl_empty();
                if is_inline {
                    w.wl(&format!(
                        "ss << {}::getTestRepresentation(textIndentation);",
                        parent_name
                    ));
                } else {
                    w.wl("ss << \"\\n\" << childIndentation;");
                    w.wl(&format!(
                        "ss << {}::getTestRepresentation(childIndentation);",
                        parent_name
                    ));
                }
                is_first_output = false;
            }

            // Separate list and non-list fields
            let (list_fields, non_list_fields): (Vec<&Field>, Vec<&Field>) =
                own_fields.iter().partition(|f| is_list_field(f));

            for f in &non_list_fields {
                output_field(w, f, is_first_output, is_inline, id_cpp, marshal);
                is_first_output = false;
            }

            if !is_inline && !list_fields.is_empty() && !non_list_fields.is_empty() {
                w.wl_empty();
                w.wl("ss << \"\\n\";");
            }

            for f in &list_fields {
                output_field(w, f, is_first_output, is_inline, id_cpp, marshal);
                is_first_output = false;
            }

            w.wl_empty();
            if is_inline {
                w.wl("ss << \"}\";");
            } else {
                w.wl("ss << \"\\n\" << textIndentation << \"}\";");
            }
            w.wl("return ss.str();");
        });
        w.wl("return \"\";");
    });
}

fn output_field(
    w: &mut IndentWriter,
    f: &Field,
    is_first: bool,
    is_inline: bool,
    id_cpp: &djinni_ast::ident_style::CppIdentStyle,
    marshal: &CppMarshal,
) {
    let name = (id_cpp.field)(&f.ident.name);
    let resolved = f.ty.resolved.as_ref().unwrap();
    let is_optional = matches!(&resolved.base, Meta::MOptional);
    let is_list = matches!(&resolved.base, Meta::MList);

    let base_type_name = match &resolved.base {
        Meta::MDef(d) => d.name.clone(),
        Meta::MExtern(e) => e.name.clone(),
        _ => marshal.field_type_from_typeref(&f.ty, &[]),
    };

    let is_ptr = is_ptr_type(&resolved.base);
    let is_list_of_ptr = is_list_of_ptr_type(&resolved.base);
    let is_smart_string = base_type_name == "SmartString";

    let inner_type = if (is_optional || is_list) && !resolved.args.is_empty() {
        &resolved.args[0].base
    } else {
        &resolved.base
    };

    let inner_type_name = match inner_type {
        Meta::MDef(d) => d.name.clone(),
        Meta::MExtern(e) => e.name.clone(),
        _ => marshal.field_type_from_typeref(&f.ty, &[]),
    };

    let is_inner_ptr = is_ptr_type(inner_type);
    let is_inner_smart_string = inner_type_name == "SmartString";
    let is_inner_enum = match inner_type {
        Meta::MDef(d) => d.def_type == DefType::Enum,
        Meta::MExtern(e) => e.def_type == DefType::Enum,
        _ => false,
    };
    let is_inner_record = match inner_type {
        Meta::MDef(d) => d.def_type == DefType::Record,
        Meta::MExtern(e) => e.def_type == DefType::Record,
        _ => false,
    };

    w.wl_empty();
    let inline_prefix = if is_inline && !is_first { ", " } else { "" };
    if !is_inline {
        w.wl("ss << \"\\n\" << childIndentation;");
    }

    if is_optional {
        w.w(&format!("if ({})", name));
        w.braced(|w| {
            let value_expr = if is_inner_enum {
                format!("to_string(*{})", name)
            } else if is_inner_smart_string {
                format!("{}->value", name)
            } else if is_inner_ptr {
                format!("(*{})->getTestRepresentation(childIndentation)", name)
            } else if is_inner_record {
                format!("{}->getTestRepresentation(childIndentation)", name)
            } else {
                format!("*{}", name)
            };
            w.wl(&format!(
                "ss << \"{}{}=\" << {};",
                inline_prefix, name, value_expr
            ));
        });
        w.w("else");
        w.braced(|w| {
            w.wl(&format!(
                "ss << \"{}{}=<none>\";",
                inline_prefix, name
            ));
        });
    } else if is_list {
        w.wl(&format!("ss << \"{}{}=[\";", inline_prefix, name));
        w.w(&format!("for (size_t i = 0; i < {}.size(); ++i)", name));
        w.braced(|w| {
            w.wl("if (i > 0) { ss << \",\"; }");
            if !is_inline {
                w.wl("ss << \"\\n\" << childIndentation << \"   \";");
            }
            let item_expr = if is_inner_enum {
                format!("to_string({}[i])", name)
            } else if is_inner_smart_string {
                format!("{}[i].value", name)
            } else if is_inner_ptr {
                format!(
                    "{}[i]->getTestRepresentation(childIndentation + \"   \")",
                    name
                )
            } else if is_inner_record {
                format!(
                    "{}[i].getTestRepresentation(childIndentation + \"   \")",
                    name
                )
            } else {
                format!("{}[i]", name)
            };
            w.wl(&format!("ss << {};", item_expr));
        });
        if !is_inline {
            w.w(&format!("if (!{}.empty())", name));
            w.braced(|w| {
                w.wl("ss << \"\\n\" << childIndentation;");
            });
        }
        w.wl("ss << \"]\";");
    } else if is_inner_enum {
        w.wl(&format!(
            "ss << \"{}{}=\" << to_string({});",
            inline_prefix, name, name
        ));
    } else if is_smart_string {
        w.wl(&format!(
            "ss << \"{}{}=\" << {}.value;",
            inline_prefix, name, name
        ));
    } else if is_ptr {
        w.wl(&format!(
            "ss << \"{}{}=\" << {}->getTestRepresentation(childIndentation);",
            inline_prefix, name, name
        ));
    } else if is_list_of_ptr {
        w.wl(&format!("ss << \"{}{}=[\";", inline_prefix, name));
        w.w(&format!("for (size_t i = 0; i < {}.size(); ++i)", name));
        w.braced(|w| {
            w.wl("if (i > 0) { ss << \",\"; }");
            if !is_inline {
                w.wl("ss << \"\\n\" << childIndentation << \"   \";");
            }
            w.wl(&format!(
                "ss << {}[i]->getTestRepresentation(childIndentation + \"   \");",
                name
            ));
        });
        if !is_inline {
            w.w(&format!("if (!{}.empty())", name));
            w.braced(|w| {
                w.wl("ss << \"\\n\" << childIndentation;");
            });
        }
        w.wl("ss << \"]\";");
    } else if is_inner_record {
        w.wl(&format!(
            "ss << \"{}{}=\" << {}.getTestRepresentation(childIndentation);",
            inline_prefix, name, name
        ));
    } else {
        w.wl(&format!(
            "ss << \"{}{}=\" << {};",
            inline_prefix, name, name
        ));
    }
}
