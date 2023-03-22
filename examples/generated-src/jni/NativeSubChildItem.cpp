// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#include "NativeSubChildItem.hpp"  // my header
#include "Marshal.hpp"

namespace djinni_generated {

NativeSubChildItem::NativeSubChildItem() = default;

NativeSubChildItem::~NativeSubChildItem() = default;

auto NativeSubChildItem::fromCpp(JNIEnv* jniEnv, const CppType& c) -> ::djinni::LocalRef<JniType> {
    const auto& data = ::djinni::JniClass<NativeSubChildItem>::get();
    auto r = ::djinni::LocalRef<JniType>{jniEnv->NewObject(data.clazz.get(), data.jconstructor,
                                                           ::djinni::get(::djinni::List<::djinni::String>::fromCpp(jniEnv, c.items)),
                                                           ::djinni::get(::djinni::String::fromCpp(jniEnv, c.parent)),
                                                           ::djinni::get(::djinni::I32::fromCpp(jniEnv, c.index)))};
    ::djinni::jniExceptionCheck(jniEnv);
    return r;
}

auto NativeSubChildItem::toCpp(JNIEnv* jniEnv, JniType j) -> CppType {
    ::djinni::JniLocalScope jscope(jniEnv, 4);
    assert(j != nullptr);
    const auto& data = ::djinni::JniClass<NativeSubChildItem>::get();
    ::textsort::SubChildItem model;
    model.items = ::djinni::List<::djinni::String>::toCpp(jniEnv, jniEnv->GetObjectField(j, data.field_items));
    model.parent = ::djinni::String::toCpp(jniEnv, (jstring)jniEnv->GetObjectField(j, data.field_parent));
    model.index = ::djinni::I32::toCpp(jniEnv, jniEnv->GetIntField(j, data.field_index));
    return model;
}

} // namespace djinni_generated
