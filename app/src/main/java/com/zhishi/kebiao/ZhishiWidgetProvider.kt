package com.zhishi.kebiao

import android.app.PendingIntent
import android.appwidget.AppWidgetManager
import android.appwidget.AppWidgetProvider
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.res.Configuration
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.RectF
import android.util.Base64
import android.view.View
import android.widget.RemoteViews
import org.json.JSONArray
import org.json.JSONObject
import java.util.Calendar
import java.util.TimeZone

class ZhishiWidgetProvider : AppWidgetProvider() {
    override fun onUpdate(context: Context, mgr: AppWidgetManager, ids: IntArray) {
        for (id in ids) updateWidget(context, mgr, id)
    }

    companion object {
        private val LEFT_ITEMS = intArrayOf(R.id.left_item_0, R.id.left_item_1)
        private val LEFT_BARS = intArrayOf(R.id.left_bar_0, R.id.left_bar_1)
        private val LEFT_NAMES = intArrayOf(R.id.left_name_0, R.id.left_name_1)
        private val LEFT_TIMES = intArrayOf(R.id.left_time_0, R.id.left_time_1)
        private val LEFT_LOCS = intArrayOf(R.id.left_loc_0, R.id.left_loc_1)

        private val RIGHT_ITEMS = intArrayOf(R.id.right_item_0, R.id.right_item_1, R.id.right_item_2)
        private val RIGHT_BARS = intArrayOf(R.id.right_bar_0, R.id.right_bar_1, R.id.right_bar_2)
        private val RIGHT_NAMES = intArrayOf(R.id.right_name_0, R.id.right_name_1, R.id.right_name_2)
        private val RIGHT_TIMES = intArrayOf(R.id.right_time_0, R.id.right_time_1, R.id.right_time_2)
        private val RIGHT_LOCS = intArrayOf(R.id.right_loc_0, R.id.right_loc_1, R.id.right_loc_2)

        fun updateAll(context: Context) {
            val mgr = AppWidgetManager.getInstance(context)
            val ids = mgr.getAppWidgetIds(ComponentName(context, ZhishiWidgetProvider::class.java))
            for (id in ids) updateWidget(context, mgr, id)
        }

        private fun makeColorBar(colorHex: String): Bitmap {
            val c = runCatching { Color.parseColor(colorHex) }.getOrDefault(Color.parseColor("#FF4AA5F0"))
            val bmp = Bitmap.createBitmap(12, 132, Bitmap.Config.ARGB_8888)
            val canvas = Canvas(bmp)
            val paint = Paint(Paint.ANTI_ALIAS_FLAG).apply { color = c }
            canvas.drawRoundRect(RectF(0f, 0f, 12f, 132f), 6f, 6f, paint)
            return bmp
        }

        private fun updateWidget(context: Context, mgr: AppWidgetManager, id: Int) {
            val prefs = context.getSharedPreferences("zhishi_storage", Context.MODE_PRIVATE)
            val theme = prefs.getString("zs_widget_theme_v1", "auto") ?: "auto"
            val sysDark = (context.resources.configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK) == Configuration.UI_MODE_NIGHT_YES
            val rawCourse = prefs.getString("course_data", "") ?: ""

            val nowMs = System.currentTimeMillis()
            val nowSec = nowMs / 1000L
            val tzOffsetSec = TimeZone.getDefault().getOffset(nowMs) / 1000L
            val todayStartSec = Calendar.getInstance().apply {
                set(Calendar.HOUR_OF_DAY, 0)
                set(Calendar.MINUTE, 0)
                set(Calendar.SECOND, 0)
                set(Calendar.MILLISECOND, 0)
            }.timeInMillis / 1000L

            // Delegate all schedule filtering, 2-column overflow, date & theme calculation to Rust core
            val vmJson = RustCore.computeWidgetState(rawCourse, theme, sysDark, nowSec, todayStartSec, tzOffsetSec)
            val vm = JSONObject(vmJson)
            val pal = vm.getJSONObject("palette")
            val isDark = pal.optBoolean("is_dark", true)
            val textColor = Color.parseColor(pal.optString("text_color", "#FFFFFFFF"))
            val subTextColor = Color.parseColor(pal.optString("sub_text_color", "#FFB4C2D6"))
            val dividerColor = Color.parseColor(pal.optString("divider_color", "#33FFFFFF"))
            val accentColor = Color.parseColor(pal.optString("accent_color", "#FF7DD3FC"))

            val rv = RemoteViews(context.packageName, R.layout.widget_zhishi)
            rv.setInt(R.id.wRoot, "setBackgroundResource", if (isDark) R.drawable.widget_bg_dark else R.drawable.widget_bg_light)
            rv.setTextColor(R.id.date_number, textColor)
            rv.setInt(R.id.week_divider_1, "setBackgroundColor", dividerColor)
            rv.setInt(R.id.week_divider_2, "setBackgroundColor", dividerColor)
            rv.setTextColor(R.id.week_number, subTextColor)
            rv.setTextColor(R.id.week_text, subTextColor)
            rv.setTextColor(R.id.day_prefix, accentColor)
            rv.setTextColor(R.id.day_of_week, accentColor)
            rv.setTextColor(R.id.future_date_label, textColor)
            rv.setTextColor(R.id.left_empty, subTextColor)
            rv.setTextColor(R.id.right_empty, subTextColor)
            rv.setTextColor(R.id.additional_courses_text, subTextColor)

            // Optional custom widget background image
            val bgRaw = prefs.getString("zs_widget_bg_cfg_v2", "") ?: ""
            var hasCustomImg = false
            if (bgRaw.isNotEmpty()) {
                runCatching {
                    val src = JSONObject(bgRaw).optString("src", "")
                    val idx = src.indexOf("base64,")
                    if (idx != -1) {
                        val bytes = Base64.decode(src.substring(idx + 7), Base64.DEFAULT)
                        val bmp = BitmapFactory.decodeByteArray(bytes, 0, bytes.size)
                        if (bmp != null) {
                            rv.setImageViewBitmap(R.id.wBgImg, bmp)
                            rv.setViewVisibility(R.id.wBgImg, View.VISIBLE)
                            hasCustomImg = true
                        }
                    }
                }
            }
            if (!hasCustomImg) rv.setViewVisibility(R.id.wBgImg, View.GONE)

            // Header from Rust view model
            rv.setTextViewText(R.id.date_number, vm.optString("date_number", "1"))
            rv.setTextViewText(R.id.week_number, vm.optString("week_number", "1"))
            rv.setTextViewText(R.id.day_of_week, vm.optString("day_of_week", "一"))

            // Left column
            val leftArr = vm.optJSONArray("left_courses") ?: JSONArray()
            val leftEmpty = if (vm.isNull("left_empty_text")) "" else vm.optString("left_empty_text", "")
            bindSlots(rv, leftArr, LEFT_ITEMS, LEFT_BARS, LEFT_NAMES, LEFT_TIMES, LEFT_LOCS, textColor)
            if (leftEmpty.isNotEmpty()) {
                rv.setTextViewText(R.id.left_empty, leftEmpty)
                rv.setViewVisibility(R.id.left_empty, View.VISIBLE)
            } else {
                rv.setViewVisibility(R.id.left_empty, View.GONE)
            }

            // Right column
            val futureLabel = if (vm.isNull("future_date_label")) "" else vm.optString("future_date_label", "")
            if (futureLabel.isNotEmpty()) {
                rv.setTextViewText(R.id.future_date_label, futureLabel)
                rv.setViewVisibility(R.id.future_date_label, View.VISIBLE)
            } else {
                rv.setViewVisibility(R.id.future_date_label, View.GONE)
            }

            val rightArr = vm.optJSONArray("right_courses") ?: JSONArray()
            bindSlots(rv, rightArr, RIGHT_ITEMS, RIGHT_BARS, RIGHT_NAMES, RIGHT_TIMES, RIGHT_LOCS, textColor)

            val rightEmpty = if (vm.isNull("right_empty_text")) "" else vm.optString("right_empty_text", "")
            if (rightEmpty.isNotEmpty()) {
                rv.setTextViewText(R.id.right_empty, rightEmpty)
                rv.setViewVisibility(R.id.right_empty, View.VISIBLE)
            } else {
                rv.setViewVisibility(R.id.right_empty, View.GONE)
            }

            val extraTxt = if (vm.isNull("additional_courses_text")) "" else vm.optString("additional_courses_text", "")
            if (extraTxt.isNotEmpty()) {
                rv.setTextViewText(R.id.additional_courses_text, extraTxt)
                rv.setViewVisibility(R.id.additional_courses_text, View.VISIBLE)
            } else {
                rv.setViewVisibility(R.id.additional_courses_text, View.GONE)
            }

            val pi = PendingIntent.getActivity(
                context,
                0,
                Intent(context, MainActivity::class.java),
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
            )
            rv.setOnClickPendingIntent(R.id.wRoot, pi)
            mgr.updateAppWidget(id, rv)
        }

        private fun bindSlots(
            rv: RemoteViews,
            arr: JSONArray,
            itemIds: IntArray,
            barIds: IntArray,
            nameIds: IntArray,
            timeIds: IntArray,
            locIds: IntArray,
            textColor: Int
        ) {
            for (i in itemIds.indices) {
                if (i < arr.length()) {
                    val item = arr.getJSONObject(i)
                    rv.setViewVisibility(itemIds[i], View.VISIBLE)
                    rv.setImageViewBitmap(barIds[i], makeColorBar(item.optString("color", "#4AA5F0")))
                    rv.setTextViewText(nameIds[i], item.optString("name", ""))
                    rv.setTextColor(nameIds[i], textColor)
                    rv.setTextViewText(timeIds[i], item.optString("time_range", ""))
                    rv.setTextColor(timeIds[i], textColor)
                    rv.setTextViewText(locIds[i], item.optString("loc", "暂未指定地点"))
                    rv.setTextColor(locIds[i], textColor)
                } else {
                    rv.setViewVisibility(itemIds[i], View.GONE)
                }
            }
        }
    }
}
