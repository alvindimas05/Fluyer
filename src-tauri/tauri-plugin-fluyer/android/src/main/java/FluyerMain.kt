package org.alvindimas05.fluyerplugin

import android.app.Activity
import android.content.pm.PackageManager
import android.widget.Toast
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.plugin.Channel
import app.tauri.plugin.JSObject

class FluyerMain {
    fun toast(activity: Activity, value: String) {
        Toast.makeText(activity, value, Toast.LENGTH_SHORT).show()
    }

    fun checkReadAudioPermission(activity: Activity): Boolean {
        return ContextCompat.checkSelfPermission(
            activity,
            android.Manifest.permission.READ_MEDIA_AUDIO
        ) == PackageManager.PERMISSION_GRANTED
    }

    fun requestReadAudioPermission(activity: Activity, channel: Channel) {
        if (checkReadAudioPermission(activity)) {
            val obj = JSObject()
            obj.put("result", true)
            channel.send(obj)
        } else {
            ActivityCompat.requestPermissions(
                activity,
                arrayOf(android.Manifest.permission.READ_MEDIA_AUDIO),
                READ_AUDIO_PERMISSION_REQUEST_CODE
            )
        }
    }

    fun handlePermissionResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray,
        channel: Channel
    ) {
        if (requestCode == READ_AUDIO_PERMISSION_REQUEST_CODE) {
            val isGranted = grantResults.isNotEmpty() && grantResults[0] == PackageManager.PERMISSION_GRANTED
            val obj = JSObject()
            obj.put("result", isGranted)
            channel.send(obj)
        }
    }
}
