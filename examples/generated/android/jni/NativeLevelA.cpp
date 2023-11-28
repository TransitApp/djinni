// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#include "NativeLevelA.h"  // my header
#include "Marshal.hpp"

namespace djinni_generated {

NativeLevelA::NativeLevelA() = default;

NativeLevelA::~NativeLevelA() = default;

auto NativeLevelA::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
    const auto& data = ::djinni::JniClass<NativeLevelA>::get();
    auto r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor,
                                                           ::djinni::get(::djinni::String::fromCpp(jniEnv, c.fieldA)))};
    ::djinni::jniExceptionCheck(jniEnv);
    return r;
}

auto NativeLevelA::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 2);
    assert(j != nullptr);
    const auto& data = ::djinni::JniClass<NativeLevelA>::get();
    ::transitLib::viewModel::LevelA model;
    model.fieldA = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldA));
    return model;
}

} // namespace djinni_generated