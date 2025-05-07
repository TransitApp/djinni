# Transit / Djinni

Djinni is a project originally created by Dropbox that generates bridging code
between C++ and other programming languages.

After Dropbox dropped the support of Djinni, Snapchat took over the maintenance.

Since we needed some changes we created a fork based on our needs. This fork is not inteded to be maintained for all languages (we only need Djinni for our view models in C++, Kotlin and Objc).

**You should probably use the official Djinni tool from Snap.**

[Original snapchat readme](README.snapchat.md) for the full Snapchat Djinni documentation.

[Original dropbox readme](README.dropbox.md) for the full Djinni documentation.



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

