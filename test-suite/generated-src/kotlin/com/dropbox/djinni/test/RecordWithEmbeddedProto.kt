// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from proto.djinni

package com.dropbox.djinni.test

import djinni.test.Test.Person

data class RecordWithEmbeddedProto(
    val mPerson: Person
) {

    override fun toString(): String  {
        return "RecordWithEmbeddedProto {" +
                "mPerson=" + mPerson +
        "}"
    }

}
