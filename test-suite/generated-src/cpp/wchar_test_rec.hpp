// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from wchar_test.djinni

#pragma once

#include <string>
#include <utility>

namespace testsuite {

struct WcharTestRec final {
    std::wstring s;

    friend bool operator==(const WcharTestRec& lhs, const WcharTestRec& rhs);
    friend bool operator!=(const WcharTestRec& lhs, const WcharTestRec& rhs);

    //NOLINTNEXTLINE(google-explicit-constructor)
    WcharTestRec(std::wstring s_)
    : s(std::move(s_))
    {}
};

} // namespace testsuite
