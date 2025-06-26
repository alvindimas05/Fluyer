package org.alvindimas05.fluyerplugin

import android.os.Build
import kotlin.math.roundToInt
import android.view.WindowInsets
import android.content.res.Resources
import android.content.res.Configuration
import android.app.Activity
import android.widget.Toast
import android.content.Intent
import java.lang.Runtime

class FluyerMain(private val activity: Activity) {
    fun toast(value: String) {
        activity.runOnUiThread {
            Toast.makeText(activity, value, Toast.LENGTH_SHORT).show()
        }
    }
    private fun dpToPx(value: Int): Int {
        return (value / activity.resources.displayMetrics.density).roundToInt()
    }
    fun getStatusBarHeight(): Int {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R) {
            val resourceId = activity.resources.getIdentifier("status_bar_height", "dimen", "android")
            return if (resourceId > 0) {
                dpToPx(activity.resources.getDimensionPixelSize(resourceId))
            } else {
                0
            }
        }
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
