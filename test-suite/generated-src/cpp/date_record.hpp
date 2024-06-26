// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from date.djinni

#pragma once

#include <chrono>
#include <memory>
#include <utility>

namespace testsuite {

struct DateRecord final {
    std::chrono::system_clock::time_point created_at;

    friend bool operator==(const DateRecord& lhs, const DateRecord& rhs);
    friend bool operator!=(const DateRecord& lhs, const DateRecord& rhs);

    friend bool operator<(const DateRecord& lhs, const DateRecord& rhs);
    friend bool operator>(const DateRecord& lhs, const DateRecord& rhs);

    friend bool operator<=(const DateRecord& lhs, const DateRecord& rhs);
    friend bool operator>=(const DateRecord& lhs, const DateRecord& rhs);

    //NOLINTNEXTLINE(google-explicit-constructor)
    DateRecord(std::chrono::system_clock::time_point created_at_)
    : created_at(std::move(created_at_))
    {}
};

} // namespace testsuite
