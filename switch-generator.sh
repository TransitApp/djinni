#! /usr/bin/env bash
#
# Switch the Djinni generator used by this repo (and by TransitLib's
# generate_view_models.sh, which calls src/run) between:
#
#   rust   - djinni-rs, a single native binary (~1s for all of TransitLib)
#   scala  - the legacy Scala generator built with bazel (the default)
#
# Usage:
#   ./switch-generator.sh rust      # build djinni-rs and make it the generator
#   ./switch-generator.sh scala     # go back to the Scala generator
#   ./switch-generator.sh status    # show which generator is active
#   ./switch-generator.sh verify    # prove both generators produce identical output
#
# The choice is stored in .djinni-generator (gitignored, per-checkout) and
# honored by src/run and src/run-assume-built. It can be overridden per
# invocation with the DJINNI_GENERATOR environment variable.
set -eu

base_dir=$(cd "$(dirname "$0")" && pwd)
marker="$base_dir/.djinni-generator"
rust_bin="$base_dir/djinni-rs/target/release/djinni"

current_generator() {
    if [ -n "${DJINNI_GENERATOR:-}" ]; then
        echo "$DJINNI_GENERATOR (from DJINNI_GENERATOR env var)"
    elif [ -f "$marker" ]; then
        cat "$marker"
    else
        echo "scala (default)"
    fi
}

require_cargo() {
    if ! command -v cargo > /dev/null; then
        echo "cargo not found. Install Rust first:" 1>&2
        echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" 1>&2
        echo "or: brew install rustup && rustup-init" 1>&2
        exit 1
    fi
}

build_rust() {
    require_cargo
    echo "Building djinni-rs (release)..."
    (cd "$base_dir/djinni-rs" && cargo build --release)
}

case "${1:-status}" in
    rust)
        build_rust
        # Smoke test: parse and generate a minimal record
        smoke_dir=$(mktemp -d)
        trap 'rm -rf "$smoke_dir"' EXIT
        printf 'smoke_test = record {\n    value: i32;\n}\n' > "$smoke_dir/smoke.djinni"
        "$rust_bin" --idl "$smoke_dir/smoke.djinni" --cpp-out "$smoke_dir/out" > /dev/null
        [ -f "$smoke_dir/out/smoke_test.hpp" ] || { echo "Smoke test failed: no output generated." 1>&2; exit 1; }
        echo "rust" > "$marker"
        echo ""
        echo "Switched to the Rust generator (djinni-rs)."
        echo "src/run and generate_view_models.sh will now use: $rust_bin"
        echo "Run './switch-generator.sh verify' to check parity against the Scala generator."
        ;;
    scala)
        rm -f "$marker"
        echo "Switched back to the Scala generator (bazel //src:djinni, the default)."
        ;;
    status)
        echo "Active generator: $(current_generator)"
        if [ -x "$rust_bin" ]; then
            echo "djinni-rs binary:  $rust_bin (built)"
        else
            echo "djinni-rs binary:  not built (run ./switch-generator.sh rust)"
        fi
        ;;
    verify)
        build_rust
        echo "Running djinni-rs golden tests (byte-for-byte comparison against Scala output)..."
        (cd "$base_dir/djinni-rs" && cargo build && cargo test)
        echo ""
        echo "All golden tests passed: djinni-rs output matches the committed Scala generator output."
        ;;
    *)
        echo "Usage: $0 [rust|scala|status|verify]" 1>&2
        exit 1
        ;;
esac
