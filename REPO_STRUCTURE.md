# Djinni Repository Structure & Architecture

Djinni is a **cross-language bridging code generator**. It takes an Interface Definition Language (IDL) and generates type-safe bindings between C++ and multiple target languages (Java, Kotlin, Objective-C, TypeScript, WebAssembly). Originally by Dropbox, maintained by Snapchat — this fork adds Kotlin support and record inheritance.

---

## How It Works (Pipeline)

```
.djinni IDL files          .yaml external type configs
        \                       /
         ↓                     ↓
    ┌─────────────────────────────┐
    │  Parser (parser.scala)      │  Scala regex parser combinators
    │  Reads IDL + YAML configs   │
    └────────────┬────────────────┘
                 ↓
    ┌─────────────────────────────┐
    │  AST (ast.scala)            │  TypeDecl, Record, Interface, Enum
    │  + Metadata (meta.scala)    │  MExpr, MDef, MExtern, MOpaque
    └────────────┬────────────────┘
                 ↓
    ┌─────────────────────────────┐
    │  Resolver (resolver.scala)  │  Type checking, scope validation,
    │                             │  duplicate detection, const checking
    └────────────┬────────────────┘
                 ↓
    ┌─────────────────────────────┐
    │  Generators + Marshals      │  One pair per target language
    │  (e.g. CppGenerator.scala   │
    │   + CppMarshal.scala)       │
    └────────────┬────────────────┘
                 ↓
    ┌─────────────────────────────┐
    │  Writer (writer.scala)      │  File output with indentation,
    │                             │  includes, namespace management
    └────────────┬────────────────┘
                 ↓
    Generated source files (.cpp, .java, .kt, .h, .m, .mm, .ts)
```

---

## Directory Structure

```
djinni/
│
├── src/source/                    # ── MAIN SOURCE (Scala) ──────────────
│   ├── Main.scala                 # Entry point, CLI argument parsing (scopt)
│   ├── parser.scala               # IDL parser (regex parser combinators)
│   ├── ast.scala                  # AST node types (TypeDecl, Record, Interface, Enum)
│   ├── meta.scala                 # Type metadata (MExpr, MDef, MExtern, MOpaque)
│   ├── resolver.scala             # Type resolution, scope checking, validation
│   ├── generator.scala            # Abstract generator base + Spec config
│   ├── Marshal.scala              # Abstract marshaling interface
│   ├── writer.scala               # Output file writer
│   ├── syntax.scala               # Error reporting utilities
│   │
│   │  # ── Language-specific generators ──
│   ├── CppGenerator.scala         # C++ headers + implementations
│   ├── CppMarshal.scala           # C++ type marshaling (std::string, vector, etc.)
│   ├── JavaGenerator.scala        # Java classes
│   ├── JavaMarshal.scala          # Java type marshaling
│   ├── KotlinGenerator.scala      # Kotlin data classes + interfaces
│   ├── JNIGenerator.scala         # JNI glue code (C++ ↔ JVM)
│   ├── JNIMarshal.scala           # JNI type marshaling
│   ├── ObjcGenerator.scala        # Objective-C headers + implementations
│   ├── ObjcMarshal.scala          # Objective-C type marshaling
│   ├── ObjcppGenerator.scala      # Objective-C++ bridge layer
│   ├── ObjcppMarshal.scala        # Objective-C++ type marshaling
│   ├── BaseObjcGenerator.scala    # Shared Obj-C / Obj-C++ logic
│   ├── SwiftBridgingHeaderGenerator.scala  # Swift bridging header
│   ├── TsGenerator.scala          # TypeScript/JavaScript bindings
│   ├── WasmGenerator.scala        # WebAssembly (Emscripten) bindings
│   └── YamlGenerator.scala        # YAML config generator for external types
│
├── support-lib/                   # ── RUNTIME SUPPORT LIBRARIES ────────
│   ├── cpp/                       # C++ runtime (Future.hpp, DataRef.hpp, expected.hpp)
│   ├── java/                      # Java runtime utilities
│   ├── jni/                       # JNI support (djinni_support.hpp, Marshal.hpp,
│   │                              #   reference guards, exception translation)
│   ├── objc/                      # Obj-C runtime (DJIMarshal, DJFuture, DJIError,
│   │                              #   proxy cache management)
│   ├── ts/                        # TypeScript runtime
│   └── wasm/                      # WebAssembly runtime
│
├── test-suite/                    # ── TESTS ─────────────────────────────
│   ├── djinni/                    # 35+ .djinni IDL test files
│   │   ├── enum.djinni            #   enums, flags
│   │   ├── client_interface.djinni#   interfaces
│   │   ├── primitive_list.djinni  #   containers
│   │   ├── constants.djinni       #   constants
│   │   ├── extended_record.djinni #   record inheritance
│   │   └── ...
│   ├── generated-src/             # Generated output from test IDL files
│   └── handwritten-src/           # Hand-written C++/Java/ObjC test impls
│
├── examples/                      # ── EXAMPLES ──────────────────────────
│   ├── example.djinni             # Example IDL (with inheritance)
│   ├── example.yaml               # Example external type YAML config
│   ├── generated/                 # Generated C++, Java, Obj-C output
│   ├── handwritten-src/           # Hand-written C++ implementations
│   ├── android/                   # Android project files
│   ├── ios/                       # iOS project files
│   └── ts/                        # TypeScript project files
│
├── bzl/                           # ── BUILD SUPPORT ─────────────────────
│   └── ...                        # Bazel helper rules
├── WORKSPACE                      # Bazel workspace (external deps)
├── BUILD                          # Top-level Bazel build
└── src/BUILD                      # Scala source Bazel build
```

