// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from primtypes.djinni

#include "NativeAssortedPrimitives.hpp"  // my header
#include "Marshal.hpp"

namespace djinni_generated {

NativeAssortedPrimitives::NativeAssortedPrimitives() = default;

NativeAssortedPrimitives::~NativeAssortedPrimitives() = default;

auto NativeAssortedPrimitives::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
    const auto& data = ::djinni::JniClass<NativeAssortedPrimitives>::get();
    auto r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor,
                                                           ::djinni::get(::djinni::Bool::fromCpp(jniEnv, c.b)),
                                                           ::djinni::get(::djinni::I8::fromCpp(jniEnv, c.eight)),
                                                           ::djinni::get(::djinni::I16::fromCpp(jniEnv, c.sixteen)),
                                                           ::djinni::get(::djinni::I32::fromCpp(jniEnv, c.thirtytwo)),
                                                           ::djinni::get(::djinni::I64::fromCpp(jniEnv, c.sixtyfour)),
                                                           ::djinni::get(::djinni::F32::fromCpp(jniEnv, c.fthirtytwo)),
                                                           ::djinni::get(::djinni::F64::fromCpp(jniEnv, c.fsixtyfour)),
                                                           ::djinni::get(::djinni::Optional<std::experimental::optional, ::djinni::Bool>::fromCpp(jniEnv, c.o_b)),
                                                           ::djinni::get(::djinni::Optional<std::experimental::optional, ::djinni::I8>::fromCpp(jniEnv, c.o_eight)),
                                                           ::djinni::get(::djinni::Optional<std::experimental::optional, ::djinni::I16>::fromCpp(jniEnv, c.o_sixteen)),
                                                           ::djinni::get(::djinni::Optional<std::experimental::optional, ::djinni::I32>::fromCpp(jniEnv, c.o_thirtytwo)),
                                                           ::djinni::get(::djinni::Optional<std::experimental::optional, ::djinni::I64>::fromCpp(jniEnv, c.o_sixtyfour)),
                                                           ::djinni::get(::djinni::Optional<std::experimental::optional, ::djinni::F32>::fromCpp(jniEnv, c.o_fthirtytwo)),
                                                           ::djinni::get(::djinni::Optional<std::experimental::optional, ::djinni::F64>::fromCpp(jniEnv, c.o_fsixtyfour)))};
    ::djinni::jniExceptionCheck(jniEnv);
    return r;
}

auto NativeAssortedPrimitives::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 15);
    assert(j != nullptr);
    const auto& data = ::djinni::JniClass<NativeAssortedPrimitives>::get();
    ::testsuite::AssortedPrimitives model;
    model.mB = ::djinni::Bool::toCpp(jniEnv, jniEnv->GetBooleanField(j, data.field_mB));
    model.mEight = ::djinni::I8::toCpp(jniEnv, jniEnv->GetByteField(j, data.field_mEight));
    model.mSixteen = ::djinni::I16::toCpp(jniEnv, jniEnv->GetShortField(j, data.field_mSixteen));
    model.mThirtytwo = ::djinni::I32::toCpp(jniEnv, jniEnv->GetIntField(j, data.field_mThirtytwo));
    model.mSixtyfour = ::djinni::I64::toCpp(jniEnv, jniEnv->GetLongField(j, data.field_mSixtyfour));
    model.mFthirtytwo = ::djinni::F32::toCpp(jniEnv, jniEnv->GetFloatField(j, data.field_mFthirtytwo));
    model.mFsixtyfour = ::djinni::F64::toCpp(jniEnv, jniEnv->GetDoubleField(j, data.field_mFsixtyfour));
    model.mOB = ::djinni::Optional<std::experimental::optional, ::djinni::Bool>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_mOB));
    model.mOEight = ::djinni::Optional<std::experimental::optional, ::djinni::I8>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_mOEight));
    model.mOSixteen = ::djinni::Optional<std::experimental::optional, ::djinni::I16>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_mOSixteen));
    model.mOThirtytwo = ::djinni::Optional<std::experimental::optional, ::djinni::I32>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_mOThirtytwo));
    model.mOSixtyfour = ::djinni::Optional<std::experimental::optional, ::djinni::I64>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_mOSixtyfour));
    model.mOFthirtytwo = ::djinni::Optional<std::experimental::optional, ::djinni::F32>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_mOFthirtytwo));
    model.mOFsixtyfour = ::djinni::Optional<std::experimental::optional, ::djinni::F64>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_mOFsixtyfour));
    return model;
}

} // namespace djinni_generated
