use serde::Serialize;
use serde_json::Value;

const WEEKDAYS: [&str; 7] = ["日", "一", "二", "三", "四", "五", "六"];

#[derive(Debug, Clone, Serialize)]
pub struct WidgetCourseItem {
    pub name: String,
    pub time_range: String,
    pub loc: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WidgetPalette {
    pub is_dark: bool,
    pub text_color: String,
    pub sub_text_color: String,
    pub divider_color: String,
    pub accent_color: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WidgetViewModel {
    pub palette: WidgetPalette,
    pub date_number: String,
    pub week_number: String,
    pub day_of_week: String,
    pub left_courses: Vec<WidgetCourseItem>,
    pub left_empty_text: Option<String>,
    pub future_date_label: Option<String>,
    pub right_courses: Vec<WidgetCourseItem>,
    pub right_empty_text: Option<String>,
    pub additional_courses_text: Option<String>,
}

#[derive(Debug, Clone)]
struct RawSlot {
    start_sec: i64,
    end_sec: i64,
    name: String,
    loc: String,
    color: String,
}

pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as u32, d as u32)
}

fn format_hm(unix_sec: i64, tz_offset_sec: i64) -> String {
    let local_sec = unix_sec + tz_offset_sec;
    let sec_in_day = local_sec.rem_euclid(86400);
    let h = sec_in_day / 3600;
    let m = (sec_in_day % 3600) / 60;
    format!("{:02}:{:02}", h, m)
}

fn format_md(unix_sec: i64, tz_offset_sec: i64) -> String {
    let local_days = (unix_sec + tz_offset_sec).div_euclid(86400);
    let (_, m, d) = civil_from_days(local_days);
    format!("{}月{}日", m, d)
}

pub fn resolve_palette(theme_pref: &str, sys_dark: bool) -> WidgetPalette {
    let is_dark = match theme_pref {
        "dark" => true,
        "light" => false,
        _ => sys_dark,
    };
    if is_dark {
        WidgetPalette {
            is_dark: true,
            text_color: "#FFFFFFFF".to_string(),
            sub_text_color: "#FFB4C2D6".to_string(),
            divider_color: "#33FFFFFF".to_string(),
            accent_color: "#FF7DD3FC".to_string(),
        }
    } else {
        WidgetPalette {
            is_dark: false,
            text_color: "#FF0F172A".to_string(),
            sub_text_color: "#FF475569".to_string(),
            divider_color: "#330F172A".to_string(),
            accent_color: "#FF2563EB".to_string(),
        }
    }
}

fn to_item(slot: &RawSlot, tz_offset_sec: i64) -> WidgetCourseItem {
    let time_range = format!(
        "{}-{}",
        format_hm(slot.start_sec, tz_offset_sec),
        format_hm(slot.end_sec, tz_offset_sec)
    );
    WidgetCourseItem {
        name: slot.name.clone(),
        time_range,
        loc: if slot.loc.is_empty() {
            "暂未指定地点".to_string()
        } else {
            slot.loc.clone()
        },
        color: slot.color.clone(),
    }
}

