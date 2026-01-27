package org.alvindimas05.fluyer
import android.graphics.Color
import android.os.Build
import android.os.Bundle
import android.view.WindowManager
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsControllerCompat

@Suppress("DEPRECATION")
class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)

    // Enable edge-to-edge layout (supports API 24+ via WindowCompat)
    WindowCompat.setDecorFitsSystemWindows(window, false)
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
      window.attributes.layoutInDisplayCutoutMode =
        WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
    }

    // Make navigation bar transparent (API 21+)
    window.navigationBarColor = Color.TRANSPARENT

    val insetsController = WindowCompat.getInsetsController(window, window.decorView)
    
    // Make navigation bar white
    insetsController.isAppearanceLightNavigationBars = false

    // Remove nav-bar contrast scrim (API 29+)
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
      window.isNavigationBarContrastEnforced = false
    }

    // Let nav-bar reappear on swipe (immersive feel)
    insetsController.systemBarsBehavior =
      WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE

    // Transparent status bar & adaptive icon colors
    window.statusBarColor = Color.TRANSPARENT
    
    // Make status bar white
    insetsController.isAppearanceLightStatusBars = false

    window.decorView.setBackgroundColor(Color.TRANSPARENT)
  }
}