// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from no_constructor.djinni

#import "DBNoConstructorRecord.h"
#include "no_constructor_record_native.hpp"

static_assert(__has_feature(objc_arc), "Djinni requires ARC to be enabled for this file");

@class DBNoConstructorRecord;

namespace djinni_generated {

struct NoConstructorRecord
{
    using CppType = ::testsuite::NoConstructorRecord;
    using ObjcType = DBNoConstructorRecord*;

    using Boxed = NoConstructorRecord;

    static CppType toCpp(ObjcType objc);
    static ObjcType fromCpp(const CppType& cpp);
};

} // namespace djinni_generated
