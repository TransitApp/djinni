// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from primtypes.djinni

#import "DBAssortedPrimitives+Private.h"
#import "DJIMarshal+Private.h"
#include <cassert>

namespace djinni_generated {

auto AssortedPrimitives::toCpp(ObjcType obj) -> CppType
{
    assert(obj);
    ::testsuite::AssortedPrimitives model;
    model.mB = ::djinni::Bool::toCpp(obj.b);
    model.mEight = ::djinni::I8::toCpp(obj.eight);
    model.mSixteen = ::djinni::I16::toCpp(obj.sixteen);
    model.mThirtytwo = ::djinni::I32::toCpp(obj.thirtytwo);
    model.mSixtyfour = ::djinni::I64::toCpp(obj.sixtyfour);
    model.mFthirtytwo = ::djinni::F32::toCpp(obj.fthirtytwo);
    model.mFsixtyfour = ::djinni::F64::toCpp(obj.fsixtyfour);
    model.mOB = ::djinni::Optional<std::experimental::optional, ::djinni::Bool>::toCpp(obj.oB);
    model.mOEight = ::djinni::Optional<std::experimental::optional, ::djinni::I8>::toCpp(obj.oEight);
    model.mOSixteen = ::djinni::Optional<std::experimental::optional, ::djinni::I16>::toCpp(obj.oSixteen);
    model.mOThirtytwo = ::djinni::Optional<std::experimental::optional, ::djinni::I32>::toCpp(obj.oThirtytwo);
    model.mOSixtyfour = ::djinni::Optional<std::experimental::optional, ::djinni::I64>::toCpp(obj.oSixtyfour);
    model.mOFthirtytwo = ::djinni::Optional<std::experimental::optional, ::djinni::F32>::toCpp(obj.oFthirtytwo);
    model.mOFsixtyfour = ::djinni::Optional<std::experimental::optional, ::djinni::F64>::toCpp(obj.oFsixtyfour);
    return model;
}

auto AssortedPrimitives::fromCpp(const CppType& cpp) -> ObjcType
{
    return [[DBAssortedPrimitives alloc] initWithB:(::djinni::Bool::fromCpp(cpp.b))
                                             eight:(::djinni::I8::fromCpp(cpp.eight))
                                           sixteen:(::djinni::I16::fromCpp(cpp.sixteen))
                                         thirtytwo:(::djinni::I32::fromCpp(cpp.thirtytwo))
                                         sixtyfour:(::djinni::I64::fromCpp(cpp.sixtyfour))
                                        fthirtytwo:(::djinni::F32::fromCpp(cpp.fthirtytwo))
                                        fsixtyfour:(::djinni::F64::fromCpp(cpp.fsixtyfour))
                                                oB:(::djinni::Optional<std::experimental::optional, ::djinni::Bool>::fromCpp(cpp.o_b))
                                            oEight:(::djinni::Optional<std::experimental::optional, ::djinni::I8>::fromCpp(cpp.o_eight))
                                          oSixteen:(::djinni::Optional<std::experimental::optional, ::djinni::I16>::fromCpp(cpp.o_sixteen))
                                        oThirtytwo:(::djinni::Optional<std::experimental::optional, ::djinni::I32>::fromCpp(cpp.o_thirtytwo))
                                        oSixtyfour:(::djinni::Optional<std::experimental::optional, ::djinni::I64>::fromCpp(cpp.o_sixtyfour))
                                       oFthirtytwo:(::djinni::Optional<std::experimental::optional, ::djinni::F32>::fromCpp(cpp.o_fthirtytwo))
                                       oFsixtyfour:(::djinni::Optional<std::experimental::optional, ::djinni::F64>::fromCpp(cpp.o_fsixtyfour))];
}

} // namespace djinni_generated
