// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from outcome.djinni

#import "DBNestedOutcome+Private.h"
#import "DJIMarshal+Private.h"
#import "Outcome_objc.hpp"
#include <cassert>

namespace djinni_generated {

auto NestedOutcome::toCpp(ObjcType obj) -> CppType
{
    assert(obj);
    ::testsuite::NestedOutcome model;
    model.mO = ::djinni::Outcome<::djinni::I32, ::djinni::String>::toCpp(obj.o);
    return model;
}

auto NestedOutcome::fromCpp(const CppType& cpp) -> ObjcType
{
    return [[DBNestedOutcome alloc] initWithO:(::djinni::Outcome<::djinni::I32, ::djinni::String>::fromCpp(cpp.o))];
}

} // namespace djinni_generated