---

## IDL Syntax (the `.djinni` language)

```djinni
# Enums
color = enum {
    red; green; blue;
}

# Records (value types / structs)
vehicle = record {
    id: string;
    wheels: i32;
}

# Record inheritance (this fork's feature)
bus = record extends vehicle {
    headsign: string;
}

# Interfaces (classes with methods)
# +c = C++ impl, +j = Java impl, +o = Obj-C impl, +w = Wasm impl
driver = interface +c +j +o {
    drive(vehicle: vehicle): string;
    static create(): driver;
    const get_name(): string;
}

# Generics
container<T> = record { value: T; }

# Built-in types
#   Primitives: i8, i16, i32, i64, f32, f64, bool, string, binary, date, void
#   Collections: list<T>, set<T>, map<K,V>, array<T>, optional<T>

# Constants
my_constant: i32 = 42;

# Derivings: deriving(ord), deriving(parcelable), deriving(nscopy)
```

---

## Type Mapping Across Languages

| Djinni          | C++                    | Java            | Kotlin        | Obj-C               |
|-----------------|------------------------|-----------------|---------------|----------------------|
| `i32`           | `int32_t`              | `int`           | `Int`         | `int32_t`            |
| `string`        | `std::string`          | `String`        | `String`      | `NSString *`         |
| `optional<T>`   | `std::optional<T>`     | `T` (nullable)  | `T?`          | `T` (nullable)       |
| `list<T>`       | `std::vector<T>`       | `ArrayList<T>`  | `List<T>`     | `NSArray<T>`         |
| `map<K,V>`      | `std::unordered_map`   | `HashMap<K,V>`  | `Map<K,V>`    | `NSDictionary<K,V>`  |
| interface       | `shared_ptr<I>`        | proxy class     | proxy class   | proxy class          |

---

## Build System

- **Bazel** is the primary build tool
- Scala 2.11.12 with dependencies: `scopt` (CLI), `scala-parser-combinators` (parsing), `snakeyaml` (YAML)
- Main target: `//src:djinni`

## Key CLI Arguments

```
--idl <file.djinni>       # Input IDL file
--cpp-out <dir>           # C++ output directory
--java-out <dir>          # Java output directory
--kotlin-out <dir>        # Kotlin output directory
--objc-out <dir>          # Objective-C output directory
--jni-out <dir>           # JNI output directory
--ts-out <dir>            # TypeScript output directory
--wasm-out <dir>          # WebAssembly output directory
```

Plus language-specific options for namespaces, prefixes, include paths, and identifier styles.

---

## Key Architectural Concepts

1. **Each language gets a Generator + Marshal pair** — the Generator handles file structure/layout, the Marshal handles type name translation and conversion code
2. **JNI and Obj-C++ are bridge layers** — they sit between the native language (Java/Obj-C) and C++, handling memory management and type conversion
3. **External types via YAML** — existing C++ types can be mapped to other languages without modifying the IDL, using YAML config files
4. **Support libraries are required at runtime** — generated code depends on the support-lib headers/classes for marshaling, proxy caching, exception translation, and async support
5. **Interface proxying** — when a C++ interface is passed to Java (or vice versa), a proxy object is created that forwards method calls across the language boundary
