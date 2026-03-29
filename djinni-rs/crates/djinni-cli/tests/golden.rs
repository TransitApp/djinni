// vinsg
// Golden file comparison tests for the Djinni code generator.
//
// These tests run the Rust djinni binary with the same arguments as
// test-suite/run_djinni.sh, then compare the generated output against
// the committed golden files in test-suite/generated-src/.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn repo_root() -> PathBuf {
    // djinni-rs/crates/djinni-cli/tests/golden.rs -> djinni-rs -> repo root
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent() // crates
        .unwrap()
        .parent() // djinni-rs
        .unwrap()
        .parent() // repo root
        .unwrap()
        .to_path_buf()
}

fn djinni_binary() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path = path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join("debug")
        .join("djinni");
    path
}

/// Read all files in a directory recursively into a BTreeMap of relative_path -> content.
fn read_dir_recursive(dir: &Path) -> BTreeMap<String, String> {
    let mut files = BTreeMap::new();
    if !dir.exists() {
        return files;
    }
    for entry in WalkDir::new(dir).sort_by_file_name() {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(dir).unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(entry.path()).unwrap_or_else(|_| {
                format!("<binary file: {}>", rel)
            });
            files.insert(rel, content);
        }
    }
    files
}

/// Compare two directory trees and return a diff report.
/// Returns None if identical, Some(report) if different.
fn compare_dirs(expected_dir: &Path, actual_dir: &Path) -> Option<String> {
    let expected = read_dir_recursive(expected_dir);
    let actual = read_dir_recursive(actual_dir);

    let mut diffs = Vec::new();

    // Files in expected but not in actual
    for key in expected.keys() {
        if !actual.contains_key(key) {
            diffs.push(format!("MISSING in actual: {}", key));
        }
    }

    // Files in actual but not in expected
    for key in actual.keys() {
        if !expected.contains_key(key) {
            diffs.push(format!("EXTRA in actual: {}", key));
        }
    }

    // Files that differ
    for (key, expected_content) in &expected {
        if let Some(actual_content) = actual.get(key) {
            if expected_content != actual_content {
                let diff = similar::TextDiff::from_lines(expected_content, actual_content);
                let unified = diff
                    .unified_diff()
                    .context_radius(3)
                    .header(&format!("expected/{}", key), &format!("actual/{}", key))
                    .to_string();
                diffs.push(format!("DIFF in {}:\n{}", key, unified));
            }
        }
    }

    if diffs.is_empty() {
        None
    } else {
        Some(diffs.join("\n\n"))
    }
}

/// Run the djinni binary with the given arguments, in the given working directory.
fn run_djinni(working_dir: &Path, args: &[&str]) -> Result<(), String> {
    let binary = djinni_binary();
    if !binary.exists() {
        return Err(format!(
            "Djinni binary not found at {:?}. Run `cargo build` first.",
            binary
        ));
    }

    let output = std::process::Command::new(&binary)
        .current_dir(working_dir)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run djinni: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Djinni exited with status {}.\nstdout: {}\nstderr: {}",
            output.status, stdout, stderr
        ));
    }

    Ok(())
}

