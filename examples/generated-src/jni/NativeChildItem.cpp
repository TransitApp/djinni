// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#include "NativeChildItem.hpp"  // my header
#include "Marshal.hpp"

namespace djinni_generated {

NativeChildItem::NativeChildItem() = default;

NativeChildItem::~NativeChildItem() = default;

auto NativeChildItem::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
    const auto& data = ::djinni::JniClass<NativeChildItem>::get();
    auto r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor,
                                                           ::djinni::get(::djinni::List<::djinni::String>::fromCpp(jniEnv, c.items)),
                                                           ::djinni::get(::djinni::String::fromCpp(jniEnv, c.parent)))};
    ::djinni::jniExceptionCheck(jniEnv);
    return r;
}

auto NativeChildItem::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 3);
    assert(j != nullptr);
    const auto& data = ::djinni::JniClass<NativeChildItem>::get();
    ::textsort::ChildItem model;
    model.items = ::djinni::List<::djinni::String>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_items));
    model.parent = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_parent));
    return model;
}

} // namespace djinni_generated
