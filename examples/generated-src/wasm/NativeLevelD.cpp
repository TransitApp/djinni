// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#include "NativeLevelD.hpp"  // my header

namespace djinni_generated {

auto NativeLevelD::toCpp(const JsType& j) -> CppType {
    return {::djinni::String::Boxed::toCpp(j["fieldD"])};
}
auto NativeLevelD::fromCpp(const CppType& c) -> JsType {
    em::val js = em::val::object();
    js.set("fieldD", ::djinni::String::Boxed::fromCpp(c.fieldD));
    return js;
}

} // namespace djinni_generated