pub fn compute_widget_state(
    raw_course_json: &str,
    theme_pref: &str,
    sys_dark: bool,
    now_sec: i64,
    today_start_sec: i64,
    tz_offset_sec: i64,
) -> WidgetViewModel {
    let palette = resolve_palette(theme_pref, sys_dark);

    let local_days = (today_start_sec + tz_offset_sec + 43200).div_euclid(86400);
    let (_, _, day_of_month) = civil_from_days(local_days);
    let dow_idx = (local_days + 4).rem_euclid(7) as usize;
    let date_number = day_of_month.to_string();
    let day_of_week = WEEKDAYS[dow_idx].to_string();

    let mut week_str = "1".to_string();
    let mut all_slots: Vec<RawSlot> = Vec::new();

    if !raw_course_json.trim().is_empty() {
        if let Ok(root) = serde_json::from_str::<Value>(raw_course_json) {
            let data_obj = root.get("data");
            if let Some(d) = data_obj {
                let first_mon = d
                    .get("first_monday_time")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                if first_mon > 0 {
                    let wk = ((today_start_sec - first_mon) / 604800) + 1;
                    if wk > 0 {
                        week_str = wk.to_string();
                    } else if let Some(cw) = d.get("cur_week_num").and_then(|v| v.as_str()) {
                        week_str = cw.to_string();
                    }
                } else if let Some(cw) = d.get("cur_week_num").and_then(|v| v.as_str()) {
                    week_str = cw.to_string();
                }
            }

            let arr = data_obj
                .and_then(|d| d.get("list"))
                .and_then(|v| v.as_array())
                .or_else(|| root.get("course_list").and_then(|v| v.as_array()));

            if let Some(list) = arr {
                for c in list {
                    let name = c
                        .get("course_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let color = c
                        .get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#4AA5F0")
                        .to_string();
                    let groups = match c.get("groups").and_then(|v| v.as_array()) {
                        Some(g) => g,
                        None => continue,
                    };
                    for grp in groups {
                        let loc = grp
                            .get("loc")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let times = match grp.get("times").and_then(|v| v.as_array()) {
                            Some(t) => t,
                            None => continue,
                        };
                        for tup in times {
                            let arr_t = match tup.as_array() {
                                Some(a) if a.len() >= 4 => a,
                                _ => continue,
                            };
                            let start_sec = arr_t[2].as_i64().unwrap_or(0);
                            let dur_sec = arr_t[3].as_i64().unwrap_or(2700);
                            if start_sec > 0 {
                                all_slots.push(RawSlot {
                                    start_sec,
                                    end_sec: start_sec + dur_sec,
                                    name: name.clone(),
                                    loc: loc.clone(),
                                    color: color.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    let mut today_all: Vec<&RawSlot> = all_slots
        .iter()
        .filter(|s| s.start_sec >= today_start_sec && s.start_sec < today_start_sec + 86400)
        .collect();
    today_all.sort_by_key(|s| s.start_sec);

    let today_upcoming: Vec<&RawSlot> = today_all
        .iter()
        .copied()
        .filter(|s| s.end_sec > now_sec)
        .collect();

    let (left_courses, left_empty_text) = if today_upcoming.is_empty() {
        let msg = if today_all.is_empty() {
            "今天没有课程"
        } else {
            "今天已经没有其他课程了"
        };
        (Vec::new(), Some(msg.to_string()))
    } else {
        let items = today_upcoming
            .iter()
            .take(2)
            .map(|s| to_item(s, tz_offset_sec))
            .collect();
        (items, None)
    };

    let (future_date_label, right_courses, right_empty_text, additional_courses_text) =
        if today_upcoming.len() > 2 {
            let overflow = &today_upcoming[2..];
            let items: Vec<WidgetCourseItem> = overflow
                .iter()
                .take(2)
                .map(|s| to_item(s, tz_offset_sec))
                .collect();
            let extra = overflow.len().saturating_sub(2);
            let footer = if extra > 0 {
                format!("今天还有{}节课", extra)
            } else {
                "今天已经没有其他课程了".to_string()
            };
            (None, items, None, Some(footer))
        } else {
            let mut found_label: Option<String> = None;
            let mut found_slots: Vec<&RawSlot> = Vec::new();
            for offset in 1..=14 {
                let d_start = today_start_sec + offset * 86400;
                let mut day_slots: Vec<&RawSlot> = all_slots
                    .iter()
                    .filter(|s| s.start_sec >= d_start && s.start_sec < d_start + 86400)
                    .collect();
                if !day_slots.is_empty() {
                    day_slots.sort_by_key(|s| s.start_sec);
                    found_slots = day_slots;
                    found_label = Some(match offset {
                        1 => "明天".to_string(),
                        2 => "后天".to_string(),
                        _ => format_md(d_start + 43200, tz_offset_sec),
                    });
                    break;
                }
            }
            if found_slots.is_empty() {
                (None, Vec::new(), Some("暂无后续课程".to_string()), None)
            } else {
                let items: Vec<WidgetCourseItem> = found_slots
                    .iter()
                    .take(2)
                    .map(|s| to_item(s, tz_offset_sec))
                    .collect();
                let extra = found_slots.len().saturating_sub(2);
                let footer = if extra > 0 {
                    format!("其他{}节课程", extra)
                } else {
                    "暂无其它课程".to_string()
                };
                (found_label, items, None, Some(footer))
            }
        };

    WidgetViewModel {
        palette,
        date_number,
        week_number: week_str,
        day_of_week,
        left_courses,
        left_empty_text,
        future_date_label,
        right_courses,
        right_empty_text,
        additional_courses_text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_from_days_known_date() {
        // 2026-11-24 is Unix day 20781 (Tuesday)
        let (y, m, d) = civil_from_days(20781);
        assert_eq!((y, m, d), (2026, 11, 24));
        assert_eq!(WEEKDAYS[(20781_i64 + 4).rem_euclid(7) as usize], "二");
    }

    #[test]
    fn test_widget_two_column_layout() {
        let tz = 8 * 3600;
        let today_start = 1795449600 - tz; // 2026-11-24 00:00 CST
        let now = today_start + 7 * 3600;
        let sample = format!(
            r#"{{
              "data": {{
                "cur_week_num": "13",
                "list": [
                  {{"course_name":"近世代数","color":"#f59e0b","groups":[{{"loc":"达夫楼509","times":[[1,2,{},6000,2]]}}]}},
                  {{"course_name":"数学分析","color":"#38bdf8","groups":[{{"loc":"达夫楼609","times":[[1,2,{},6000,3]]}}]}}
                ]
              }}
            }}"#,
            today_start + 8 * 3600,
            today_start + 86400 + 8 * 3600
        );
        let vm = compute_widget_state(&sample, "dark", false, now, today_start, tz);
        assert!(vm.palette.is_dark);
        assert_eq!(vm.week_number, "13");
        assert_eq!(vm.left_courses.len(), 1);
        assert_eq!(vm.left_courses[0].name, "近世代数");
        assert_eq!(vm.future_date_label.as_deref(), Some("明天"));
        assert_eq!(vm.right_courses.len(), 1);
        assert_eq!(vm.right_courses[0].name, "数学分析");
        assert_eq!(vm.additional_courses_text.as_deref(), Some("暂无其它课程"));
    }
}
