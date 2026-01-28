package org.alvindimas05.fluyerplugin

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log

class MediaControlReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        val action = intent.action ?: return
        Log.d(LOG_TAG, "MediaControlReceiver received action: $action")
        
        pendingCallback?.invoke(action)
    }

    companion object {
        const val ACTION_PLAY = "org.alvindimas05.fluyerplugin.ACTION_PLAY"
        const val ACTION_PAUSE = "org.alvindimas05.fluyerplugin.ACTION_PAUSE"
        const val ACTION_PREVIOUS = "org.alvindimas05.fluyerplugin.ACTION_PREVIOUS"
        const val ACTION_NEXT = "org.alvindimas05.fluyerplugin.ACTION_NEXT"
        const val ACTION_STOP = "org.alvindimas05.fluyerplugin.ACTION_STOP"

        var pendingCallback: ((String) -> Unit)? = null
    }
}
