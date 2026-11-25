package com.zhishi.kebiao

object RustCore {
    init {
        System.loadLibrary("zhishi_core")
    }

    @JvmStatic
    external fun coreVersion(): String

    @JvmStatic
    external fun computeWidgetState(
        rawCourseJson: String,
        themePref: String,
        sysDark: Boolean,
        nowSec: Long,
        todayStartSec: Long,
        tzOffsetSec: Long
    ): String

    @JvmStatic
    external fun parseWakeupSchedule(rawNdjson: String): String
}
