// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from no_constructor.djinni

package com.dropbox.djinni.test

data class NoConstructorRecordNative(
    val mFirstValue: Int,
    val mSecondValue: String
) {
    companion object {
        val RECORD_CONST_VALUE: String = "test"
        /** Weirdness in casing should be preserved: this should become XXXWeirdCase in CamelCase */
        val XXXWEIRD_CASE: Int = 1
    }

    override fun toString(): String  {
        return "NoConstructorRecordNative {" +
                "mFirstValue=" + mFirstValue +
                "," + "mSecondValue=" + mSecondValue +
        "}"
    }

}
