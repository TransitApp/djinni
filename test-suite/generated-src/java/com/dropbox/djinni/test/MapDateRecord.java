// AUTOGENERATED FILE - DO NOT MODIFY!
// This file was generated by Djinni from date.djinni

package com.dropbox.djinni.test;

import java.util.HashMap;
import javax.annotation.CheckForNull;
import javax.annotation.Nonnull;

public class MapDateRecord {


    /*package*/ final HashMap<String, java.util.Date> mDatesById;

    public MapDateRecord(
            @Nonnull HashMap<String, java.util.Date> datesById) {
        this.mDatesById = datesById;
    }

    @Nonnull
    public HashMap<String, java.util.Date> getDatesById() {
        return mDatesById;
    }

    @Override
    public boolean equals(@CheckForNull Object obj) {
        if (!(obj instanceof MapDateRecord)) {
            return false;
        }
        MapDateRecord other = (MapDateRecord) obj;
        return this.mDatesById.equals(other.mDatesById);
    }

    @Override
    public int hashCode() {
        // Pick an arbitrary non-zero starting value
        int hashCode = 17;
        hashCode = hashCode * 31 + mDatesById.hashCode();
        return hashCode;
    }

    @Override
    public String toString() {
        return "MapDateRecord{" +
                "mDatesById=" + mDatesById +
        "}";
    }

}
