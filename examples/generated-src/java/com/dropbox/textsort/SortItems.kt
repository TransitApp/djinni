// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from example.djinni

package com.dropbox.textsort;

import com.snapchat.djinni.NativeObjectManager;
import java.util.concurrent.atomic.AtomicBoolean;

internal abstract class SortItems {
    /** For the iOS / Android demo */
    abstract fun sort(order: SortOrder, items: ItemList)

    companion object {

        @kotlin.jvm.JvmStatic
        external fun createWithListener(listener: TextboxListener): SortItems

        /** For the localhost / command-line demo */
        @kotlin.jvm.JvmStatic
        external fun runSort(items: ItemList): ItemList
    }

    class CppProxy(private val nativeRef: Long) : SortItems() {
        private val destroyed = AtomicBoolean(false)

        init {
            if (nativeRef == 0L) throw RuntimeException("nativeRef is zero")
            NativeObjectManager.register(this, nativeRef)
        }

        companion object {
            @kotlin.jvm.JvmStatic
            external fun nativeDestroy(nativeRef: Long)
        }

        override fun sort(order: SortOrder, items: ItemList) {
            assert(!destroyed.get()) { "trying to use a destroyed object" }
            native_sort(nativeRef, order, items)
        }
        private external fun native_sort(_nativeRef: Long, order: SortOrder, items: ItemList)
    }
}