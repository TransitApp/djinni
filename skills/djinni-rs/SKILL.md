---
name: djinni-rs
description: Use the djinni-rs Rust code generator to generate cross-platform ViewModels (C++/Kotlin/JNI/ObjC) from Djinni IDL files, switch between the Scala and Rust generators, verify generator parity, and modify the generator itself. Use when working with .djinni files, ViewModel generation, generate_view_models.sh, or the TransitApp/djinni repo.
---

# djinni-rs — Transit's Djinni code generator

Djinni translates `.djinni` IDL files into C++, Kotlin, JNI, Objective-C, and
Objective-C++ bindings (Transit's ViewModels). This repo contains two
implementations that produce **byte-for-byte identical output**:

- **Rust** (`djinni-rs/`) — single native binary, ~1s for all of TransitLib. Preferred.
- **Scala** (`src/`) — legacy, needs bazel + JVM. Still the default until you switch.

The repo lives at `Submodules/djinni` inside TransitLib
(`app/src/main/cpp/TransitLib/Submodules/djinni` in Transit-Android checkouts).

## Switching generators

All scripts funnel through `src/run`, which dispatches on a gitignored
`.djinni-generator` marker:

```sh
./switch-generator.sh rust      # build djinni-rs and use it everywhere
./switch-generator.sh scala     # back to Scala
./switch-generator.sh status    # which generator is active
./switch-generator.sh verify    # cargo golden tests: byte-for-byte vs committed output
```

One-off override without switching: `DJINNI_GENERATOR=rust ./src/run ...`

Requirements for rust: cargo (`brew install rustup && rustup-init`). No JVM/bazel.

## Generating Transit ViewModels

From the TransitLib repo root (NOT this submodule):

```sh
./transitLib/Djinni/generate_view_models.sh              # all .djinni files
./transitLib/Djinni/generate_view_models.sh TripDetails  # single module
```

This merges all `.djinni` files (from `transitLib/Djinni/` and
`transitLib/ViewModules/`) into one `TransitLib.djinni`, runs the generator via
`src/run`, and rsyncs changed files into `transitLib/Djinni/generated/`.
Generated files are committed; never edit them by hand.

## Running the generator directly

```sh
djinni-rs/target/release/djinni \
    --idl my.djinni \
    --cpp-out gen/cpp --cpp-namespace transitLib::vm \
    --kotlin-out gen/kotlin --java-package djinni.java.src \
    --jni-out gen/jni \
    --objc-out gen/objc --objcpp-out gen/objc
```

Full flag reference: `djinni-rs/README.md` (or `djinni --help`). Flags are
identical to the Scala generator; existing scripts work unchanged.

## Testing and parity verification

```sh
cd djinni-rs
cargo build && cargo test        # golden tests need the DEBUG binary
```

Golden tests regenerate the whole test suite (7 invocations, 9 targets, ~770
files) and diff byte-for-byte against `test-suite/generated-src/`.

For a full end-to-end parity check against the Scala generator on real input:
generate TransitLib's ViewModels once with `DJINNI_GENERATOR=scala` and once
with `DJINNI_GENERATOR=rust` into two temp dirs and `diff -r` them.

Known deliberate deviations from Scala (do not "fix"):
- Java Parcelable writes `mField` instead of `this.mField`
- TypeScript imports are emitted in deterministic order

## Modifying the generator

Any change to generator behavior must land in **both** implementations until
the Scala generator is deleted, and golden files must be regenerated:

1. Change the Scala generator (`src/source/*.scala`) — it is the reference.
2. Port the change to the matching Rust module
   (`djinni-rs/crates/djinni-generator/src/*_gen.rs` / `*_marshal.rs`).
3. Regenerate goldens with the Scala generator:
   `./test-suite/run_djinni.sh && ./examples/run_djinni.sh && ./perftest/run_djinni.sh`
   (make sure the marker is on `scala`, or `DJINNI_GENERATOR=scala`).
4. `cd djinni-rs && cargo build && cargo test` — must pass byte-for-byte.
5. Commit generator changes and regenerated goldens together.

Crate map: `djinni-parser` (pest grammar + extern YAML), `djinni-resolver`
(type resolution), `djinni-generator` (one `*_gen.rs` + `*_marshal.rs` pair per
target), `djinni-cli` (flags → `Spec`, golden tests in `tests/golden.rs`).

## Transit IDL conventions

- Records with text properties use `SmartStringLabel`, never bare `SmartString`
  or `string` (see TransitLib docs).
- `# @test-representation-inline` on a record inlines its
  `getTestRepresentation()` output; `# @test-representation-disabled-property`
  on a property excludes it (use for values that change every run, e.g. ids).
- Extern types (SmartString, colors, UserAction, ...) come from
  `TransitLib.yaml`; record inheritance uses `record extends Base`.

## Troubleshooting

- `clang: error: unknown argument: '-fno-canonical-system-headers'` — Scala/bazel
  issue: run `bazel clean --expunge` at the repo root (or just switch to rust).
- Golden tests fail with "Djinni binary not found at .../target/debug/djinni" —
  run `cargo build` (debug) first; the golden tests use the debug binary.
- `Error: YAML parse error` on an extern file — check the YAML documents are
  `---`-separated mappings with `name`, `typedef`, `params`, `prefix` keys.
