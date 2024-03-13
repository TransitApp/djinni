// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#pragma once

#include "LevelCViewModel.h"
#include "djinni_support.hpp"

namespace djinni_generated {

class NativeLevelC final {
public:
    using CppType = std::shared_ptr<::transitLib::viewModel::LevelC>;
    using JniType = jobject;

    using Boxed = NativeLevelC;

    ~NativeLevelC();

    static CppType toCpp(JNIEnv* jniEnv, JniType j);
    static ::djinni::LocalRef<JniType> fromCpp(JNIEnv* jniEnv, const CppType& c);

private:
    NativeLevelC();
    friend ::djinni::JniClass<NativeLevelC>;

    const ::djinni::GlobalRef<jclass> clazz { ::djinni::jniFindClass("djinni/java/src/LevelC") };
    const jmethodID jconstructor { ::djinni::jniGetMethodID(clazz.get(), "<init>", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V") };
    const jfieldID field_fieldA { ::djinni::jniGetFieldID(clazz.get(), "fieldA", "Ljava/lang/String;") };
    const jfieldID field_fieldB { ::djinni::jniGetFieldID(clazz.get(), "fieldB", "Ljava/lang/String;") };
    const jfieldID field_fieldC { ::djinni::jniGetFieldID(clazz.get(), "fieldC", "Ljava/lang/String;") };
};

} // namespace djinni_generated
