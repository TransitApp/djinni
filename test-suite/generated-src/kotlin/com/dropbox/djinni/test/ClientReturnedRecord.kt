// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from client_interface.djinni

package com.dropbox.djinni.test

/** Record returned by a client */
data class ClientReturnedRecord(
    val mRecordId: Long,
    val mContent: String,
    val mMisc: String?
) {

    override fun toString(): String  {
        return "ClientReturnedRecord {" +
                "mRecordId=" + mRecordId +
                "," + "mContent=" + mContent +
                "," + "mMisc=" + mMisc +
        "}"
    }

}
