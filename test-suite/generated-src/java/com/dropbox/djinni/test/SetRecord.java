// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from set.djinni

package com.dropbox.djinni.test;

import java.util.HashSet;
import javax.annotation.CheckForNull;
import javax.annotation.Nonnull;

public class SetRecord {


    /*package*/ final HashSet<String> mSet;

    /*package*/ final HashSet<Integer> mIset;

    public SetRecord(
            @Nonnull HashSet<String> set,
            @Nonnull HashSet<Integer> iset) {
        this.mSet = set;
        this.mIset = iset;
    }

    @Nonnull
    public HashSet<String> getSet() {
        return mSet;
    }

    @Nonnull
    public HashSet<Integer> getIset() {
        return mIset;
    }

    @Override
    public boolean equals(@CheckForNull Object obj) {
        if (!(obj instanceof SetRecord)) {
            return false;
        }
        SetRecord other = (SetRecord) obj;
        return this.mSet.equals(other.mSet) &&
                this.mIset.equals(other.mIset);
    }

    @Override
    public int hashCode() {
        // Pick an arbitrary non-zero starting value
        int hashCode = 17;
        hashCode = hashCode * 31 + mSet.hashCode();
        hashCode = hashCode * 31 + mIset.hashCode();
        return hashCode;
    }

    @Override
    public String toString() {
        return "SetRecord{" +
                "mSet=" + mSet +
                "," + "mIset=" + mIset +
        "}";
    }

}
