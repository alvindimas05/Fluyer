package org.alvindimas05.fluyerplugin

import android.app.Activity
import androidx.media3.exoplayer.ExoPlayer
import java.io.File
import android.net.Uri
import app.tauri.annotation.InvokeArg
import androidx.media3.common.MediaItem
import androidx.media3.common.MediaMetadata
import androidx.media3.common.Player
import androidx.media3.session.MediaSession
import java.util.Locale
import androidx.core.net.toUri

enum class PlayerCommand(val value: String) {
    Play("play"),
    Pause("pause"),
    Stop("stop"),
    Next("next"),
    Seek("seek"),
    Volume("volume"),
    Clear("clear"),
    RemovePlaylist("removePlaylist"),
    GotoPlaylist("gotoPlaylist"),
    Repeat("repeat"),
    RepeatOne("repeatOne"),
    RepeatNone("repeatNone"),
}

@InvokeArg
class PlayerCommandArgs {
    lateinit var command: String
    var seekPosition: Long? = null
    var volume: Float? = null
    var playlistRemoveIndex: Int? = null
    var playlistGotoIndex: Int? = null
}

@InvokeArg
class PlayerPlaylistAddArgs {
    lateinit var playlist: List<PlaylistAddMusic>
}

data class PlaylistAddMusic (
    val filePath: String,
    val title: String,
    val artist: String,
    val image: String?,
)

data class PlayerGetInfo (
    val currentPosition: Long,
    val isEmpty: Boolean,
    val isPlaying: Boolean,
    val index: Int,
)

class FluyerPlayer(activity: Activity) {
    private val player = ExoPlayer.Builder(activity).build()
    private val mediaSession = MediaSession.Builder(activity, player).build()

    fun sendCommand(args: PlayerCommandArgs) {
        val command = PlayerCommand.valueOf(args.command.replaceFirstChar { if (it.isLowerCase()) it.titlecase(
            Locale.ROOT
        ) else it.toString() })
        when(command){
            PlayerCommand.Play -> player.play()
            PlayerCommand.Pause, PlayerCommand.Stop -> player.pause()
            PlayerCommand.Seek -> player.seekTo(args.seekPosition!!)
            PlayerCommand.Volume -> player.volume = args.volume!!
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
            PlayerCommand.Repeat -> {
                player.repeatMode = Player.REPEAT_MODE_ALL
            }
            PlayerCommand.RepeatOne -> {
                player.repeatMode = Player.REPEAT_MODE_ONE
            }
            PlayerCommand.RepeatNone -> {
                player.repeatMode = Player.REPEAT_MODE_OFF
            }
        }
    }

    fun addPlaylist(playlist: List<PlaylistAddMusic>) {
        playlist.forEach { item ->
            val mediaItem = MediaItem.Builder()
                .setUri(Uri.fromFile(File(item.filePath)))
                .setMediaMetadata(MediaMetadata.Builder()
                    .setTitle(item.title)
                    .setArtist(item.artist)
                    .setArtworkUri(item.image?.toUri())
                    .build())
                .build()
            player.addMediaItem(mediaItem)
        }
        player.prepare()
    }

    fun getInfo(): PlayerGetInfo {
        return PlayerGetInfo (
            currentPosition = player.currentPosition,
            isEmpty = player.mediaItemCount == 0,
            isPlaying = player.isPlaying,
            index = player.currentMediaItemIndex,
        )
    }

    fun listenPlaylistChange(callback: () -> Unit){
        player.addListener(object : Player.Listener {
            override fun onMediaItemTransition(mediaItem: MediaItem?, reason: Int) {
                super.onMediaItemTransition(mediaItem, reason)
                callback()
            }
        })
    }
}