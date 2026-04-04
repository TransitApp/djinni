# djinni-rs

A Rust implementation of the [Djinni](https://github.com/nicklockwood/djinni) cross-platform code generator. Translates Djinni IDL interface definitions into type-safe bindings for **9 language targets** -- replacing the original Scala implementation with a fast, single-binary tool.

```
djinni IDL ──> djinni-rs ──> C++, Java, Kotlin, JNI, Objective-C,
                              Objective-C++, WASM, TypeScript, YAML
```

## Why Rust?

The original Djinni generator requires a JVM + SBT toolchain, takes seconds to start, and is difficult to distribute. `djinni-rs` compiles to a single native binary that runs the full test suite -- 7 invocations generating 771 files across 9 targets -- in **~150ms**.

## Quick start

```sh
# Build
cargo build --release

# Generate bindings from an IDL file
./target/release/djinni \
    --idl my_types.djinni \
    --cpp-out gen/cpp \
    --java-out gen/java --java-package com.example \
    --kotlin-out gen/kotlin \
    --jni-out gen/jni \
    --objc-out gen/objc --objcpp-out gen/objc \
    --ts-out gen/ts --ts-module my_types \
    --wasm-out gen/wasm --wasm-namespace my_types
```

## Project structure

```
djinni-rs/
├── crates/
│   ├── djinni-ast/           # AST types, meta type system, identifier styles
│   ├── djinni-parser/        # PEG-based IDL parser (pest)
│   ├── djinni-resolver/      # Type resolution and validation
│   ├── djinni-generator/     # All code generators and type marshals
│   │   └── src/
│   │       ├── cpp_gen.rs        # C++ headers and implementation
│   │       ├── cpp_marshal.rs    # C++ type marshaling
│   │       ├── java_gen.rs       # Java classes with Parcelable support
│   │       ├── java_marshal.rs   # Java type marshaling
│   │       ├── kotlin_gen.rs     # Kotlin data classes and interfaces
│   │       ├── jni_gen.rs        # JNI bridge (C++ <-> Java/Kotlin)
│   │       ├── jni_marshal.rs    # JNI type marshaling
│   │       ├── objc_gen.rs       # Objective-C headers and implementation
│   │       ├── objc_marshal.rs   # Objective-C type marshaling
│   │       ├── objcpp_gen.rs     # Objective-C++ bridge (C++ <-> ObjC)
│   │       ├── objcpp_marshal.rs # Objective-C++ type marshaling
│   │       ├── wasm_gen.rs       # WASM/Emscripten bindings
│   │       ├── ts_gen.rs         # TypeScript type definitions
│   │       ├── yaml_gen.rs       # YAML metadata for extern type round-trips
│   │       ├── gen.rs            # Shared generator utilities
│   │       └── writer.rs         # Indented code writer
│   └── djinni-cli/           # CLI entry point and golden tests
│       ├── src/main.rs
│       ├── tests/golden.rs   # Golden file comparison tests
│       └── benches/          # Criterion benchmarks
└── Cargo.toml                # Workspace definition
```

## Building

```sh
# Debug build
cargo build

# Release build (recommended for actual use)
cargo build --release

# The binary is at target/release/djinni (or target/debug/djinni)
```

**Requirements:** Rust 1.70+ (uses 2021 edition)

## Testing

The test suite compares generated output against committed golden files from the original Scala generator, ensuring byte-for-byte compatibility across all 9 targets.

```sh
# Run all tests
cargo test

# Run only the golden file comparison tests
cargo test -p djinni-cli golden

# Run a specific golden test
cargo test -p djinni-cli golden_test_suite_main
```

### Golden test status

| Target        | Status | Files |
|---------------|--------|-------|
| C++           | PASS   | 106 files (.hpp/.cpp) |
| JNI           | PASS   | 110 files (.hpp/.cpp) |
| Objective-C   | PASS   | 156 files (.h/.m/.mm) |
| Java          | PASS   | 62 files (.java) |
| Kotlin        | PASS   | 62 files (.kt) |
| TypeScript    | PASS   | 3 files (.ts) |
| WASM          | PASS   | 126 files (.hpp/.cpp) |
| YAML          | PASS   | round-trip verified |

## Benchmarks

```sh
# Build release first (benchmarks use the release binary)
cargo build --release

# Run benchmarks
cargo bench -p djinni-cli
```

### Results (Apple M1 Pro)

| Benchmark | Time | Description |
|-----------|------|-------------|
| `full_suite_7_invocations` | **~150ms** | All 7 test invocations, all 9 targets, 771 output files |
| `single_invocation/all_targets` | **~110ms** | `all.djinni` (30+ types), all 9 targets |
| `single_invocation/cpp_only` | **~27ms** | `all.djinni`, C++ only |

Each invocation includes process startup, IDL parsing, type resolution, and file I/O. Actual generation time per invocation is ~15-90ms depending on target count.

## CLI reference

### Input

| Flag | Description |
|------|-------------|
| `--idl <path>` | IDL file to process (repeatable) |
| `--idl-include-path <path>` | Search path for `@import` directives (repeatable) |

### C++ output

| Flag | Default | Description |
|------|---------|-------------|
| `--cpp-out <dir>` | | C++ output directory |
| `--cpp-header-out <dir>` | same as `--cpp-out` | Separate header output directory |
| `--cpp-namespace <ns>` | | C++ namespace |
| `--cpp-include-prefix <pfx>` | `""` | Prefix for `#include` paths |
| `--cpp-extended-record-include-prefix <pfx>` | `""` | Include prefix for extended records |
| `--cpp-optional-template <tpl>` | `std::optional` | Optional type template |
| `--cpp-optional-header <hdr>` | `<optional>` | Header for optional type |
| `--cpp-use-wide-strings <bool>` | `false` | Use `std::wstring` instead of `std::string` |
| `--cpp-struct-constructor <bool>` | `true` | Generate struct constructors |
| `--cpp-enum-hash-workaround <bool>` | `true` | Generate enum hash specialization |
| `--cpp-nn-header <hdr>` | | Non-nullable pointer header |
| `--cpp-nn-type <type>` | | Non-nullable pointer type |

### Java output

| Flag | Default | Description |
|------|---------|-------------|
| `--java-out <dir>` | | Java output directory |
| `--java-package <pkg>` | | Java package name |
| `--java-use-final-for-record <bool>` | `true` | Generate `final` records |
| `--java-implement-android-os-parcelable <bool>` | `false` | Generate Parcelable support |
| `--java-gen-interface <bool>` | `false` | Generate interfaces instead of abstract classes |
| `--java-nullable-annotation <class>` | | Nullable annotation class |
| `--java-nonnull-annotation <class>` | | NonNull annotation class |

### Kotlin output

| Flag | Description |
|------|-------------|
| `--kotlin-out <dir>` | Kotlin output directory |

### JNI output

| Flag | Default | Description |
|------|---------|-------------|
| `--jni-out <dir>` | | JNI C++ output directory |
| `--jni-header-out <dir>` | same as `--jni-out` | Separate JNI header directory |
| `--jni-use-on-load-initializer <bool>` | `false` | Use JNI_OnLoad for initialization |
| `--jni-function-prologue-file <path>` | | Header to include at top of JNI functions |

### Objective-C / Objective-C++ output

| Flag | Default | Description |
|------|---------|-------------|
| `--objc-out <dir>` | | Objective-C output directory |
| `--objcpp-out <dir>` | | Objective-C++ output directory |
| `--objc-type-prefix <pfx>` | `""` | Type name prefix (e.g., `DB`) |
| `--objc-gen-protocol <bool>` | `false` | Generate `@protocol` instead of `@interface` |
| `--objcpp-function-prologue-file <path>` | | Header to include at top of ObjC++ functions |

### WASM / TypeScript output

| Flag | Default | Description |
|------|---------|-------------|
| `--wasm-out <dir>` | | WASM C++ output directory |
| `--wasm-namespace <ns>` | | WASM/JS namespace |
| `--ts-out <dir>` | | TypeScript output directory |
| `--ts-module <name>` | `module` | TypeScript module name |

### YAML output

| Flag | Description |
|------|-------------|
| `--yaml-out <dir>` | YAML output directory |
| `--yaml-out-file <name>` | Single combined YAML file name |
| `--yaml-prefix <pfx>` | Type name prefix in YAML |

### Identifier styles

Override naming conventions per language with patterns like `mFooBar`, `foo_bar!`, `FooBar!Native`:

| Flag | Affects |
|------|---------|
| `--ident-cpp-enum-type`, `--ident-cpp-file`, `--ident-cpp-type`, ... | C++ identifiers |
| `--ident-java-field`, `--ident-java-type`, `--ident-java-enum` | Java identifiers |
| `--ident-jni-class`, `--ident-jni-file` | JNI identifiers |
| `--ident-objc-type`, `--ident-objc-enum`, `--ident-objc-const`, ... | Objective-C identifiers |

## Architecture

The pipeline follows four stages:

```
IDL files ──> Parser ──> Resolver ──> Generators ──> Output files
              (pest)    (2-pass)     (per-target)
```

1. **Parser** (`djinni-parser`): PEG grammar parses `.djinni` files into AST nodes. Handles imports, extern YAML types, protobuf definitions, and all Djinni IDL syntax.

2. **Resolver** (`djinni-resolver`): Two-pass resolution builds a global scope, resolves type references, validates generic arity, detects duplicates, and converts extern YAML properties into `MExtern` metadata.

3. **Generators** (`djinni-generator`): Each target has a generator module (`*_gen.rs`) and optionally a marshal module (`*_marshal.rs`). Generators produce the code structure; marshals handle type name mapping and conversion expressions.

4. **CLI** (`djinni-cli`): Parses 50+ command-line flags, builds the `Spec` configuration, orchestrates the pipeline, and writes file lists.

## License

Same license as the original Djinni project.
