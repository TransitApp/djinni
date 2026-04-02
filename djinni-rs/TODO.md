# Djinni Rust Rewrite - Implementation Plan

## Current State

The Rust rewrite (`djinni-rs`) covers the core Djinni pipeline: parsing, resolution, and
code generation for **C++, JNI, Objective-C, Objective-C++, Java, Kotlin, and YAML**.

### Golden Test Status

| Target   | Status  | Notes |
|----------|---------|-------|
| C++      | PASS    | Full comparison including YAML round-trip types |
| JNI      | PASS    | Full comparison, all 7 invocations |
| ObjC     | PASS    | Full comparison (ObjC + ObjC++ in same dir) |
| Java     | SKIPPED | Parcelable `writeToParcel`/`createFromParcel` not implemented |
| Kotlin   | SKIPPED | Minor blank-line formatting diffs in `companion object` |
| WASM     | SKIPPED | Generator not implemented |
| TS       | SKIPPED | Generator not implemented |

---

## Phase 1 - Polish Existing Generators

### 1.1 Kotlin blank-line formatting (SMALL)
**Files:** `kotlin_gen.rs`
**Issue:** Extra/missing blank lines around `companion object` and static methods.
The Scala `KotlinGenerator` uses `w.wl` spacing that differs from the Rust version.
**Fix:** Adjust `wl_empty()` placement around:
- `companion object` opening (lines ~277, ~351, ~716)
- First static method inside companion (skip blank line when no `init` block)
- Interface CppProxy companion object
**Test:** Re-enable Kotlin golden comparison in `golden.rs`

### 1.2 Java Parcelable (LARGE)
**Files:** `java_gen.rs`
**Reference:** `JavaGenerator.scala:474-619` (~145 lines)
**What:** Generate Android `Parcelable` implementation when `deriving(parcelable)` is set:
- Static `CREATOR` field (anonymous `Parcelable.Creator<T>`)
- Constructor from `android.os.Parcel`
- `describeContents()` (returns 0)
- `writeToParcel(Parcel out, int flags)`
**Types to handle in serialization:**
- Primitives (byte, short, int, long, float, double, boolean) with byte/short special cases
- String, Binary (byte[]), Date (via `getTime()`/`new Date(readLong())`)
- Collections: List, Set (as ArrayList), Map
- Records (recursive `writeToParcel`)
- Enums (ordinal/values[])
- External types (use `e.java.readFromParcel`/`writeToParcel` format strings with `%s`)
- Optional types (null check with `readByte` sentinel)
**Test:** Re-enable Java golden comparison in `golden.rs`

---

## Phase 2 - New Generators

### 2.1 TypeScript Generator (MEDIUM)
**New file:** `ts_gen.rs`
**Reference:** `TsGenerator.scala` (~303 lines)
**What:** Generate TypeScript type definitions for WASM bindings.
- Single combined output file (not one-per-type)
- Type mapping: primitives -> boolean/number/bigint, arrays -> typed arrays, collections -> Array/Set/Map
- Enum: `export enum Foo { ... }`
- Record: `export interface Foo { field: Type; }`
- Interface: `export interface Foo { method(params): RetType; }` with separate `_statics` interface
- Import statements from external modules
- Optional fields use `?: Type` syntax
**Test:** Re-enable `ts` in golden subdirs comparison

### 2.2 WASM Generator (LARGE)
**New file:** `wasm_gen.rs`
**Reference:** `WasmGenerator.scala` (~495 lines)
**What:** Generate C++ Emscripten bindings for JavaScript interop.
- Enum: `WasmEnum` struct + `EM_JS` function + `EMSCRIPTEN_BINDINGS`
- Interface: `JsInterface` wrapper, C++ stubs -> JS, JS Proxy class -> C++
- Record: field marshaling (toCpp/fromCpp) + construction/deconstruction
- C++ template metaprogramming: `JsClassName<...>`, helper templates
- Exception translation, bidirectional marshaling
**Test:** Re-enable `wasm` in golden subdirs comparison

---

## Phase 3 - Feature Parity

### 3.1 Missing CLI Flags
Add CLI flags that exist in Scala but not Rust. Grouped by priority:

**High priority (affect golden test correctness):**
- `--jni-include-prefix`, `--jni-include-cpp-prefix`, `--jni-base-lib-include-prefix`
- `--jni-header-out` (separate JNI header output dir)
- `--cpp-header-out` (separate C++ header output dir)
- `--cpp-base-lib-include-prefix`
- `--objc-include-prefix`, `--objc-extended-record-include-prefix`
- `--objcpp-include-objc-prefix`

**Medium priority (feature completeness):**
- `--java-annotation`, `--java-cpp-exception`, `--java-class-access-modifier`
- `--objcpp-disable-exception-translation`
- `--objc-swift-bridging-header`
- `--cpp-nn-header`, `--cpp-nn-type`, `--cpp-nn-check-expression`

**Low priority (identifier styles - only needed for non-standard naming):**
- `--ident-java-enum`
- `--ident-cpp-enum`, `--ident-cpp-field`, `--ident-cpp-method`, etc.
- `--ident-objc-field`, `--ident-objc-method`, etc.

### 3.2 Swift Bridging Header (TRIVIAL)
**New file:** `swift_gen.rs` (~30-40 lines)
**Reference:** `SwiftBridgingHeaderGenerator.scala` (~60 lines)
**What:** Single header file with `#import "..."` for each generated ObjC type.
Triggered by `--objc-swift-bridging-header` flag.

---

## Implementation Notes

### Test Strategy
Each item should:
1. Implement the feature
2. Re-enable the corresponding golden test comparison
3. Verify all golden tests still pass

### Architecture
- Generators live in `crates/djinni-generator/src/`
- Each generator is a standalone module (`foo_gen.rs`) with a public `generate_foo()` function
- Marshals (`foo_marshal.rs`) handle type mapping; generators handle code structure
- CLI wiring in `crates/djinni-cli/src/main.rs`
- Golden tests in `crates/djinni-cli/tests/golden.rs`

### Key References
- Scala source: `../src/source/` (relative to djinni-rs)
- Golden files: `../test-suite/generated-src/`
- Test IDL files: `../test-suite/djinni/`
- Test invocations: `../test-suite/run_djinni.sh` and `golden.rs`
