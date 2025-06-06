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
import android.os.Build
import android.util.Log
import androidx.annotation.RequiresApi

@InvokeArg
class ToastArgs {
    lateinit var value: String
}

@InvokeArg
class PlaylistChangeWatcherArgs {
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
class FluyerPlugin(activity: Activity): Plugin(activity) {
    private val implementation = FluyerMain(activity)
    private val player = FluyerPlayer(activity)

    @Command
    fun toast(invoke: Invoke) {
        val args = invoke.parseArgs(ToastArgs::class.java)
        implementation.toast(args.value)
        invoke.resolve()
    }

    @Command
    fun getNavigationBarHeight(invoke: Invoke){
        val obj = JSObject().put("value", implementation.getNavigationBarHeight())
        invoke.resolve(obj)
    }

    @RequiresApi(Build.VERSION_CODES.R)
    @Command
    fun getStatusBarHeight(invoke: Invoke){
        val obj = JSObject().put("value", implementation.getStatusBarHeight())
        invoke.resolve(obj)
    }

    @Command
    fun watchPlaylistChange(invoke: Invoke) {
        val args = invoke.parseArgs(PlaylistChangeWatcherArgs::class.java)
        player.listenPlaylistChange {
            args.channel.send(JSObject())
        }
        invoke.resolve(JSObject().put("value", true))
    }
    
    @Command
    fun restartApp(invoke: Invoke) {
        implementation.restartApp()
        invoke.resolve()
    }
    
    @Command
    fun playerRunCommand(invoke: Invoke) {
        Log.d("Fluyer", invoke.getArgs().toString())
        try {
            val args = invoke.parseArgs(PlayerCommandArgs::class.java)
            player.sendCommand(args)
        } catch (err: Exception){
            Log.e("Fluyer", err.toString())
        }
        invoke.resolve()
    }

    @Command
    fun playerGetInfo(invoke: Invoke){
        try {
            val info = player.getInfo()
            invoke.resolve(JSObject()
                .put("currentPosition", info.currentPosition)
                .put("isEmpty", info.isEmpty)
                .put("isPlaying", info.isPlaying)
                .put("index", info.index)
            )
        } catch (err: Exception){
            Log.e("Fluyer", err.toString())
        }
    }
}
