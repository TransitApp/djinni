// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#pragma once

#include "child_item.hpp"
#include <cstdint>
#include <utility>

namespace textsort {

struct SubChildItem : public ChildItem {
    int32_t index;

    //NOLINTNEXTLINE(google-explicit-constructor)
    SubChildItem(std::vector<std::string> items_,
                 std::string parent_,
                 int32_t index_)
    : 
    ChildItem( items_,
               parent_), index(std::move(index_)){}
};

} // namespace textsort