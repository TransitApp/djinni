// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from extended_record.djinni

package com.dropbox.djinni.test;

/** Extended record */
data class ExtendedRecord(
    val mFoo: Boolean,
) {
    companion object {
        val EXTENDED_RECORD_CONST: ExtendedRecord =  ExtendedRecord(
            true /* mFoo */ )
    }

    override fun toString(): String  {
        return "ExtendedRecord {" +
                "mFoo=" + mFoo +
        "}"
    }

}