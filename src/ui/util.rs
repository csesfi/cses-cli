use console::{Style, StyledObject};

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
