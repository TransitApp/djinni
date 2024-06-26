// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from constants.djinni

#include "NativeConstants.hpp"  // my header

namespace djinni_generated {

NativeConstants::NativeConstants() = default;

NativeConstants::~NativeConstants() = default;

auto NativeConstants::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
    (void)c; // Suppress warnings in release builds for empty records
    const auto& data = ::djinni::JniClass<NativeConstants>::get();
    auto r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor)};
    ::djinni::jniExceptionCheck(jniEnv);
    return r;
}

auto NativeConstants::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 1);
    assert(j != nullptr);
    (void)j; // Suppress warnings in release builds for empty records
    ::testsuite::Constants model;
    return model;
}

} // namespace djinni_generated
