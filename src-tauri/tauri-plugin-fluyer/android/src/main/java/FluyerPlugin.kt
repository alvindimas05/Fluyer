package org.alvindimas05.fluyerplugin

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.annotation.Permission
import app.tauri.plugin.Channel
import android.Manifest

@InvokeArg
class ToastArgs {
  lateinit var value: String
}

@InvokeArg
class StateWatcherArgs {
    lateinit var channel: Channel
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
    private var stateChannel: Channel? = null

    override fun onPause(){
        super.onPause()
        if(stateChannel != null){
            stateChannel.send(JSObject().put("value", "pause"))
        }
    }


    override fun onResume(){
        super.onResume()
        if(stateChannel != null){
            stateChannel.send(JSObject().put("value", "resume"))
        }
    }

    @Command
    fun toast(invoke: Invoke) {
        val args = invoke.parseArgs(ToastArgs::class.java)
        implementation.toast(args.value)
        invoke.resolve()
    }

    @Command
    fun getNavigationBarHeight(invoke: Invoke){
        var obj = JSObject().put("value", implementation.getNavigationBarHeight())
        invoke.resolve(obj)
    }

    @Command
    fun getStatusBarHeight(invoke: Invoke){
        var obj = JSObject().put("value", implementation.getStatusBarHeight())
        invoke.resolve(obj)
    }

    @Command
    fun watchState(invoke: Invoke){
        if(stateWatcher != null){
            invoke.resolve(false)
            return
        }

        val args = invoke.parseArgs(StateWatcherArgs::class.java)
        stateWatcher = args.channel
        invoke.resolve(true)
    }
}
