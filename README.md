# Transit / Djinni

Djinni is a project originally created by Dropbox that generates bridging code
between C++ and other programming languages.

After Dropbox dropped the support of Djinni, Snapchat took over the maintenance.

Since we needed some changes we created a fork based on our needs. This fork is not inteded to be maintained for all languages (we only need Djinni for our view models in C++, Kotlin and Objc).

**You should probably use the official Djinni tool from Snap.**

[Original snapchat readme](README.snapchat.md) for the full Snapchat Djinni documentation.

[Original dropbox readme](README.dropbox.md) for the full Djinni documentation.

## Rust rewrite (djinni-rs)

The repo contains two implementations of the generator:

- **Scala** (`src/`) — the original generator, built with bazel + a JVM. Still the default.
- **Rust** (`djinni-rs/`) — a rewrite that produces **byte-for-byte identical output** and
  compiles to a single native binary. Generating all of TransitLib's ViewModels takes
  about **1 second**, versus the JVM/bazel startup alone taking much longer. See
  [djinni-rs/README.md](djinni-rs/README.md) for architecture, CLI reference, and benchmarks.

### Switching between generators

Everything (including TransitLib's `generate_view_models.sh`) invokes Djinni through
`src/run`, which dispatches to whichever generator is active:

```sh
./switch-generator.sh rust      # build djinni-rs and use it everywhere
./switch-generator.sh scala     # go back to the Scala generator
./switch-generator.sh status    # show which generator is active
./switch-generator.sh verify    # run byte-for-byte golden tests against committed output
```

The choice is stored per-checkout in a gitignored `.djinni-generator` file, so
switching never dirties the repo or affects CI. A single invocation can also be
forced with the `DJINNI_GENERATOR` environment variable:

```sh
DJINNI_GENERATOR=rust ./src/run --idl my.djinni --cpp-out out
```

Requirements for the Rust generator: a Rust toolchain (`brew install rustup && rustup-init`,
Rust 1.70+). No JVM, sbt, or bazel needed.

### Parity guarantees

- The golden tests in `djinni-rs` (`cargo test`) compare generated output for all 9
  language targets against the committed output of the Scala generator.
- The full merged `TransitLib.djinni` (all ViewModules, ~3200 lines) generates
  byte-for-byte identical C++, Kotlin, JNI, ObjC, and ObjC++ with both generators.
- Two known cosmetic deviations, both deliberate: Java Parcelable writes
  `mField` instead of `this.mField`, and TypeScript imports are emitted in
  deterministic order (the Scala generator's order depends on map iteration).

### Claude Code skill

The repo ships a [Claude Code](https://claude.com/claude-code) skill that teaches the
agent how to drive djinni-rs (generating ViewModels, switching generators, verifying
parity). Install it from the repo root:

```sh
npx skills add TransitApp/djinni
```

This picks up the `djinni-rs` skill from `skills/djinni-rs/` and installs it into
`~/.claude/skills/` (choose the global scope when prompted, or `--global`).



# Djinni

## Errors

If you ever get this error

`clang: error: unknown argument: '-fno-canonical-system-headers'`

Please run `bazel clean --expunge` at the root where the `WORKSPACE` file is located

## Modifications

 - Added Kotlin support
 - Added inheritance support (only in C++, Objc, Kotlin, Java)
 - Cpp default values

## Using new features

### Kotlin support

In your script replace `--java-out` by `--kotlin-out`:


### Inheritance

You can add inheritence to your records. Here is an example:

```
Vehicle = record {
   id : string;
} 

Bus = record extends Vehicle {
  headsign : string;
}
```

##### Inheritance limitations with collections

When using a vector in C++, a parent class should be a `shared_ptr` in order to be able to do a `dynamic_cast`.

The JNI and Objc generated code should then try to cast items to every possible children types and call the proper `fromCpp` methods. 

**This is not done yet.** 

If you want to use inherited records in lists here is what you can do: 

Instead of declaring `items : list<Vehicle>` create a special type like this 
`items : VehicleListItems`

Then, in a YAML file, define something like this:

```
---
name: VehicleListItems
typedef: 'record deriving(od)'
params: []
prefix: ''
cpp:
    typename: 'std::vector<std::shared_ptr<Vehicle>>'
    header: '"Vehicle.h"'
    byValue: false
objc:
    typename: 'NSArray<NSVehicle *>'
    header: '"NSVehicle.h"'
    boxed: 'NSArray<NSVehicle *>'
    pointer: true
    hash: '%s.hash'
objcpp:
    translator: 'MyDjinniTranslator:: VehicleListItems'
    header: '"MyDjinniTranslator.h"'
java:
    typename: 'ArrayList<Vehicle>'
    hash: '%s.hashCode()'
    boxed: 'ArrayList<Vehicle>'
    reference: false
    generic: true
jni:
    typename: jobject
    typeSignature: 'Ljava/util/ArrayList;'
    translator: 'djinniTranslator::VehicleListItems'
    header: '"DjinniTranslator.h"'
---
```

Then in you translators, implements the methods where you will iterate threw your vector of Vehicles and do dynamic cast to properly call the right djinni methods.

```
example for JNI:

    ::djinni::LocalRef<jobject> VehicleListItems::fromCpp(JNIEnv* jniEnv, const VehicleListItems::CppType& items) {
     		//create a jobject of ArrayList
     		//then iterate
       for (auto const& item : items) {
			            if (auto bus = dynamic_pointer_cast<Bus>(item)) {
			            jobject obj = djinni_generated::NativeBus::fromCpp(jniEnv, *bus).release();
			            // add obj to your array list
			}
		}
		
		  auto ref = ::djinni::LocalRef<jobject>{myJniArrayList};
        ::djinni::jniExceptionCheck(jniEnv);
        return ref;
    }
```

### Properties default values

You can add a default value to properties in C++

```
Bike = record {
    text: string = "default string";
    count: i32 = 0;
}
```

will generate

```
struct Bike final {
    std::string text = "default string";
    int32_t count = 0;
};
```

