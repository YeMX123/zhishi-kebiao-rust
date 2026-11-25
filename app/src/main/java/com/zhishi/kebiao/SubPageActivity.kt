package com.zhishi.kebiao

import android.graphics.Color
import android.os.Bundle
import android.view.Gravity
import android.widget.LinearLayout
import android.widget.TextView

class SubPageActivity : MainActivity() {
    override fun startUrl(): String = intent.getStringExtra("url") ?: "file:///android_asset/prod/entry.html"

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val title = intent.getStringExtra("title") ?: "知时课表"
        val bg = runCatching { Color.parseColor(intent.getStringExtra("bg") ?: "#0B1224") }.getOrDefault(Color.parseColor("#0B1224"))
        val fg = runCatching { Color.parseColor(intent.getStringExtra("fg") ?: "#F8FAFC") }.getOrDefault(Color.WHITE)

        val root = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setBackgroundColor(bg) }
        val bar = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            gravity = Gravity.CENTER_VERTICAL
            setPadding(24, 36, 24, 24)
            setBackgroundColor(bg)
        }
        val back = TextView(this).apply {
            text = "← 返回"
            textSize = 15f
            setTextColor(fg)
            setPadding(8, 8, 24, 8)
            setOnClickListener { finish() }
        }
        val ttl = TextView(this).apply {
            text = title
            textSize = 17f
            setTextColor(fg)
        }
        bar.addView(back)
        bar.addView(ttl)

        (webView.parent as? android.view.ViewGroup)?.removeView(webView)
        root.addView(bar, LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT))
        root.addView(webView, LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, 0, 1f))
        setContentView(root)
    }
}
