#!/usr/bin/env bash
set -eu

# Verify that generated Swift files compile correctly.
# This script compiles the generated Swift output to catch syntax errors
# and type mismatches in the generator.
#
# Usage: ./verify_swift.sh [--regenerate]
#   --regenerate: Also run Djinni generation before compilation

base_dir=$(cd "$(dirname "$0")" && pwd)
swift_out="$base_dir/generated-src/swift"

if [ "${1:-}" = "--regenerate" ]; then
    echo "Running Djinni generation first..."
    "$base_dir/run_djinni.sh"
fi

if [ ! -d "$swift_out" ]; then
    echo "Error: No generated Swift files found at $swift_out"
    echo "Run run_djinni.sh first, or use --regenerate flag."
    exit 1
fi

swift_files=("$swift_out"/*.swift)
if [ ${#swift_files[@]} -eq 0 ]; then
    echo "Error: No .swift files found in $swift_out"
    exit 1
fi

echo "Found ${#swift_files[@]} Swift files to verify."
echo ""

# Separate files by what they import:
# - Files importing only Foundation/UIKit can be compiled with the iOS SDK
# - Files importing external frameworks (TestFramework, etc.) can only be syntax-checked
compilable_files=()
external_files=()

for f in "${swift_files[@]}"; do
    imports=$(grep '^import ' "$f" | awk '{print $2}')
    needs_external=false
    for imp in $imports; do
        case "$imp" in
            Foundation|UIKit) ;; # available in iOS SDK
            *) needs_external=true ;;
        esac
    done
    if $needs_external; then
        external_files+=("$f")
    else
        compilable_files+=("$f")
    fi
done

errors=0

# Compile all Foundation/UIKit files together with iOS SDK
# (They reference each other, so they must be compiled together)
if [ ${#compilable_files[@]} -gt 0 ]; then
    ios_sdk=$(xcrun --sdk iphonesimulator --show-sdk-path 2>/dev/null || true)
    if [ -n "$ios_sdk" ]; then
        echo "Compiling ${#compilable_files[@]} files with iOS SDK..."
        if swiftc -typecheck -sdk "$ios_sdk" -target arm64-apple-ios16.0-simulator "${compilable_files[@]}" 2>&1; then
            echo "  OK: All ${#compilable_files[@]} files compile successfully."
        else
            echo "  FAIL: Compilation errors found."
            errors=$((errors + 1))
        fi
    else
        echo "No iOS SDK found. Falling back to macOS SDK..."
        macos_sdk=$(xcrun --sdk macosx --show-sdk-path)
        if swiftc -typecheck -sdk "$macos_sdk" "${compilable_files[@]}" 2>&1; then
            echo "  OK: All ${#compilable_files[@]} files compile successfully."
        else
            echo "  FAIL: Compilation errors found."
            errors=$((errors + 1))
        fi
    fi
fi

# Syntax-check files with external framework imports
if [ ${#external_files[@]} -gt 0 ]; then
    echo ""
    echo "Syntax-checking ${#external_files[@]} files with external framework imports..."
    for f in "${external_files[@]}"; do
        fname=$(basename "$f")
        if swiftc -parse "$f" 2>/dev/null; then
            true
        else
            echo "  FAIL: Syntax error in $fname"
            errors=$((errors + 1))
        fi
    done
    if [ $errors -eq 0 ]; then
        echo "  OK: All external-import files pass syntax check."
    fi
fi

echo ""
if [ $errors -eq 0 ]; then
    echo "All ${#swift_files[@]} Swift files verified successfully."
    exit 0
else
    echo "FAILED: $errors compilation error(s) found."
    exit 1
fi
