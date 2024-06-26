// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from constants.djinni

package com.dropbox.djinni.test

import com.snapchat.djinni.NativeObjectManager
import java.util.concurrent.atomic.AtomicBoolean

/** Interface containing constants */
abstract class ConstantsInterface {
    companion object {
        val BOOL_CONSTANT: Boolean = true
        val I8_CONSTANT: Byte = 1
        val I16_CONSTANT: Short = 2
        /** i32_constant has documentation. */
        val I32_CONSTANT: Int = 3
        /**
         * i64_constant has long documentation.
         * (Second line of multi-line documentation.
         *   Indented third line of multi-line documentation.)
         */
        val I64_CONSTANT: Long = 4L
        val F32_CONSTANT: Float = 5.0F
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
         * This constant will not be generated correctly with style FooBar
         * to get it correct we would have to use "FooBar!" (see ident_explicit)
         */
        val UPPER_CASE_CONSTANT: String = "upper-case-constant"
    }
    /**
     * No support for null optional constants
     * No support for optional constant records
     * No support for constant binary, list, set, map
     */
    abstract fun dummy()

    class CppProxy(private val nativeRef: Long) : ConstantsInterface() {
        private val destroyed = AtomicBoolean(false)

        init {
            if (nativeRef == 0L) throw RuntimeException("nativeRef is zero")
            NativeObjectManager.register(this, nativeRef)
        }

        companion object {
            @kotlin.jvm.JvmStatic
            external fun nativeDestroy(nativeRef: Long)
        }

        override fun dummy() {
            assert(!destroyed.get()) { "trying to use a destroyed object" }
            native_dummy(nativeRef)
        }
        private external fun native_dummy(_nativeRef: Long)
    }
}
