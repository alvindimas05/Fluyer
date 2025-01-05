package org.alvindimas05.fluyerplugin

import android.content.Context
import android.graphics.Point
import android.os.Build
import android.view.Display
import android.view.WindowManager
import android.util.TypedValue
import kotlin.math.roundToInt
import android.view.WindowInsets
import android.content.res.Resources
import android.content.res.Configuration
import android.graphics.Rect
import android.app.Activity
import android.widget.Toast
import android.content.BroadcastReceiver
import android.content.Intent
import android.content.IntentFilter
import app.tauri.plugin.JSObject
import app.tauri.plugin.Channel
import android.util.Log
import java.lang.Runtime

class FluyerMain(private val activity: Activity) {
    var headsetChangeChannel: Channel? = null

    val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) {
            if (Intent.ACTION_HEADSET_PLUG == intent.action) {
                headsetChangeChannel?.let {
                    it.send(JSObject().put("value", intent.getIntExtra("state", 0) == 1))
                }
            }
        }
    }

    fun watchHeadsetChange() {
        val receiverFilter = IntentFilter(Intent.ACTION_HEADSET_PLUG)
        activity.registerReceiver(broadcastReceiver, receiverFilter)
    }
    fun toast(value: String) {
        activity.runOnUiThread {
            Toast.makeText(activity, value, Toast.LENGTH_SHORT).show()
        }
    }
    fun dpToPx(value: Int): Int {
        return (value / activity.resources.displayMetrics.density).roundToInt()
    }
    fun getStatusBarHeight(): Int {
        return dpToPx(activity.window.decorView.rootWindowInsets
            .getInsets(WindowInsets.Type.statusBars()).top)
    }
    fun getNavigationBarHeight(): Int {
        val resources: Resources = activity.resources

        val resName = if (resources.configuration.orientation == Configuration.ORIENTATION_PORTRAIT) {
            "navigation_bar_height"
        } else {
            "navigation_bar_height_landscape"
        }

        val id: Int = resources.getIdentifier(resName, "dimen", "android")

        return if (id > 0) {
            dpToPx(resources.getDimensionPixelSize(id))
        } else {
            0
        }
    }
    
    fun restartApp() {
        val context = activity.applicationContext
        val intent = context.packageManager.getLaunchIntentForPackage(context.packageName)
        val mainIntent = Intent.makeRestartActivityTask(intent!!.component)
        context.startActivity(mainIntent)
        Runtime.getRuntime().exit(0);
    }
}