/// Main golden test: runs the generator on test-suite IDL files and compares output.
///
/// This mirrors the invocations in test-suite/run_djinni.sh.
#[test]
fn golden_test_suite_main() {
    let root = repo_root();
    let test_suite = root.join("test-suite");
    let temp = tempfile::tempdir().unwrap();
    let temp_out = temp.path();

    // Invocation 1: wchar_test.djinni
    run_djinni(
        &test_suite,
        &[
            "--java-out", &temp_out.join("java").to_string_lossy(),
            "--java-package", "com.dropbox.djinni.test",
            "--java-nullable-annotation", "javax.annotation.CheckForNull",
            "--java-nonnull-annotation", "javax.annotation.Nonnull",
            "--java-use-final-for-record", "false",
            "--ident-java-field", "mFooBar",
            "--kotlin-out", &temp_out.join("kotlin").to_string_lossy(),
            "--cpp-out", &temp_out.join("cpp").to_string_lossy(),
            "--cpp-namespace", "testsuite",
            "--ident-cpp-enum-type", "foo_bar",
            "--cpp-optional-template", "std::experimental::optional",
            "--cpp-optional-header", "\"../../handwritten-src/cpp/optional.hpp\"",
            "--cpp-extended-record-include-prefix", "../../handwritten-src/cpp/",
            "--cpp-use-wide-strings", "true",
            "--jni-out", &temp_out.join("jni").to_string_lossy(),
            "--ident-jni-class", "NativeFooBar",
            "--ident-jni-file", "NativeFooBar",
            "--objc-out", &temp_out.join("objc").to_string_lossy(),
            "--objcpp-out", &temp_out.join("objc").to_string_lossy(),
            "--objc-type-prefix", "DB",
            "--wasm-out", &temp_out.join("wasm").to_string_lossy(),
            "--wasm-namespace", "testsuite",
            "--ts-out", &temp_out.join("ts").to_string_lossy(),
            "--ts-module", "test_wchar",
            "--yaml-out", &temp_out.join("yaml").to_string_lossy(),
            "--yaml-out-file", "yaml-test.yaml",
            "--yaml-prefix", "test_",
            "--idl", "djinni/wchar_test.djinni",
        ],
    )
    .unwrap();

    // Invocation 2: all.djinni (the main test)
    run_djinni(
        &test_suite,
        &[
            "--java-out", &temp_out.join("java").to_string_lossy(),
            "--java-package", "com.dropbox.djinni.test",
            "--java-nullable-annotation", "javax.annotation.CheckForNull",
            "--java-nonnull-annotation", "javax.annotation.Nonnull",
            "--java-use-final-for-record", "false",
            "--java-implement-android-os-parcelable", "true",
            "--ident-java-field", "mFooBar",
            "--kotlin-out", &temp_out.join("kotlin").to_string_lossy(),
            "--cpp-out", &temp_out.join("cpp").to_string_lossy(),
            "--cpp-namespace", "testsuite",
            "--ident-cpp-enum-type", "foo_bar",
            "--cpp-optional-template", "std::experimental::optional",
            "--cpp-optional-header", "\"../../handwritten-src/cpp/optional.hpp\"",
            "--cpp-extended-record-include-prefix", "../../handwritten-src/cpp/",
            "--jni-out", &temp_out.join("jni").to_string_lossy(),
            "--jni-use-on-load-initializer", "false",
            "--ident-jni-class", "NativeFooBar",
            "--ident-jni-file", "NativeFooBar",
            "--objc-out", &temp_out.join("objc").to_string_lossy(),
            "--objcpp-out", &temp_out.join("objc").to_string_lossy(),
            "--objc-type-prefix", "DB",
            "--wasm-out", &temp_out.join("wasm").to_string_lossy(),
            "--wasm-namespace", "testsuite",
            "--ts-out", &temp_out.join("ts").to_string_lossy(),
            "--ts-module", "test",
            "--yaml-out", &temp_out.join("yaml").to_string_lossy(),
            "--yaml-out-file", "yaml-test.yaml",
            "--yaml-prefix", "test_",
            "--idl", "djinni/all.djinni",
            "--idl-include-path", "djinni/vendor",
        ],
    )
    .unwrap();

    // Invocation 3: function_prologue.djinni
    run_djinni(
        &test_suite,
        &[
            "--java-out", &temp_out.join("java").to_string_lossy(),
            "--java-package", "com.dropbox.djinni.test",
            "--java-nullable-annotation", "javax.annotation.CheckForNull",
            "--java-nonnull-annotation", "javax.annotation.Nonnull",
            "--java-use-final-for-record", "false",
            "--ident-java-field", "mFooBar",
            "--kotlin-out", &temp_out.join("kotlin").to_string_lossy(),
            "--cpp-out", &temp_out.join("cpp").to_string_lossy(),
            "--cpp-namespace", "testsuite",
            "--ident-cpp-enum-type", "foo_bar",
            "--cpp-optional-template", "std::experimental::optional",
            "--cpp-optional-header", "\"../../handwritten-src/cpp/optional.hpp\"",
            "--cpp-extended-record-include-prefix", "../../handwritten-src/cpp/",
            "--jni-out", &temp_out.join("jni").to_string_lossy(),
            "--ident-jni-class", "NativeFooBar",
            "--ident-jni-file", "NativeFooBar",
            "--jni-function-prologue-file", "../../handwritten-src/cpp/jni_prologue.hpp",
            "--objc-out", &temp_out.join("objc").to_string_lossy(),
            "--objcpp-out", &temp_out.join("objc").to_string_lossy(),
            "--objc-type-prefix", "DB",
            "--objcpp-function-prologue-file", "../../handwritten-src/cpp/objcpp-prologue.hpp",
            "--idl", "djinni/function_prologue.djinni",
        ],
    )
    .unwrap();

    // Invocation 4: ident_explicit.djinni (with strict ident styles)
    run_djinni(
        &test_suite,
        &[
            "--java-out", &temp_out.join("java").to_string_lossy(),
            "--java-package", "com.dropbox.djinni.test",
            "--java-nullable-annotation", "javax.annotation.CheckForNull",
            "--java-nonnull-annotation", "javax.annotation.Nonnull",
            "--java-use-final-for-record", "false",
            "--ident-java-type", "FooBar!Native",
            "--ident-java-field", "mFooBar!",
            "--kotlin-out", &temp_out.join("kotlin").to_string_lossy(),
            "--cpp-out", &temp_out.join("cpp").to_string_lossy(),
            "--cpp-namespace", "testsuite",
            "--ident-cpp-file", "foo_bar!_native",
            "--ident-cpp-enum-type", "foo_bar!",
            "--cpp-optional-template", "std::experimental::optional",
            "--cpp-optional-header", "\"../../handwritten-src/cpp/optional.hpp\"",
            "--cpp-extended-record-include-prefix", "../../handwritten-src/cpp/",
            "--jni-out", &temp_out.join("jni").to_string_lossy(),
            "--ident-jni-file", "FooBar!Native",
            "--ident-jni-class", "FooBar!Native",
            "--jni-function-prologue-file", "../../handwritten-src/cpp/jni_prologue.hpp",
            "--objc-out", &temp_out.join("objc").to_string_lossy(),
            "--objcpp-out", &temp_out.join("objc").to_string_lossy(),
            "--objc-type-prefix", "DB",
            "--objcpp-function-prologue-file", "../../handwritten-src/cpp/objcpp-prologue.hpp",
            "--ident-objc-type", "FooBar!",
            "--ident-objc-enum", "FooBar!Native",
            "--ident-objc-const", "FooBar!Native",
            "--idl", "djinni/ident_explicit.djinni",
        ],
    )
    .unwrap();

    // Invocation 5: interface_and_abstract_class.djinni
    run_djinni(
        &test_suite,
        &[
            "--java-out", &temp_out.join("java").to_string_lossy(),
            "--java-package", "com.dropbox.djinni.test",
            "--java-nullable-annotation", "javax.annotation.CheckForNull",
            "--java-nonnull-annotation", "javax.annotation.Nonnull",
            "--java-use-final-for-record", "false",
            "--java-implement-android-os-parcelable", "true",
            "--java-gen-interface", "true",
            "--ident-java-field", "mFooBar",
            "--kotlin-out", &temp_out.join("kotlin").to_string_lossy(),
            "--cpp-out", &temp_out.join("cpp").to_string_lossy(),
            "--cpp-namespace", "testsuite",
            "--ident-cpp-enum-type", "foo_bar",
            "--cpp-optional-template", "std::experimental::optional",
            "--cpp-optional-header", "\"../../handwritten-src/cpp/optional.hpp\"",
            "--cpp-extended-record-include-prefix", "../../handwritten-src/cpp/",
            "--jni-out", &temp_out.join("jni").to_string_lossy(),
            "--jni-use-on-load-initializer", "false",
            "--ident-jni-class", "NativeFooBar",
            "--ident-jni-file", "NativeFooBar",
            "--objc-out", &temp_out.join("objc").to_string_lossy(),
            "--objcpp-out", &temp_out.join("objc").to_string_lossy(),
            "--objc-type-prefix", "DB",
            "--yaml-out", &temp_out.join("yaml").to_string_lossy(),
            "--yaml-out-file", "yaml-interface-test.yaml",
            "--yaml-prefix", "test_",
            "--idl", "djinni/interface_and_abstract_class.djinni",
            "--idl-include-path", "djinni/vendor",
        ],
    )
    .unwrap();

    // Invocation 6: no_constructor.djinni (with no struct constructor)
    run_djinni(
        &test_suite,
        &[
            "--java-out", &temp_out.join("java").to_string_lossy(),
            "--java-package", "com.dropbox.djinni.test",
            "--java-nullable-annotation", "javax.annotation.CheckForNull",
            "--java-nonnull-annotation", "javax.annotation.Nonnull",
            "--java-use-final-for-record", "false",
            "--ident-java-type", "FooBarNative",
            "--ident-java-field", "mFooBar",
            "--kotlin-out", &temp_out.join("kotlin").to_string_lossy(),
            "--cpp-out", &temp_out.join("cpp").to_string_lossy(),
            "--cpp-namespace", "testsuite",
            "--cpp-struct-constructor", "false",
            "--ident-cpp-file", "foo_bar_native",
            "--ident-cpp-enum-type", "foo_bar",
            "--cpp-optional-template", "std::experimental::optional",
            "--cpp-optional-header", "\"../../handwritten-src/cpp/optional.hpp\"",
            "--cpp-extended-record-include-prefix", "../../handwritten-src/cpp/",
            "--jni-out", &temp_out.join("jni").to_string_lossy(),
            "--ident-jni-file", "FooBarNative",
            "--ident-jni-class", "FooBarNative",
            "--jni-function-prologue-file", "../../handwritten-src/cpp/jni_prologue.hpp",
            "--objc-out", &temp_out.join("objc").to_string_lossy(),
            "--objcpp-out", &temp_out.join("objc").to_string_lossy(),
            "--objc-type-prefix", "DB",
            "--objcpp-function-prologue-file", "../../handwritten-src/cpp/objcpp-prologue.hpp",
            "--ident-objc-type", "FooBar",
            "--ident-objc-enum", "FooBarNative",
            "--ident-objc-const", "FooBarNative",
            "--idl", "djinni/no_constructor.djinni",
        ],
    )
    .unwrap();

    // Compare each output subdirectory against golden files
    let generated = test_suite.join("generated-src");
    let subdirs = &["cpp", "jni", "objc", "wasm", "ts"];

    for subdir in subdirs {
        let expected = generated.join(subdir);
        let actual = temp_out.join(subdir);
        if let Some(report) = compare_dirs(&expected, &actual) {
            panic!(
                "Golden file mismatch in '{}':\n\n{}",
                subdir, report
            );
        }
    }

    // Java golden files are nested under java/com/dropbox/djinni/test
    let java_expected = generated.join("java").join("com").join("dropbox").join("djinni").join("test");
    let java_actual = temp_out.join("java");
    if let Some(report) = compare_dirs(&java_expected, &java_actual) {
        panic!("Golden file mismatch in 'java':\n\n{}", report);
    }

    // Kotlin golden files
    let kotlin_expected = generated.join("kotlin").join("com").join("dropbox").join("djinni").join("test");
    let kotlin_actual = temp_out.join("kotlin");
    if let Some(report) = compare_dirs(&kotlin_expected, &kotlin_actual) {
        panic!("Golden file mismatch in 'kotlin':\n\n{}", report);
    }
}

/// Test that compares a single subdirectory (useful for incremental development).
/// Run with: cargo test -- golden_cpp_only
#[test]
fn golden_cpp_only() {
    let root = repo_root();
    let test_suite = root.join("test-suite");
    let temp = tempfile::tempdir().unwrap();
    let temp_out = temp.path();

    // Run just the main all.djinni with C++ output only
    run_djinni(
        &test_suite,
        &[
            "--cpp-out", &temp_out.join("cpp").to_string_lossy(),
            "--cpp-namespace", "testsuite",
            "--ident-cpp-enum-type", "foo_bar",
            "--cpp-optional-template", "std::experimental::optional",
            "--cpp-optional-header", "\"../../handwritten-src/cpp/optional.hpp\"",
            "--cpp-extended-record-include-prefix", "../../handwritten-src/cpp/",
            "--idl", "djinni/all.djinni",
            "--idl-include-path", "djinni/vendor",
        ],
    )
    .unwrap();

    let expected = test_suite.join("generated-src").join("cpp");
    let actual = temp_out.join("cpp");
    if let Some(report) = compare_dirs(&expected, &actual) {
        panic!("Golden file mismatch in 'cpp':\n\n{}", report);
    }
}
