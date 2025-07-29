package org.alvindimas05.fluyerplugin.utils

import android.content.Context
import android.net.Uri
import android.os.Build
import android.os.storage.StorageManager
import android.provider.DocumentsContract
import androidx.annotation.RequiresApi
import java.io.File
import java.lang.reflect.Array

object FileUtil {
    private const val PRIMARY_VOLUME_NAME = "primary"

    fun getFullPathFromTreeUri(treeUri: Uri?, con: Context): String? {
        if (treeUri == null) return null
        var volumePath = getVolumePath(getVolumeIdFromTreeUri(treeUri), con)
        if (volumePath == null) return File.separator
        if (volumePath.endsWith(File.separator)) volumePath =
            volumePath.substring(0, volumePath.length - 1)

        var documentPath = getDocumentPathFromTreeUri(treeUri)
        if (documentPath.endsWith(File.separator)) documentPath =
            documentPath.substring(0, documentPath.length - 1)

        if (documentPath.length > 0) {
            if (documentPath.startsWith(File.separator)) return volumePath + documentPath
            else return volumePath + File.separator + documentPath
        } else return volumePath
    }


    private fun getVolumePath(volumeId: String?, context: Context): String? {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP) return null
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) return getVolumePathForAndroid11AndAbove(
            volumeId,
            context
        )
        else return getVolumePathBeforeAndroid11(volumeId, context)
    }


    private fun getVolumePathBeforeAndroid11(volumeId: String?, context: Context): String? {
        try {
            val mStorageManager =
                context.getSystemService(Context.STORAGE_SERVICE) as StorageManager
            val storageVolumeClazz = Class.forName("android.os.storage.StorageVolume")
            val getVolumeList = mStorageManager.javaClass.getMethod("getVolumeList")
            val getUuid = storageVolumeClazz.getMethod("getUuid")
            val getPath = storageVolumeClazz.getMethod("getPath")
            val isPrimary = storageVolumeClazz.getMethod("isPrimary")
            val result = getVolumeList.invoke(mStorageManager)

            val length: Int = java.lang.reflect.Array.getLength(result)
            for (i in 0 until length) {
                val storageVolumeElement: Any? = java.lang.reflect.Array.get(result, i)
                val uuid = getUuid.invoke(storageVolumeElement) as String?
                val primary = isPrimary.invoke(storageVolumeElement) as Boolean

                if (primary && PRIMARY_VOLUME_NAME == volumeId)  // primary volume?
                    return getPath.invoke(storageVolumeElement) as String?

                if (uuid != null && uuid == volumeId)  // other volumes?
                    return getPath.invoke(storageVolumeElement) as String?
            }
            // not found.
            return null
        } catch (ex: Exception) {
            return null
        }
    }

    @RequiresApi(Build.VERSION_CODES.R)
    private fun getVolumePathForAndroid11AndAbove(volumeId: String?, context: Context): String? {
        try {
            val mStorageManager =
                context.getSystemService(Context.STORAGE_SERVICE) as StorageManager
            val storageVolumes = mStorageManager.getStorageVolumes()
            for (storageVolume in storageVolumes) {
                // primary volume?
                if (storageVolume.isPrimary() && PRIMARY_VOLUME_NAME == volumeId) return storageVolume.getDirectory()!!
                    .getPath()

                // other volumes?
                val uuid = storageVolume.getUuid()
                if (uuid != null && uuid == volumeId) return storageVolume.getDirectory()!!
                    .getPath()
            }
            // not found.
            return null
        } catch (ex: Exception) {
            return null
        }
    }

    private fun getVolumeIdFromTreeUri(treeUri: Uri?): String? {
        val docId = DocumentsContract.getTreeDocumentId(treeUri)
        val split = docId.split(":".toRegex()).dropLastWhile { it.isEmpty() }.toTypedArray()
        if (split.isNotEmpty()) return split[0]
        else return null
    }


    private fun getDocumentPathFromTreeUri(treeUri: Uri?): String {
        val docId = DocumentsContract.getTreeDocumentId(treeUri)
        val split = docId.split(":".toRegex()).dropLastWhile { it.isEmpty() }.toTypedArray()
        if (split.size >= 2) return split[1]
        else return File.separator
    }
}