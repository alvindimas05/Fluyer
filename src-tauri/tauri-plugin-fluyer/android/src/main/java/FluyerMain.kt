package org.alvindimas05.fluyerplugin

import android.app.Activity
import android.widget.Toast

class FluyerMain {
    fun toast(activity: Activity, value: String) {
        Toast.makeText(activity, value, Toast.LENGTH_SHORT).show()
    }
}
