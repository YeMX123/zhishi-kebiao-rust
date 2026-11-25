package com.zhishi.kebiao

import android.annotation.SuppressLint
import android.graphics.Color
import android.graphics.drawable.GradientDrawable
import android.os.Bundle
import android.view.Gravity
import android.webkit.WebChromeClient
import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.Button
import android.widget.FrameLayout
import androidx.appcompat.app.AppCompatActivity

class PortalActivity : AppCompatActivity() {
    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val url = intent.getStringExtra("url") ?: "https://xk.henu.edu.cn"
        val initJs = intent.getStringExtra("initJs") ?: ""

        val root = FrameLayout(this)
        val wv = WebView(this)
        wv.settings.apply {
            javaScriptEnabled = true
            domStorageEnabled = true
            useWideViewPort = true
            loadWithOverviewMode = true
            builtInZoomControls = true
            displayZoomControls = false
            mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
            userAgentString = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        }
        wv.addJavascriptInterface(WebBridge(this), "AndroidBridge")
        wv.webViewClient = WebViewClient()
        wv.webChromeClient = WebChromeClient()
        wv.loadUrl(url)

        val btn = Button(this).apply {
            text = "导入课表"
            setTextColor(Color.WHITE)
            textSize = 14f
            background = GradientDrawable().apply {
                setColor(Color.parseColor("#2563EB"))
                cornerRadius = 60f
            }
            setPadding(44, 24, 44, 24)
            setOnClickListener {
                val shim = "window.HB={Storage:{setStorageSync:function(o){window.AndroidBridge.setStorage(o.key,o.value);}},Interface:{navigateBack:function(){window.AndroidBridge.closePage();}}};"
                wv.evaluateJavascript(shim + initJs, null)
            }
        }
        val lp = FrameLayout.LayoutParams(FrameLayout.LayoutParams.WRAP_CONTENT, FrameLayout.LayoutParams.WRAP_CONTENT, Gravity.BOTTOM or Gravity.END).apply {
            setMargins(0, 0, 42, 64)
        }
        root.addView(wv)
        root.addView(btn, lp)
        setContentView(root)
    }
}
