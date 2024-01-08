// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#include "NativeLevelB.h"  // my header
#include "Marshal.hpp"
#include "NativeLevelB2.h"
#include "NativeLevelB2C.h"
#include "NativeLevelC.h"
#include "NativeLevelD.h"
#include "NativeLevelD2.h"
#include "NativeLevelE.h"
#include "NativeLevelF.h"

namespace djinni_generated {

NativeLevelB::NativeLevelB() = default;

NativeLevelB::~NativeLevelB() = default;

auto NativeLevelB::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
    ::djinni::LocalRef<JniType> r;
    if (auto myObject = dynamic_pointer_cast<::transitLib::viewModel::LevelD2>(c))
    {
        r = NativeLevelD2::fromCpp(jniEnv, *myObject);
    }
    else if (auto myObject = dynamic_pointer_cast<::transitLib::viewModel::LevelF>(c))
    {
        r = NativeLevelF::fromCpp(jniEnv, *myObject);
    }
    else if (auto myObject = dynamic_pointer_cast<::transitLib::viewModel::LevelE>(c))
    {
        r = NativeLevelE::fromCpp(jniEnv, myObject);
    }
    else if (auto myObject = dynamic_pointer_cast<::transitLib::viewModel::LevelD>(c))
    {
        r = NativeLevelD::fromCpp(jniEnv, myObject);
    }
    else if (auto myObject = dynamic_pointer_cast<::transitLib::viewModel::LevelC>(c))
    {
        r = NativeLevelC::fromCpp(jniEnv, myObject);
    }
    else if (auto myObject = dynamic_pointer_cast<::transitLib::viewModel::LevelB2C>(c))
    {
        r = NativeLevelB2C::fromCpp(jniEnv, *myObject);
    }
    else if (auto myObject = dynamic_pointer_cast<::transitLib::viewModel::LevelB2>(c))
    {
        r = NativeLevelB2::fromCpp(jniEnv, myObject);
    }
    else {
        const auto& data = ::djinni::JniClass<NativeLevelB>::get();
        r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor,
                                                          ::djinni::get(::djinni::String::fromCpp(jniEnv, c->fieldA)),
                                                          ::djinni::get(::djinni::String::fromCpp(jniEnv, c->fieldB)))};
        ::djinni::jniExceptionCheck(jniEnv);
    }
    return r;
}

auto NativeLevelB::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 3);
    assert(j != nullptr);
    const auto& data = ::djinni::JniClass<NativeLevelB>::get();
    std::shared_ptr<::transitLib::viewModel::LevelB> model;
    model->fieldA = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldA));
    model->fieldB = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldB));
    return model;
}

} // namespace djinni_generated
