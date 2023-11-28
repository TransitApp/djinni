#! /usr/bin/env bash
set -eu
shopt -s nullglob

# Locate the script file.  Cross symlinks if necessary.
loc="$0"
while [ -h "$loc" ]; do
    ls=`ls -ld "$loc"`
    link=`expr "$ls" : '.*-> \(.*\)$'`
    if expr "$link" : '/.*' > /dev/null; then
        loc="$link"  # Absolute link
    else
        loc="`dirname "$loc"`/$link"  # Relative link
    fi
done
base_dir=$(cd "`dirname "$loc"`" && pwd)

temp_out="$base_dir/djinni-output-temp"

in="$base_dir/example.djinni"

cpp_out="$base_dir/generated/cpp"
jni_out="$base_dir/generated/android/jni"
objc_out="$base_dir/generated/objc"
java_out="$base_dir/generated/android/djinni/java/src"

java_package="djinni.java.src"

gen_stamp="$temp_out/gen.stamp"

if [ $# -eq 0 ]; then
    # Normal build.
    true
elif [ $# -eq 1 ]; then
    command="$1"; shift
    if [ "$command" != "clean" ]; then
        echo "Unexpected argument: \"$command\"." 1>&2
        exit 1
    fi
    for dir in "$temp_out" "$cpp_out" "$jni_out" "$java_out"; do
        if [ -e "$dir" ]; then
            echo "Deleting \"$dir\"..."
            rm -r "$dir"
        fi
    done
    exit
fi

# Build djinni
"$base_dir/../src/build.sh"

[ ! -e "$temp_out" ] || rm -r "$temp_out"
"$base_dir/../src/run-assume-built" \
    --cpp-out "$temp_out/cpp" \
    --cpp-namespace transitLib::viewModel \
    --ident-cpp-file FooBarViewModel \
    --ident-cpp-enum FooBar \
    --cpp-struct-constructor false\
    \
    --kotlin-out "$temp_out/java" \
    --java-package djinni.java.src \
    --java-annotation androidx.compose.runtime.Immutable \
    --java-nullable-annotation androidx.annotation.Nullable \
    \
    --hpp-ext h \
    --jni-out "$temp_out/jni" \
    --ident-jni-class NativeFooBar \
    --ident-jni-file NativeFooBar \
    \
    --objc-out "$temp_out/objc" \
    --ident-objc-type SPFooBarViewModel \
    \
    --objcpp-out "$temp_out/objc" \
    --idl "$in"

# Copy changes from "$temp_output" to final dir.

mirror() {
    local prefix="$1" ; shift
    local src="$1" ; shift
    local dest="$1" ; shift
    mkdir -p "$dest"
    rsync -r --delete --checksum --itemize-changes "$src"/ "$dest" | sed "s/^/[$prefix]/"
}

echo "Copying generated code to final directories..."
mirror "cpp" "$temp_out/cpp" "$cpp_out"
mirror "java" "$temp_out/java" "$java_out"
mirror "jni" "$temp_out/jni" "$jni_out"
mirror "objc" "$temp_out/objc" "$objc_out"

date > "$gen_stamp"

echo "djinni completed."
