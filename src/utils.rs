
pub fn parse_memory_line(line: &str, remove_index: bool) -> String {
    let cleaned_line = if remove_index {
        line.split('[').next().unwrap_or(line).trim()
    } else {
        line
    };

    cleaned_line
        .split_whitespace()
        .map(|word| {
            if word.ends_with("K") {
                if let Ok(kb) = word[..word.len() - 1].parse::<f64>() {
                    format!("{:.2} MB", kb / 1024.0)
                } else {
                    word.to_string()
                }
            } else if word.ends_with("MB") {
                word.to_string()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}