// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from map.djinni

package com.dropbox.djinni.test

import java.util.HashMap

data class MapRecord(
    val mMap: HashMap<String, Long>,
    val mImap: HashMap<Int, Int>
) {

    override fun equals(other: Any?): Boolean  {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as MapRecord

        if (mMap != other.mMap) return false
        if (mImap != other.mImap) return false

        return true
    }

    override fun hashCode(): Int  {
        // Pick an arbitrary non-zero starting value
        var hashCode = 17;
        hashCode = hashCode * 31 + mMap.hashCode()
        hashCode = hashCode * 31 + mImap.hashCode()
        return hashCode
    }

    override fun toString(): String  {
        return "MapRecord {" +
                "mMap=" + mMap +
                "," + "mImap=" + mImap +
        "}"
    }

}
