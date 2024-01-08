// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#include "NativeLevelB2.h"  // my header
#include "Marshal.hpp"
#include "NativeLevelB2C.h"

namespace djinni_generated {

NativeLevelB2::NativeLevelB2() = default;

NativeLevelB2::~NativeLevelB2() = default;

auto NativeLevelB2::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
    ::djinni::LocalRef<JniType> r;
    if (auto myObject = dynamic_pointer_cast<::transitLib::viewModel::LevelB2C>(c)) {
       r = NativeLevelB2C::fromCpp(jniEnv, *myObject);
    }
    else {
    const auto& data = ::djinni::JniClass<NativeLevelB2>::get();
    r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor,
                                                      ::djinni::get(::djinni::String::fromCpp(jniEnv, c->fieldA)),
                                                      ::djinni::get(::djinni::String::fromCpp(jniEnv, c->fieldB)),
                                                      ::djinni::get(::djinni::String::fromCpp(jniEnv, c->fieldB2)))};
    ::djinni::jniExceptionCheck(jniEnv);
    }
    return r;
}

auto NativeLevelB2::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 4);
    assert(j != nullptr);
    const auto& data = ::djinni::JniClass<NativeLevelB2>::get();
    std::shared_ptr<::transitLib::viewModel::LevelB2> model;
    model->fieldA = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldA));
    model->fieldB = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldB));
    model->fieldB2 = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldB2));
    return model;
}

} // namespace djinni_generated
