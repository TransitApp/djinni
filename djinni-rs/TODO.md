# Djinni Rust Rewrite - Status

## Current State

The Rust rewrite (`djinni-rs`) is **feature-complete** for all code generation targets.
All golden tests pass, and all CLI flags from the Scala implementation are wired.

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

### Completed Phases

- **Phase 1** - Polish existing generators (Kotlin formatting, Java Parcelable)
- **Phase 2** - New generators (TypeScript, WASM)
- **Phase 3** - CLI flag parity (~35 flags added)

---

## Remaining (Low Priority)

### Swift Bridging Header (TRIVIAL)
**New file:** `swift_gen.rs` (~30-40 lines)
**Reference:** `SwiftBridgingHeaderGenerator.scala` (~60 lines)
**What:** Single header file with `#import "..."` for each generated ObjC type.
Triggered by `--objc-swift-bridging-header` flag.
Not used in any golden test invocations.

---

## Architecture

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
