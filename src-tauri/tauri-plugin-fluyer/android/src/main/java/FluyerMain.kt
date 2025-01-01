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

class FluyerMain(private val activity: Activity) {
    val broadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) {
            val action = intent.action
            if (Intent.ACTION_HEADSET_PLUG == action) {
                val state = intent.getIntExtra("state", -1)
                when (state) {
                    0 -> toast("Headset not plugged in")
                    1 -> toast("Headset plugged in")
                }
            }
        }
    }

    fun listenToHeadsetChange() {
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
}
