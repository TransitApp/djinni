// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from proto.djinni

#import "DBRecordWithEmbeddedProto+Private.h"
#import "DJIMarshal+Private.h"
#import "proto/objc/test.pbobjc.h"
#include <cassert>

namespace djinni_generated {

auto RecordWithEmbeddedProto::toCpp(ObjcType obj) -> CppType
{
    assert(obj);
    ::testsuite::RecordWithEmbeddedProto model;
    model.mPerson = ::djinni::Protobuf<::djinni::test::Person, DJTestPerson>::toCpp(obj.person);
    return model;
}

auto RecordWithEmbeddedProto::fromCpp(const CppType& cpp) -> ObjcType
{
    return [[DBRecordWithEmbeddedProto alloc] initWithPerson:(::djinni::Protobuf<::djinni::test::Person, DJTestPerson>::fromCpp(cpp.person))];
}

} // namespace djinni_generated
