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

class FluyerMain(private val activity: Activity) {
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