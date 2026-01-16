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
            // First check if the file has a video stream (cover art)
            val probeSession = FFprobeKit.getMediaInformation(path)
            val info = probeSession.mediaInformation ?: return null
            
            var hasVideoStream = false
            for (stream in info.streams) {
                if (stream.type == "video") {
                    hasVideoStream = true
                    break
                }
            }
            
            if (!hasVideoStream) {
                return null
            }
            
            // Create temp file for cover art
            val tempFile = File(cacheDir, "cover_${System.currentTimeMillis()}.jpg")
            
            // Try to copy embedded image directly first
            val copyArgs = "-i \"$path\" -an -c:v copy -vframes 1 -y \"${tempFile.absolutePath}\""
            var session = FFmpegKit.execute(copyArgs)
            
            if (session.returnCode.value != ReturnCode.SUCCESS || !tempFile.exists() || tempFile.length() == 0L) {
                // Fallback: re-encode as JPEG
                tempFile.delete()
                val encodeArgs = "-i \"$path\" -an -c:v mjpeg -vframes 1 -y \"${tempFile.absolutePath}\""
                session = FFmpegKit.execute(encodeArgs)
                
                if (session.returnCode.value != ReturnCode.SUCCESS || !tempFile.exists() || tempFile.length() == 0L) {
                    tempFile.delete()
                    return null
                }
            }
            
            // Return the temp file path - Rust will read and clean it up
            return tempFile.absolutePath
        } catch (e: Exception) {
            Log.e(LOG_TAG, "Failed to get image: ${e.message}")
            return null
        }
    }
}
