// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#import "SPLevelAViewModel.h"
#include "LevelAViewModel.h"

static_assert(__has_feature(objc_arc), "Djinni requires ARC to be enabled for this file");

@class SPLevelAViewModel;

namespace djinni_generated {

struct LevelA
{
    using CppType = ::transitLib::vm::LevelA;
    using ObjcType = SPLevelAViewModel*;

    using Boxed = LevelA;

    static CppType toCpp(ObjcType objc);
    static ObjcType fromCpp(const CppType& cpp);
};

} // namespace djinni_generated
