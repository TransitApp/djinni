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
| Java     | PASS    | Full comparison, Parcelable implemented |
| Kotlin   | PASS    | Full comparison, blank-line formatting fixed |
| WASM     | PASS    | Full comparison, Emscripten bindings implemented |
| TS       | PASS    | Full comparison, all invocations |

---

## Phase 1 - Polish Existing Generators

### ~~1.1 Kotlin blank-line formatting~~ DONE
### ~~1.2 Java Parcelable~~ DONE

---

## Phase 2 - New Generators

### ~~2.1 TypeScript Generator~~ DONE
### ~~2.2 WASM Generator~~ DONE

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
