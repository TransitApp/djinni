// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from constant_enum.djinni

#import "DBConstantWithEnum+Private.h"
#import "DBConstantEnum+Private.h"
#include <cassert>

namespace djinni_generated {

auto ConstantWithEnum::toCpp(ObjcType obj) -> CppType
{
    assert(obj);
    (void)obj; // Suppress warnings in relase builds for empty records
    ::testsuite::ConstantWithEnum model;
    return model;
}

auto ConstantWithEnum::fromCpp(const CppType& cpp) -> ObjcType
{
    (void)cpp; // Suppress warnings in relase builds for empty records
    return [[DBConstantWithEnum alloc] init];
}

} // namespace djinni_generated
