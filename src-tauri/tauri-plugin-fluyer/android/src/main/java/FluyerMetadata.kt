package org.alvindimas05.fluyerplugin

import android.content.Context
import android.util.Log
import app.tauri.plugin.JSObject
import com.arthenica.ffmpegkit.FFmpegKit
import com.arthenica.ffmpegkit.FFprobeKit
import com.arthenica.ffmpegkit.ReturnCode
import org.json.JSONObject
import java.io.File

object FluyerMetadata {
    private var cacheDir: File? = null
    
    fun initialize(context: Context) {
        cacheDir = context.cacheDir
    }

    fun getMetadata(path: String): String? {
        try {
            val session = FFprobeKit.getMediaInformation(path)
            val info = session.mediaInformation ?: return null
            return info.allProperties?.toString()
        } catch (e: Exception) {
            Log.e(LOG_TAG, "Failed to get metadata: ${e.message}")
            return null
        }
    }
    

    
    fun getImage(path: String): String? {
        try {
            val tempFile = File(cacheDir, "cover_${System.currentTimeMillis()}.jpg")
            val result = java.util.concurrent.atomic.AtomicReference<String?>(null)
            val latch = java.util.concurrent.CountDownLatch(1)
            
            // Try to copy embedded image directly first (async)
            val copyArgs = "-i \"$path\" -an -c:v copy -vframes 1 -y \"${tempFile.absolutePath}\""
            FFmpegKit.executeAsync(copyArgs) { session ->
                if (session.returnCode.value == ReturnCode.SUCCESS && tempFile.exists() && tempFile.length() > 0L) {
                    result.set(tempFile.absolutePath)
                    latch.countDown()
                } else {
                    // Fallback: re-encode as JPEG (async)
                    tempFile.delete()
                    val encodeArgs = "-i \"$path\" -an -c:v mjpeg -vframes 1 -y \"${tempFile.absolutePath}\""
                    FFmpegKit.executeAsync(encodeArgs) { fallbackSession ->
                        if (fallbackSession.returnCode.value == ReturnCode.SUCCESS && tempFile.exists() && tempFile.length() > 0L) {
                            result.set(tempFile.absolutePath)
                        } else {
                            tempFile.delete()
                        }
                        latch.countDown()
                    }
                }
            }
            
            // Wait for async operation to complete (with timeout)
            latch.await(10, java.util.concurrent.TimeUnit.SECONDS)
            return result.get()
        } catch (e: Exception) {
            Log.e(LOG_TAG, "Failed to get image: ${e.message}")
            return null
        }
    }
}
