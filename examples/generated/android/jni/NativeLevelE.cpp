// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#include "NativeLevelE.h"  // my header
#include "Marshal.hpp"

namespace djinni_generated {

NativeLevelE::NativeLevelE() = default;

NativeLevelE::~NativeLevelE() = default;

auto NativeLevelE::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
    const auto& data = ::djinni::JniClass<NativeLevelE>::get();
    auto r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor,
                                                           ::djinni::get(::djinni::String::fromCpp(jniEnv, c.fieldA)),
                                                           ::djinni::get(::djinni::String::fromCpp(jniEnv, c.fieldB)),
                                                           ::djinni::get(::djinni::String::fromCpp(jniEnv, c.fieldC)),
                                                           ::djinni::get(::djinni::String::fromCpp(jniEnv, c.fieldD)),
                                                           ::djinni::get(::djinni::String::fromCpp(jniEnv, c.fieldE)))};
    ::djinni::jniExceptionCheck(jniEnv);
    return r;
}

auto NativeLevelE::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 6);
    assert(j != nullptr);
    const auto& data = ::djinni::JniClass<NativeLevelE>::get();
    ::transitLib::vm::LevelE model;
    model.fieldA = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldA));
    model.fieldB = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldB));
    model.fieldC = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldC));
    model.fieldD = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldD));
    model.fieldE = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldE));
    return model;
}

} // namespace djinni_generated
