// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from no_constructor.djinni

#import "DBNoConstructorRecord+Private.h"
#import "DJIMarshal+Private.h"
#include <cassert>

namespace djinni_generated {

auto NoConstructorRecord::toCpp(ObjcType obj) -> CppType
{
    assert(obj);
    return {::djinni::I32::toCpp(obj.FirstValue),
            ::djinni::String::toCpp(obj.secondValue)};
}

auto NoConstructorRecord::fromCpp(const CppType& cpp) -> ObjcType
{
    return [[DBNoConstructorRecord alloc] initWithFirstValue:(::djinni::I32::fromCpp(cpp.FirstValue))
                                                 secondValue:(::djinni::String::fromCpp(cpp.second_value))];
}

} // namespace djinni_generated