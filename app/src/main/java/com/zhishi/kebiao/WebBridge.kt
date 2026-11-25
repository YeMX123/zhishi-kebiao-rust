package com.zhishi.kebiao

import android.app.Activity
import android.content.ClipboardManager
import android.content.Context
import android.content.Intent
import android.graphics.Color
import android.webkit.JavascriptInterface
import androidx.core.view.WindowInsetsControllerCompat

class WebBridge(private val activity: Activity) {
    private val prefs = activity.getSharedPreferences("zhishi_storage", Context.MODE_PRIVATE)

    @JavascriptInterface
    fun getCoreEngine(): String = RustCore.coreVersion()

    @JavascriptInterface
    fun parseWakeupNative(rawNdjson: String): String = RustCore.parseWakeupSchedule(rawNdjson)

    @JavascriptInterface
    fun getStorage(key: String): String = prefs.getString(key, "") ?: ""

    @JavascriptInterface
    fun setStorage(key: String, value: String) {
        prefs.edit().putString(key, value).apply()
        if (key == "course_data" || key == "zs_widget_theme_v1" || key == "zs_widget_bg_cfg_v2") {
            ZhishiWidgetProvider.updateAll(activity)
        }
    }

    @JavascriptInterface
    fun refreshWidgets() {
        ZhishiWidgetProvider.updateAll(activity)
    }

    @JavascriptInterface
    fun openPage(url: String, title: String, bgHex: String, fgHex: String) {
        activity.runOnUiThread {
            val intent = Intent(activity, SubPageActivity::class.java).apply {
                putExtra("url", url)
                putExtra("title", title)
                putExtra("bg", bgHex)
                putExtra("fg", fgHex)
            }
            activity.startActivity(intent)
        }
    }

    @JavascriptInterface
    fun openPortal(url: String, initJs: String) {
        activity.runOnUiThread {
            val intent = Intent(activity, PortalActivity::class.java).apply {
                putExtra("url", url)
                putExtra("initJs", initJs)
            }
            activity.startActivity(intent)
        }
    }

    @JavascriptInterface
    fun closePage() {
        activity.runOnUiThread { activity.finish() }
    }

    @JavascriptInterface
    fun setStatusBar(isDark: Boolean) {
        activity.runOnUiThread {
            val col = Color.parseColor(if (isDark) "#090F20" else "#EEF3FB")
            activity.window.statusBarColor = col
            activity.window.navigationBarColor = col
            WindowInsetsControllerCompat(activity.window, activity.window.decorView).apply {
                isAppearanceLightStatusBars = !isDark
                isAppearanceLightNavigationBars = !isDark
            }
        }
    }

    @JavascriptInterface
    fun getClipboard(): String {
        val cm = activity.getSystemService(Context.CLIPBOARD_SERVICE) as? ClipboardManager
        return cm?.primaryClip?.getItemAt(0)?.coerceToText(activity)?.toString() ?: ""
    }
}
