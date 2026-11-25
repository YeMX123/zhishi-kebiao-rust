pub mod holiday_engine;
pub mod wakeup_parser;
pub mod widget_engine;

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jlong, jstring};
use jni::JNIEnv;

fn jstr_to_string(env: &mut JNIEnv<'_>, s: &JString<'_>) -> String {
    if s.is_null() {
        return String::new();
    }
    match env.get_string(s) {
        Ok(js) => js.to_string_lossy().into_owned(),
        Err(_) => String::new(),
    }
}

fn string_to_jstr(env: &JNIEnv<'_>, s: &str) -> jstring {
    match env.new_string(s) {
        Ok(js) => js.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_zhishi_kebiao_RustCore_coreVersion<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    string_to_jstr(&env, "zhishi-rust-core 3.3.0")
}

#[no_mangle]
pub extern "system" fn Java_com_zhishi_kebiao_RustCore_computeWidgetState<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    raw_course_json: JString<'local>,
    theme_pref: JString<'local>,
    sys_dark: jboolean,
    now_sec: jlong,
    today_start_sec: jlong,
    tz_offset_sec: jlong,
) -> jstring {
    let raw = jstr_to_string(&mut env, &raw_course_json);
    let theme = jstr_to_string(&mut env, &theme_pref);
    let state = widget_engine::compute_widget_state(
        &raw,
        &theme,
        sys_dark != 0,
        now_sec,
        today_start_sec,
        tz_offset_sec,
    );
    let out = serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_string());
    string_to_jstr(&env, &out)
}

#[no_mangle]
pub extern "system" fn Java_com_zhishi_kebiao_RustCore_parseWakeupSchedule<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    raw_ndjson: JString<'local>,
) -> jstring {
    let raw = jstr_to_string(&mut env, &raw_ndjson);
    let parsed = wakeup_parser::parse_wakeup_ndjson(&raw);
    let out = serde_json::to_string(&parsed).unwrap_or_else(|_| "{}".to_string());
    string_to_jstr(&env, &out)
}
