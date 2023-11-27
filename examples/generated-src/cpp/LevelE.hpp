// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

#pragma once

#include "LevelD.hpp"
#include <string>
#include <utility>

namespace textsort {

struct LevelE : public LevelD {
    std::string fieldE;

    virtual ~LevelE(){};

    //NOLINTNEXTLINE(google-explicit-constructor)
    LevelE(std::string fieldA_,
           std::string fieldB_,
           std::string fieldC_,
           std::string fieldD_,
           std::string fieldE_)
    : 
    LevelD( fieldA_,
            fieldB_,
            fieldC_,
            fieldD_), fieldE(std::move(fieldE_)){}
};

} // namespace textsort
