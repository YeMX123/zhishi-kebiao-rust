use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct ParsedCourse {
    pub name: String,
    pub day: i64,
    pub start: i64,
    pub end: i64,
    pub start_week: i64,
    pub end_week: i64,
    pub week_type: i64,
    pub room: String,
    pub teacher: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WakeupParseResult {
    pub ok: bool,
    pub start_date: String,
    pub total_weeks: i64,
    pub times: Vec<[String; 2]>,
    pub courses: Vec<ParsedCourse>,
    pub error: Option<String>,
}

pub fn parse_wakeup_ndjson(raw: &str) -> WakeupParseResult {
    let lines: Vec<&str> = raw
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    if lines.len() < 5 {
        return WakeupParseResult {
            ok: false,
            start_date: String::new(),
            total_weeks: 20,
            times: Vec::new(),
            courses: Vec::new(),
            error: Some("WakeUp 文件格式不完整（少于 5 行 JSON）".to_string()),
        };
    }

    let mut times: Vec<[String; 2]> = Vec::new();
    if let Ok(arr) = serde_json::from_str::<Vec<Value>>(lines[1]) {
        for item in arr {
            let st = item.get("startTime").and_then(|v| v.as_str()).unwrap_or("");
            let en = item.get("endTime").and_then(|v| v.as_str()).unwrap_or("");
            if !st.is_empty() && !en.is_empty() {
                times.push([st.to_string(), en.to_string()]);
            }
        }
    }

    let mut start_date = String::new();
    let mut total_weeks = 20_i64;
    if let Ok(meta) = serde_json::from_str::<Value>(lines[2]) {
        if let Some(sd) = meta.get("startDate").and_then(|v| v.as_str()) {
            start_date = sd.to_string();
        }
        if let Some(tw) = meta.get("maxWeek").and_then(|v| v.as_i64()) {
            if tw > 0 {
                total_weeks = tw;
            }
        }
    }

    let mut id_to_name: HashMap<i64, String> = HashMap::new();
    if let Ok(defs) = serde_json::from_str::<Vec<Value>>(lines[3]) {
        for d in defs {
            if let (Some(id), Some(name)) = (
                d.get("id").and_then(|v| v.as_i64()),
                d.get("courseName").and_then(|v| v.as_str()),
            ) {
                id_to_name.insert(id, name.to_string());
            }
        }
    }

    let mut courses: Vec<ParsedCourse> = Vec::new();
    if let Ok(slots) = serde_json::from_str::<Vec<Value>>(lines[4]) {
        for s in slots {
            let id = s.get("id").and_then(|v| v.as_i64()).unwrap_or(-1);
            let name = id_to_name
                .get(&id)
                .cloned()
                .unwrap_or_else(|| "未命名课程".to_string());
            let day = s.get("day").and_then(|v| v.as_i64()).unwrap_or(1);
            let start = s.get("startNode").and_then(|v| v.as_i64()).unwrap_or(1);
            let step = s.get("step").and_then(|v| v.as_i64()).unwrap_or(2).max(1);
            let end = start + step - 1;
            let start_week = s.get("startWeek").and_then(|v| v.as_i64()).unwrap_or(1);
            let end_week = s
                .get("endWeek")
                .and_then(|v| v.as_i64())
                .unwrap_or(total_weeks);
            let week_type = s.get("type").and_then(|v| v.as_i64()).unwrap_or(0);
            let room = s
                .get("room")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let teacher = s
                .get("teacher")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();

            courses.push(ParsedCourse {
                name,
                day,
                start,
                end,
                start_week,
                end_week,
                week_type,
                room,
                teacher,
            });
        }
    }

    WakeupParseResult {
        ok: true,
        start_date,
        total_weeks,
        times,
        courses,
        error: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wakeup_5_lines() {
        let sample = r#"{"courseLen":50}
[{"node":1,"startTime":"08:00","endTime":"08:50"},{"node":2,"startTime":"09:00","endTime":"09:50"}]
{"startDate":"2026-08-31","maxWeek":20}
[{"id":1,"courseName":"数据结构B"}]
[{"id":1,"day":1,"startNode":1,"step":2,"startWeek":1,"endWeek":16,"type":0,"room":"三山楼508","teacher":"张老师"}]"#;
        let res = parse_wakeup_ndjson(sample);
        assert!(res.ok);
        assert_eq!(res.start_date, "2026-08-31");
        assert_eq!(res.times.len(), 2);
        assert_eq!(res.courses.len(), 1);
        assert_eq!(res.courses[0].name, "数据结构B");
        assert_eq!(res.courses[0].end, 2);
        assert_eq!(res.courses[0].room, "三山楼508");
    }
}
