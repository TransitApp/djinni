// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from constants.djinni

package com.dropbox.djinni.test;

/** Record containing constants */
class Constants {
    companion object {
        /** bool_constant has documentation. */
        val BOOL_CONSTANT: Boolean = true
        val I8_CONSTANT: Byte = 1
        val I16_CONSTANT: Short = 2
        val I32_CONSTANT: Int = 3
        val I64_CONSTANT: Long = 4L
        val F32_CONSTANT: Float = 5.0F
        /**
         * f64_constant has long documentation.
         * (Second line of multi-line documentation.
         *   Indented third line of multi-line documentation.)
         */
        val F64_CONSTANT: Double = 5.0
        val OPT_BOOL_CONSTANT: Boolean? = true
        val OPT_I8_CONSTANT: Byte? = 1
        /** opt_i16_constant has documentation. */
        val OPT_I16_CONSTANT: Short? = 2
        val OPT_I32_CONSTANT: Int? = 3
        val OPT_I64_CONSTANT: Long? = 4L
        /**
         * opt_f32_constant has long documentation.
         * (Second line of multi-line documentation.
         *   Indented third line of multi-line documentation.)
         */
        val OPT_F32_CONSTANT: Float? = 5.0F
        val OPT_F64_CONSTANT: Double? = 5.0
        val STRING_CONSTANT: String = "string-constant"
        val OPT_STRING_CONSTANT: String? = "string-constant"
        val OBJECT_CONSTANT: ConstantRecord =  ConstantRecord(
            I32_CONSTANT /* mSomeInteger */ ,
            STRING_CONSTANT /* mSomeString */ )
        /**
         * No support for null optional constants
         * No support for optional constant records
         * No support for constant binary, list, set, map
         */
        val DUMMY: Boolean = false
    }

    override fun toString(): String  {
        return "Constants {" +
        "}"
    }

}