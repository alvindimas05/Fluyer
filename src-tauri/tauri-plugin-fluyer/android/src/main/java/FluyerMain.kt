package org.alvindimas05.fluyerplugin

import android.app.Activity
import android.os.Build
import kotlin.math.roundToInt
import android.view.WindowInsets
import android.content.res.Resources
import android.content.res.Configuration
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
        return dpToPx(if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            val insets = activity.window.decorView.rootWindowInsets
            insets?.getInsets(WindowInsets.Type.statusBars())?.top ?: 0
        } else {
            val resourceId = activity.resources.getIdentifier("status_bar_height", "dimen", "android")
            if (resourceId > 0) {
                activity.resources.getDimensionPixelSize(resourceId) // already px
            } else {
                0
            }
        })
    }

    fun getNavigationBarHeight(): Int {
        return dpToPx(if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            val insets = activity.window.decorView.rootWindowInsets
            insets?.getInsets(WindowInsets.Type.navigationBars())?.bottom ?: 0
        } else {
            val resources = activity.resources
            val resName = if (resources.configuration.orientation == Configuration.ORIENTATION_PORTRAIT) {
                "navigation_bar_height"
            } else {
                "navigation_bar_height_landscape"
            }
            val id = resources.getIdentifier(resName, "dimen", "android")
            if (id > 0) resources.getDimensionPixelSize(id) else 0
        })
    }

    fun restartApp() {
        val context = activity.applicationContext
        val intent = context.packageManager.getLaunchIntentForPackage(context.packageName)
        val mainIntent = Intent.makeRestartActivityTask(intent!!.component)
        context.startActivity(mainIntent)
        Runtime.getRuntime().exit(0)
    }
}
