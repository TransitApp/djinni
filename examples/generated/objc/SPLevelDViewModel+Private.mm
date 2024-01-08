// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#import "SPLevelD2ViewModel+Private.h"
#import "SPLevelDViewModel+Private.h"
#import "SPLevelEViewModel+Private.h"
#import "SPLevelFViewModel+Private.h"
#import "DJIMarshal+Private.h"
#include <cassert>

namespace djinni_generated {

auto LevelD::toCpp(ObjcType obj) -> CppType
{
    assert(obj);
    ::transitLib::viewModel::LevelD model;
    model.fieldA = ::djinni::String::toCpp(obj.fieldA);
    model.fieldB = ::djinni::String::toCpp(obj.fieldB);
    model.fieldC = ::djinni::String::toCpp(obj.fieldC);
    model.fieldD = ::djinni::String::toCpp(obj.fieldD);
    return model;
}

auto LevelD::fromCpp(const CppType& cpp) -> ObjcType
{
    return [[SPLevelDViewModel alloc] initWithFieldA:(::djinni::String::fromCpp(cpp.fieldA))
                                              fieldB:(::djinni::String::fromCpp(cpp.fieldB))
                                              fieldC:(::djinni::String::fromCpp(cpp.fieldC))
                                              fieldD:(::djinni::String::fromCpp(cpp.fieldD))];
}

} // namespace djinni_generated
