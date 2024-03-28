// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#pragma once

#include "LevelDViewModel.h"
#include "djinni_support.hpp"

namespace djinni_generated {

class NativeLevelD final {
public:
    using CppType = ::transitLib::vm::LevelD;
    using JniType = jobject;

    using Boxed = NativeLevelD;

    ~NativeLevelD();

    static CppType toCpp(JNIEnv* jniEnv, JniType j);
    static ::djinni::LocalRef<JniType> fromCpp(JNIEnv* jniEnv, const CppType& c);

private:
    NativeLevelD();
    friend ::djinni::JniClass<NativeLevelD>;

    const ::djinni::GlobalRef<jclass> clazz { ::djinni::jniFindClass("djinni/java/src/LevelD") };
    const jmethodID jconstructor { ::djinni::jniGetMethodID(clazz.get(), "<init>", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V") };
    const jfieldID field_fieldA { ::djinni::jniGetFieldID(clazz.get(), "fieldA", "Ljava/lang/String;") };
    const jfieldID field_fieldB { ::djinni::jniGetFieldID(clazz.get(), "fieldB", "Ljava/lang/String;") };
    const jfieldID field_fieldC { ::djinni::jniGetFieldID(clazz.get(), "fieldC", "Ljava/lang/String;") };
    const jfieldID field_fieldD { ::djinni::jniGetFieldID(clazz.get(), "fieldD", "Ljava/lang/String;") };
};

} // namespace djinni_generated
