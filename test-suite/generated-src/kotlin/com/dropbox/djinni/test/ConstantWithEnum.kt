// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from constant_enum.djinni

package com.dropbox.djinni.test

/** Record containing enum constant */
open class ConstantWithEnum {
    companion object {
        val CONST_ENUM: ConstantEnum = ConstantEnum.SomeValue
    }

    override fun equals(other: Any?): Boolean  {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as ConstantWithEnum


        return true
    }

    override fun hashCode(): Int  {
        // Pick an arbitrary non-zero starting value
        var hashCode = 17;
        return hashCode
    }

    override fun toString(): String  {
        return "ConstantWithEnum {" +
        "}"
    }

}
