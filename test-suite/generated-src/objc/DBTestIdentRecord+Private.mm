// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from ident_explicit.djinni

#import "DBTestIdentRecord+Private.h"
#import "DJIMarshal+Private.h"
#include <cassert>

namespace djinni_generated {

auto TestIdentRecord::toCpp(ObjcType obj) -> CppType
{
    assert(obj);
    ::testsuite::TestIdentRecord model;
    model.mFirstValue = ::djinni::I32::toCpp(obj.FirstValue);
    model.mSecondValue = ::djinni::String::toCpp(obj.secondValue);
    return model;
}

auto TestIdentRecord::fromCpp(const CppType& cpp) -> ObjcType
{
    return [[DBTestIdentRecord alloc] initWithFirstValue:(::djinni::I32::fromCpp(cpp.FirstValue))
                                             secondValue:(::djinni::String::fromCpp(cpp.second_value))];
}

} // namespace djinni_generated
