package org.alvindimas05.fluyerplugin

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class ToastArgs {
  var value: String? = null
}

@TauriPlugin
class FluyerPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = FluyerMain()

    @Command
    fun toast(invoke: Invoke) {
        val args = invoke.parseArgs(ToastArgs::class.java)
        implementation.toast(activity, args.value ?: "This is Toast")
        invoke.resolve()
    }
}
