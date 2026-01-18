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
import android.content.Intent
import android.os.Build
import android.util.Log
import android.view.View
import android.webkit.WebView
import androidx.activity.result.ActivityResult
import androidx.appcompat.app.AppCompatActivity
import app.tauri.annotation.ActivityCallback
import org.alvindimas05.fluyerplugin.utils.FileUtil
import kotlin.properties.Delegates

@InvokeArg
class ToastArgs {
    lateinit var value: String
}

@InvokeArg
class PickFolderWatcherArgs {
    lateinit var channel: Channel
}

@InvokeArg
class NavigationBarVisibilityArgs {
    var value by Delegates.notNull<Boolean>()
}

@InvokeArg
class VisualizerGetBufferArgs {
    lateinit var args: String
}

@InvokeArg
class MetadataArgs {
    lateinit var path: String
}

private const val ALIAS_READ_AUDIO: String = "audio"
private const val ALIAS_EXTERNAL_STORAGE: String = "storage"
const val LOG_TAG = "Fluyer"
@TauriPlugin(
    permissions = [
        Permission(strings = [Manifest.permission.READ_MEDIA_AUDIO],
            alias = ALIAS_READ_AUDIO
        ),
        Permission(strings = [Manifest.permission.READ_EXTERNAL_STORAGE],
            alias = ALIAS_EXTERNAL_STORAGE
        )
    ]
)
class FluyerPlugin(val activity: Activity): Plugin(activity) {
    private val implementation = FluyerMain(activity)
    private var pickFolderChannel: Channel? = null

    override fun load(webView: WebView) {
        FluyerMetadata.initialize(activity)
        super.load(webView)
    }

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

    @Command
    fun getStatusBarHeight(invoke: Invoke){
        val obj = JSObject().put("value", implementation.getStatusBarHeight())
        invoke.resolve(obj)
    }
    
    @Command
    fun restartApp(invoke: Invoke) {
        implementation.restartApp()
        invoke.resolve()
    }

    @Command
    fun getSdkVersion(invoke: Invoke) {
        invoke.resolve(JSObject().put("value", Build.VERSION.SDK_INT))
    }

    @Suppress("DEPRECATION")
    @Command
    fun setNavigationBarVisibility(invoke: Invoke){
        val visible = invoke.parseArgs(NavigationBarVisibilityArgs::class.java).value
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            val id = android.view.WindowInsets.Type.statusBars()
            if(visible) activity.window.insetsController?.show(id)
            else activity.window.insetsController?.hide(id)
        } else {
            if (visible) {
                activity.window.decorView.systemUiVisibility = View.SYSTEM_UI_FLAG_VISIBLE
            } else {
                activity.window.decorView.systemUiVisibility =
                    View.SYSTEM_UI_FLAG_HIDE_NAVIGATION or View.SYSTEM_UI_FLAG_FULLSCREEN
            }
        }
        invoke.resolve()
    }

    @Command
    fun requestPickFolder(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(PickFolderWatcherArgs::class.java)
            pickFolderChannel = args.channel
            val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
                addFlags(
                    Intent.FLAG_GRANT_READ_URI_PERMISSION or
                    Intent.FLAG_GRANT_WRITE_URI_PERMISSION or
                    Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION
                )
            }
            startActivityForResult(invoke, intent, "onFolderPicked")
            invoke.resolve(JSObject().put("value", true))
        } catch (err: Exception) {
            Log.e(LOG_TAG, err.toString())
        }
    }

    @ActivityCallback
    fun onFolderPicked(invoke: Invoke, result: ActivityResult) {
        if (result.resultCode == Activity.RESULT_OK) {
            val folderUri = result.data?.data
            if (folderUri != null) {
                activity.contentResolver.takePersistableUriPermission(
                    folderUri,
                    Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_WRITE_URI_PERMISSION
                )
                pickFolderChannel!!.send(JSObject().put("value", FileUtil.getFullPathFromTreeUri(folderUri, activity)))
            }
        }
        pickFolderChannel!!.send(JSObject().put("value", null))
    }
    
    @Command
    fun visualizerGetBuffer(invoke: Invoke): Boolean {
        try {
            val args = invoke.parseArgs(VisualizerGetBufferArgs::class.java)
            val result = FluyerVisualizer.getBuffer(args.args)
            invoke.resolve(JSObject().put("value", result))
            return result
        } catch (err: Exception){
            Log.e(LOG_TAG, err.message.toString())
            invoke.resolve(JSObject().put("value", false))
            return false
        }
    }
    
    @Command
    fun metadataGet(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(MetadataArgs::class.java)
            val result = FluyerMetadata.getMetadata(args.path)
            val response = JSObject()
            response.put("value", result)
            invoke.resolve(response)
        } catch (err: Exception) {
            Log.e(LOG_TAG, "metadataGet error: ${err.message}")
            invoke.resolve(JSObject())
        }
    }
    
    @Command
    fun metadataGetImage(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(MetadataArgs::class.java)
            val result = FluyerMetadata.getImage(args.path)
            invoke.resolve(JSObject().put("path", result))
        } catch (err: Exception) {
            Log.e(LOG_TAG, "metadataGetImage error: ${err.message}")
            invoke.resolve(JSObject().put("path", null))
        }
    }
    
    @Command
    fun audioConvertToWav(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(MetadataArgs::class.java)
            val result = FluyerMetadata.convertToPcmWav(args.path)
            invoke.resolve(JSObject().put("path", result))
        } catch (err: Exception) {
            Log.e(LOG_TAG, "audioConvertToWav error: ${err.message}")
            invoke.resolve(JSObject().put("path", null))
        }
    }
}
