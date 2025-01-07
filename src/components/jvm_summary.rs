use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::state::State;

fn parse_memory_line(line: &str, remove_index: bool) -> String {
    let cleaned_line = if remove_index {
        // Remove memory index part if exists
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

pub fn jvm_summary(state: &State) -> List {
    if let Some((memory_metrics, _)) = &state.metrics {
        // Parse and format the memory metrics
        let lines: Vec<&str> = memory_metrics.lines().collect();

        // Heap memory data
        let heap_info = parse_memory_line(lines.get(1).unwrap_or(&"").trim(), true);
        let region_info = parse_memory_line(lines.get(2).unwrap_or(&"").trim(), false);

        // Metaspace and Class space data
        let metaspace_info = parse_memory_line(lines.get(3).unwrap_or(&"").trim(), false);
        let class_space_info = parse_memory_line(lines.get(4).unwrap_or(&"").trim(), false);

        // Add selected JVM PID and description
        let jvm_info = if let Some(selected) = state.selected_jvm.selected() {
            if let Some((pid, description)) = state.jvms.get(selected) {
                vec![
                    Spans::from(vec![
                        Span::styled("JVM PID: ", Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)),
                        Span::raw(pid.to_string()),
                    ]),
                    Spans::from(vec![
                        Span::styled(
                            "JVM Description: ",
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(description.to_string()),
                    ]),
                ]
            } else {
                vec![Spans::from(Span::raw("No JVM selected."))]
            }
        } else {
            vec![Spans::from(Span::raw("No JVM selected."))]
        };

        // Build structured memory details
        let memory_lines = vec![
            Spans::from(vec![Span::styled("Heap Memory:", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))]),
            Spans::from(vec![Span::raw(format!("  Total: {}", heap_info.split(',').next().unwrap_or("Unknown")))]),
            Spans::from(vec![Span::raw(format!("  Used: {}", heap_info.split(',').nth(1).unwrap_or("Unknown")))]),
            Spans::from(vec![Span::raw(format!("  {}", region_info))]), // Region size and survivors
            Spans::from(vec![Span::styled("Metaspace:", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))]),
            Spans::from(vec![Span::raw(format!("  {}", metaspace_info))]),
            Spans::from(vec![Span::styled("Class Space:", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))]),
            Spans::from(vec![Span::raw(format!("  {}", class_space_info))]),
        ];

        // Combine all lines
        let combined_lines = [
            vec![Spans::from(vec![Span::styled("JVM Information:", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))])],
            jvm_info,
            vec![Spans::from(Span::raw(""))],
            memory_lines,
        ]
        .concat();

        // Create styled ListItems
        let items: Vec<ListItem> = combined_lines.into_iter().map(ListItem::new).collect();

        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("JVM Summary"),
        )
    } else {
        List::new(vec![ListItem::new("No data available.")])
            .block(Block::default().borders(Borders::ALL).title("JVM Summary"))
    }
}
