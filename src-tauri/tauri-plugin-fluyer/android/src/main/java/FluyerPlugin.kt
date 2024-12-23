package org.alvindimas05.fluyerplugin

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
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
    private val implementation = FluyerMain(activity)

    override fun onPause(){
        super.onPause()
        implementation.toast("User is pausing...")
    }


    override fun onResume(){
        super.onResume()
        implementation.toast("User is resuming...")
    }

    @Command
    fun toast(invoke: Invoke) {
        val args = invoke.parseArgs(ToastArgs::class.java)
        implementation.toast(args.value)
        invoke.resolve()
    }

    @Command
    fun getNavigationBarHeight(invoke: Invoke){
        var obj = JSObject()
        obj.put("value", implementation.getNavigationBarHeight())
        invoke.resolve(obj)
    }

    @Command
    fun getStatusBarHeight(invoke: Invoke){
        var obj = JSObject()
        obj.put("value", implementation.getStatusBarHeight())
        invoke.resolve(obj)
    }
}
