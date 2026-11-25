use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HolidayMatch {
    pub id: &'static str,
    pub name: &'static str,
    pub tag: &'static str,
}

pub fn match_holiday(year: i32, month: u32, day: u32, weekday_sun0: u32) -> Option<HolidayMatch> {
    // Verified 2026-2028 lunar-to-solar statutory & traditional dates
    let exact: Option<(&'static str, &'static str, &'static str)> = match (year, month, day) {
        (2026, 2, 17) | (2027, 2, 6) | (2028, 1, 26) => Some(("spring", "春节", "法定假日")),
        (2026, 3, 3) | (2027, 2, 20) | (2028, 2, 9) => Some(("lantern", "元宵节", "传统佳节")),
        (2026, 4, 5) | (2027, 4, 5) | (2028, 4, 4) => Some(("qingming", "清明节", "法定假日")),
        (2026, 6, 19) | (2027, 6, 9) | (2028, 5, 28) => Some(("dragonboat", "端午节", "法定假日")),
        (2026, 8, 19) | (2027, 8, 8) | (2028, 8, 26) => Some(("qixi", "七夕节", "传统佳节")),
        (2026, 9, 25) | (2027, 9, 15) | (2028, 10, 3) => Some(("midautumn", "中秋节", "法定假日")),
        (2026, 10, 18) | (2027, 10, 8) | (2028, 10, 26) => Some(("chongyang", "重阳节", "传统佳节")),
        (2026, 12, 22) | (2027, 12, 22) | (2028, 12, 21) => Some(("solstice", "冬至", "传统节气")),
        _ => None,
    };

    if let Some((id, name, tag)) = exact {
        return Some(HolidayMatch { id, name, tag });
    }

    let fixed: Option<(&'static str, &'static str, &'static str)> = match (month, day) {
        (1, 1) => Some(("newyear", "元旦", "法定假日")),
        (2, 14) => Some(("valentine", "情人节", "节日彩蛋")),
        (3, 8) => Some(("womens", "妇女节", "节日彩蛋")),
        (4, 1) => Some(("aprilfools", "愚人节", "节日彩蛋")),
        (5, 1) => Some(("labor", "劳动节", "法定假日")),
        (5, 4) => Some(("youth", "青年节", "节日彩蛋")),
        (6, 1) => Some(("children", "儿童节", "节日彩蛋")),
        (9, 10) => Some(("teachers", "教师节", "节日彩蛋")),
        (10, 1) => Some(("national", "国庆节", "法定假日")),
        (10, 31) => Some(("halloween", "万圣夜", "节日彩蛋")),
        (12, 25) => Some(("christmas", "圣诞节", "节日彩蛋")),
        _ => None,
    };

    if let Some((id, name, tag)) = fixed {
        return Some(HolidayMatch { id, name, tag });
    }

    match weekday_sun0 {
        6 => Some(HolidayMatch {
            id: "saturday",
            name: "周六",
            tag: "周末时光",
        }),
        0 => Some(HolidayMatch {
            id: "sunday",
            name: "周日",
            tag: "周末时光",
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holiday_lookup() {
        assert_eq!(match_holiday(2026, 10, 1, 4).unwrap().id, "national");
        assert_eq!(match_holiday(2026, 10, 18, 0).unwrap().id, "chongyang");
        assert_eq!(match_holiday(2026, 11, 28, 6).unwrap().id, "saturday");
        assert!(match_holiday(2026, 11, 24, 2).is_none());
    }
}
