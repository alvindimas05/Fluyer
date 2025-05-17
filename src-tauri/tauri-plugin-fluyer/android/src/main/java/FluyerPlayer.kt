package org.alvindimas05.fluyerplugin

import android.app.Activity
import androidx.media3.exoplayer.ExoPlayer
import java.io.File
import android.net.Uri
import app.tauri.annotation.InvokeArg
import androidx.media3.common.MediaItem
import androidx.media3.common.Player
import java.util.Locale

enum class PlayerCommand(val value: String) {
    Play("play"),
    Pause("pause"),
    Stop("stop"),
    Next("next"),
    Seek("seek"),
    Volume("volume"),
    Clear("clear"),
    AddPlaylist("addPlaylist"),
    RemovePlaylist("removePlaylist"),
    GotoPlaylist("gotoPlaylist"),
}

@InvokeArg
class PlayerCommandArgs {
    lateinit var command: String
    var seekPosition: Long? = null
    var volume: Float? = null
    var playlistFilePath: String? = null
    var playlistRemoveIndex: Int? = null
    var playlistGotoIndex: Int? = null
}

data class PlayerGetInfo (
    val currentPosition: Long,
    val isEmpty: Boolean,
    val isPlaying: Boolean,
    val index: Int,
)

class FluyerPlayer(activity: Activity) {
    val player = ExoPlayer.Builder(activity).build()

    init {
        player.addListener(object : Player.Listener {
            override fun onPlaybackStateChanged(state: Int) {
                if (state == Player.STATE_ENDED) {
                    player.clearMediaItems()
                }
            }
        })
    }
    
    fun sendCommand(args: PlayerCommandArgs) {
        val command = PlayerCommand.valueOf(args.command.replaceFirstChar { if (it.isLowerCase()) it.titlecase(
            Locale.ROOT
        ) else it.toString() })
        when(command){
            PlayerCommand.Play -> player.play()
            PlayerCommand.Pause, PlayerCommand.Stop -> player.pause()
            PlayerCommand.Seek -> player.seekTo(args.seekPosition!!)
            PlayerCommand.Volume -> player.volume = args.volume!!
            PlayerCommand.AddPlaylist -> {
                val file = File(args.playlistFilePath!!)
                player.addMediaItem(MediaItem.fromUri(Uri.fromFile(file)))
                player.prepare()
            }
            PlayerCommand.Next -> {
                player.seekToNext()
            }
            PlayerCommand.RemovePlaylist -> {
                player.removeMediaItem(args.playlistRemoveIndex!!)
            }
            PlayerCommand.Clear -> {
                player.clearMediaItems()
            }

            PlayerCommand.GotoPlaylist -> {
                player.seekTo(args.playlistGotoIndex!!, 0)
            }
        }
    }

    fun getInfo(): PlayerGetInfo {
        return PlayerGetInfo (
            currentPosition = player.currentPosition,
            isEmpty = player.mediaItemCount == 0,
            isPlaying = player.isPlaying,
            index = player.currentMediaItemIndex,
        )
    }
}