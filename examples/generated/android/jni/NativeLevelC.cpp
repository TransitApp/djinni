// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#include "NativeLevelC.h"  // my header
#include "Marshal.hpp"
#include "NativeLevelD.h"
#include "NativeLevelD2.h"
#include "NativeLevelE.h"
#include "NativeLevelF.h"

namespace djinni_generated {

NativeLevelC::NativeLevelC() = default;

NativeLevelC::~NativeLevelC() = default;

auto NativeLevelC::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
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
    else {
        const auto& data = ::djinni::JniClass<NativeLevelC>::get();
        r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor,
                                                          ::djinni::get(::djinni::String::fromCpp(jniEnv, c->fieldA)),
                                                          ::djinni::get(::djinni::String::fromCpp(jniEnv, c->fieldB)),
                                                          ::djinni::get(::djinni::String::fromCpp(jniEnv, c->fieldC)))};
        ::djinni::jniExceptionCheck(jniEnv);
    }
    return r;
}

auto NativeLevelC::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 4);
    assert(j != nullptr);
    const auto& data = ::djinni::JniClass<NativeLevelC>::get();
    std::shared_ptr<::transitLib::viewModel::LevelC> model;
    model->fieldA = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldA));
    model->fieldB = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldB));
    model->fieldC = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_fieldC));
    return model;
}

} // namespace djinni_generated
