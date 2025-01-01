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
import android.util.Log
import android.os.Bundle

@InvokeArg
class ToastArgs {
    lateinit var value: String
}

@InvokeArg
class StateWatcherArgs {
    lateinit var channel: Channel
}

enum class WatcherStateType(val value: String) {
    Pause("pause"),
    Resume("resume");
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
        stateChannel?.let{
            it.send(JSObject().put("value", WatcherStateType.Pause.value))
        }
    }

    override fun onResume(){
        super.onResume()
        stateChannel?.let{
            it.send(JSObject().put("value", WatcherStateType.Resume))
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
    fun watchState(invoke: Invoke) {
        if (stateChannel != null) {
            invoke.resolve(JSObject().put("value", true))
            return
        }

        val args = invoke.parseArgs(StateWatcherArgs::class.java)
        stateChannel = args.channel
        invoke.resolve(JSObject().put("value", stateChannel != null))
    }
    
    @Command
    fun listenToHeadsetChange(invoke: Invoke) {
        implementation.listenToHeadsetChange()
        invoke.resolve()
    }
}
