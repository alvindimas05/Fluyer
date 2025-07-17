package org.alvindimas05.fluyerplugin

import android.app.Activity
import android.content.ComponentName
import android.content.Intent
import androidx.media3.exoplayer.ExoPlayer
import java.io.File
import android.net.Uri
import android.util.Log
import app.tauri.annotation.InvokeArg
import androidx.media3.common.MediaItem
import androidx.media3.common.MediaMetadata
import androidx.media3.common.Player
import androidx.media3.common.Tracks
import androidx.media3.session.MediaController
import androidx.media3.session.MediaSession
import java.util.Locale
import androidx.media3.session.MediaSessionService
import androidx.media3.session.SessionToken
import com.google.common.util.concurrent.ListenableFuture
import com.google.common.util.concurrent.MoreExecutors
import kotlin.properties.Delegates

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

@InvokeArg
class PlaylistAddMusic {
    lateinit var filePath: String
    lateinit var title: String
    lateinit var artist: String
    var image: String? = null
}

@InvokeArg
class PlayerPlaylistMoveToArgs {
    var from by Delegates.notNull<Int>()
    var to by Delegates.notNull<Int>()
}

data class PlayerGetInfo (
    val currentPosition: Long,
    val isEmpty: Boolean,
    val isPlaying: Boolean,
    val index: Int,
)

class FluyerPlayer(val activity: Activity) {
    private lateinit var factory: ListenableFuture<MediaController>
    private var player: MediaController? = null

    private var callbackPlaylistChange: ((isNext: Boolean) -> Unit)? = null

    fun initialize() {
        factory = MediaController.Builder(
            activity, SessionToken(activity, ComponentName(activity, PlaybackService::class.java)))
            .buildAsync()
        factory.addListener ({
            factory.let {
                if (it.isDone) {
                    player = it.get()
                    player!!.addListener(object: Player.Listener {
                        override fun onMediaItemTransition(mediaItem: MediaItem?, reason: Int) {
                            super.onMediaItemTransition(mediaItem, reason)
                            Log.d("FluyerPlayer", "onMediaItemTransition: $reason")
                            callbackPlaylistChange?.invoke(arrayListOf(
                                Player.MEDIA_ITEM_TRANSITION_REASON_PLAYLIST_CHANGED,
                                Player.MEDIA_ITEM_TRANSITION_REASON_SEEK,
                            ).contains(reason))
                        }
                    })
                }
            }
        }, MoreExecutors.directExecutor())
    }

    fun sendCommand(args: PlayerCommandArgs) {
        val command = PlayerCommand.valueOf(args.command.replaceFirstChar { if (it.isLowerCase()) it.titlecase(
            Locale.ROOT
        ) else it.toString() })
        when(command){
            PlayerCommand.Play -> player?.play()
            PlayerCommand.Pause, PlayerCommand.Stop -> player?.pause()
            PlayerCommand.Seek -> player?.seekTo(args.seekPosition!!)
            PlayerCommand.Volume -> player?.volume = args.volume!!
            PlayerCommand.Next -> {
                player?.seekToNext()
            }
            PlayerCommand.RemovePlaylist -> {
                player?.removeMediaItem(args.playlistRemoveIndex!!)
            }
            PlayerCommand.Clear -> {
                player?.clearMediaItems()
            }
            PlayerCommand.GotoPlaylist -> {
                player?.seekTo(args.playlistGotoIndex!!, 0)
            }
            PlayerCommand.Repeat -> {
                player?.repeatMode = Player.REPEAT_MODE_ALL
            }
            PlayerCommand.RepeatOne -> {
                player?.repeatMode = Player.REPEAT_MODE_ONE
            }
            PlayerCommand.RepeatNone -> {
                player?.repeatMode = Player.REPEAT_MODE_OFF
            }
        }
    }

    fun addPlaylist(playlist: List<PlaylistAddMusic>) {
        playlist.forEach { item ->
            val metadata = MediaMetadata.Builder()
                .setTitle(item.title)
                .setArtist(item.artist)
//                .setArtworkUri(if(item.image.isNotEmpty()) item.image.toUri() else null)
                .build()

            val mediaItem = MediaItem.Builder()
                .setUri(Uri.fromFile(File(item.filePath)))
                .setMediaMetadata(metadata)
                .build()
            player?.addMediaItem(mediaItem)
        }
        player?.prepare()
    }

    fun playlistMoveTo(from: Int, to: Int) {
        player?.moveMediaItem(from, to)
    }

    fun getInfo(): PlayerGetInfo {
        return PlayerGetInfo (
            currentPosition = player?.currentPosition ?: 0,
            isEmpty = player?.mediaItemCount == 0,
            isPlaying = player?.isPlaying == true,
            index = player?.currentMediaItemIndex ?: -1,
        )
    }

    fun listenPlaylistChange(callback: (isNext: Boolean) -> Unit){
        callbackPlaylistChange = callback
    }
}

class PlaybackService : MediaSessionService() {
    private var mediaSession: MediaSession? = null
    override fun onCreate() {
        super.onCreate()
        val player = ExoPlayer.Builder(this).build()
        mediaSession = MediaSession.Builder(this, player)
            .build()
    }

    override fun onTaskRemoved(rootIntent: Intent?) {
        val player = mediaSession?.player

        if (player == null) {
            return
        }

        if (player.playWhenReady || player.mediaItemCount == 0) {
            stopSelf()
        }
    }

    override fun onGetSession(controllerInfo: MediaSession.ControllerInfo): MediaSession? {
        return mediaSession
    }

    override fun onDestroy() {
        mediaSession.run {
            this?.player?.release()
            this?.release()
            mediaSession = null
        }
        super.onDestroy()
    }
}