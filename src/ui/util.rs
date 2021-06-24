use console::{style, Style, StyledObject};

use crate::entities::TaskStatus;

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
pub fn format_test_groups(groups: &Option<Vec<u64>>) -> Option<String> {
    groups.as_ref().map(|groups| {
        let mut text: String = groups
            .iter()
            .map(|&group| group.to_string() + ",")
            .collect();
        text.pop(); // Removes the trailing comma
        text
    })
}

pub fn styled_task_status_or_score(
    status: Option<TaskStatus>,
    score: Option<u64>,
) -> StyledObject<String> {
    if let Some(points) = score {
        return match points {
            points if points >= 100 => style(points.to_string()).black().on_green(),
            points if points > 60 => style(points.to_string()).green(),
            points if points > 10 => style(points.to_string()).yellow(),
            _ => style(points.to_string()).red(),
        }
        .bold();
    }

    match status {
        Some(TaskStatus::Pass) => style("+".to_string()).green(),
        Some(TaskStatus::Fail) => style("X".to_string()).red(),
        Some(TaskStatus::None) | None => style("-".to_string()).dim(),
    }
    .bold()
}
