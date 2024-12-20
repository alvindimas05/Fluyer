package org.alvindimas05.fluyerplugin

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Channel
import app.tauri.plugin.JSObject
import app.tauri.annotation.Permission
import android.Manifest

@InvokeArg
class ToastArgs {
  lateinit var value: String
}

private const val ALIAS_READ_AUDIO: String = "audio"
@TauriPlugin(
    permissions = [
        Permission(strings = [
            Manifest.permission.READ_MEDIA_AUDIO
        ],
            alias = ALIAS_READ_AUDIO
        )
    ]
)
class FluyerPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = FluyerMain()
    private var channel: Channel? = null

    @Command
    fun toast(invoke: Invoke) {
        val args = invoke.parseArgs(ToastArgs::class.java)
        implementation.toast(activity, args.value)
        invoke.resolve()
    }
}
