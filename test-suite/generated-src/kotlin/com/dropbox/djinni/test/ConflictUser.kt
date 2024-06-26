// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from test.djinni

package com.dropbox.djinni.test

import com.snapchat.djinni.NativeObjectManager
import java.util.HashSet
import java.util.concurrent.atomic.AtomicBoolean

abstract class ConflictUser {
    abstract fun Conflict(): Conflict

    abstract fun conflictArg(cs: HashSet<Conflict>): Boolean

    class CppProxy(private val nativeRef: Long) : ConflictUser() {
        private val destroyed = AtomicBoolean(false)

        init {
            if (nativeRef == 0L) throw RuntimeException("nativeRef is zero")
            NativeObjectManager.register(this, nativeRef)
        }

        companion object {
            @kotlin.jvm.JvmStatic
            external fun nativeDestroy(nativeRef: Long)
        }

        override fun Conflict(): Conflict {
            assert(!destroyed.get()) { "trying to use a destroyed object" }
            return native_Conflict(nativeRef)
        }
        private external fun native_Conflict(_nativeRef: Long): Conflict

        override fun conflictArg(cs: HashSet<Conflict>): Boolean {
            assert(!destroyed.get()) { "trying to use a destroyed object" }
            return native_conflictArg(nativeRef, cs)
        }
        private external fun native_conflictArg(_nativeRef: Long, cs: HashSet<Conflict>): Boolean
    }
}
