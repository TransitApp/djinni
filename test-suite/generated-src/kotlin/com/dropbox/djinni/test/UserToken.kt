// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from user_token.djinni

package com.dropbox.djinni.test

import com.snapchat.djinni.NativeObjectManager
import java.util.concurrent.atomic.AtomicBoolean

abstract class UserToken {
    abstract fun whoami(): String

    class CppProxy(private val nativeRef: Long) : UserToken() {
        private val destroyed = AtomicBoolean(false)

        init {
            if (nativeRef == 0L) throw RuntimeException("nativeRef is zero")
            NativeObjectManager.register(this, nativeRef)
        }

        companion object {
            @kotlin.jvm.JvmStatic
            external fun nativeDestroy(nativeRef: Long)
        }

        override fun whoami(): String {
            assert(!destroyed.get()) { "trying to use a destroyed object" }
            return native_whoami(nativeRef)
        }
        private external fun native_whoami(_nativeRef: Long): String
    }
}
