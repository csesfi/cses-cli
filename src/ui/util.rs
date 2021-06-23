use console::{style, Style, StyledObject};

use crate::entities::CourseTaskStatus;

pub fn result_with_color(line: &str) -> StyledObject<&str> {
    let color = match line {
        "ACCEPTED" => Style::new().green(),
        "UNKNOWN" => Style::new().white(),
        _ => Style::new().red(),
    };
    color.apply_to(line)
}
pub fn format_code_time(time: Option<u64>) -> String {
    match time {
        Some(time) => format!("{:.2} s", time as f64 / 1000.0),
        None => "--".to_owned(),
    }
}
pub fn format_test_groups(groups: &Option<Vec<u64>>) -> String {
    match groups {
        Some(groups) => {
            let mut text: String = groups
                .iter()
                .map(|&group| group.to_string() + ",")
                .collect();
            text.pop(); // Removes the trailing comma
            text
        }
        None => "".to_owned(),
    }
}

pub fn styled_task_status_or_score(
    status: Option<CourseTaskStatus>,
    score: Option<u64>,
) -> StyledObject<String> {
    if let Some(points) = score {
        return match points {
            points if points >= 100 => style(points.to_string()).black().on_green(),
            points if points > 60 => style(points.to_string()).green(),
            points if points > 10 => style(points.to_string()).yellow(),
            _ => style(points.to_string()).red(),
        };
    }

    match status {
        Some(CourseTaskStatus::Pass) => style("+".to_string()).green(),
        Some(CourseTaskStatus::Fail) => style("X".to_string()).red(),
        Some(CourseTaskStatus::None) => style("-".to_string()).dim(),
        None => style("-".to_string()).dim(),
    }
}
