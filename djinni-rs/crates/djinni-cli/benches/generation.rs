// vinsg
// Benchmark for the Djinni code generator.
//
// Measures end-to-end generation time using the test-suite IDL files.
// Run with: cargo bench -p djinni-cli
//
// Requires a release build of the djinni binary:
//   cargo build --release

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
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
        .join("release")
        .join("djinni");
    if !path.exists() {
        // Fall back to debug build
        path = path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("debug")
            .join("djinni");
    }
    path
}

fn run_djinni(working_dir: &Path, args: &[&str]) {
    let binary = djinni_binary();
    assert!(
        binary.exists(),
        "Djinni binary not found at {:?}. Run `cargo build --release` first.",
        binary
    );

    let output = std::process::Command::new(&binary)
        .current_dir(working_dir)
        .args(args)
        .stderr(std::process::Stdio::null())
        .output()
        .expect("Failed to run djinni");

    assert!(
        output.status.success(),
        "Djinni exited with status {}",
        output.status
    );
}

/// Runs all 7 invocations from run_djinni.sh (same as golden_test_suite_main)
fn run_all_invocations(test_suite: &Path, temp_out: &Path) {
    // Invocation 1: wchar_test.djinni
    run_djinni(
        test_suite,
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
    );

    // Invocation 2: all.djinni (the main test)
    run_djinni(
        test_suite,
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
    );

    // Invocation 3: function_prologue.djinni
    run_djinni(
        test_suite,
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
    );

    // Invocation 4: ident_explicit.djinni
    run_djinni(
        test_suite,
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
    );

    // Invocation 5: interface_and_abstract_class.djinni
    run_djinni(
        test_suite,
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
    );

    // Invocation 6: no_constructor.djinni
    run_djinni(
        test_suite,
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
    );

    // Invocation 7: YAML round-trip
    let yaml_dir = temp_out.join("yaml");
    fs::copy(
        test_suite.join("djinni").join("yaml-test.djinni"),
        yaml_dir.join("yaml-test.djinni"),
    )
    .unwrap();

    run_djinni(
        test_suite,
        &[
            "--java-out", &temp_out.join("java").to_string_lossy(),
            "--java-package", "com.dropbox.djinni.test",
            "--ident-java-field", "mFooBar",
            "--kotlin-out", &temp_out.join("kotlin").to_string_lossy(),
            "--cpp-out", &temp_out.join("cpp").to_string_lossy(),
            "--ident-cpp-enum-type", "foo_bar",
            "--cpp-optional-template", "std::experimental::optional",
            "--cpp-optional-header", "\"../../handwritten-src/cpp/optional.hpp\"",
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
            "--ts-module", "test_yaml",
            "--idl", &temp_out.join("yaml").join("yaml-test.djinni").to_string_lossy(),
        ],
    );
}

/// Single invocation: all.djinni with all targets (the heaviest invocation)
fn run_all_djinni_single(test_suite: &Path, temp_out: &Path) {
    run_djinni(
        test_suite,
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
    );
}

/// Single invocation: all.djinni with C++ only (measures single-target overhead)
fn run_cpp_only(test_suite: &Path, temp_out: &Path) {
    run_djinni(
        test_suite,
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
    );
}

fn bench_full_suite(c: &mut Criterion) {
    let root = repo_root();
    let test_suite = root.join("test-suite");

    c.bench_function("full_suite_7_invocations", |b| {
        b.iter(|| {
            let temp = tempfile::tempdir().unwrap();
            run_all_invocations(&test_suite, temp.path());
        })
    });
}

fn bench_single_invocation(c: &mut Criterion) {
    let root = repo_root();
    let test_suite = root.join("test-suite");

    let mut group = c.benchmark_group("single_invocation");

    group.bench_function("all_targets", |b| {
        b.iter(|| {
            let temp = tempfile::tempdir().unwrap();
            run_all_djinni_single(&test_suite, temp.path());
        })
    });

    group.bench_function("cpp_only", |b| {
        b.iter(|| {
            let temp = tempfile::tempdir().unwrap();
            run_cpp_only(&test_suite, temp.path());
        })
    });

    group.finish();
}

criterion_group!(benches, bench_full_suite, bench_single_invocation);
criterion_main!(benches);
