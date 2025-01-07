use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

pub fn thread_details(metrics: &Option<(String, String)>, selected_thread: usize) -> Table {
    if let Some((_, thread_metrics)) = metrics {
        let lines: Vec<&str> = thread_metrics.lines().collect();

        // Find the selected thread's name line
        let thread_lines: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains("\""))
            .map(|(index, _)| index)
            .collect();

        if let Some(&selected_index) = thread_lines.get(selected_thread) {
            let line = lines[selected_index];

            // Extract thread name
            let name_start = line.find('"').unwrap();
            let name_end = line[name_start + 1..].find('"').unwrap() + name_start + 1;
            let name = line[name_start + 1..name_end].to_string(); // Convert to owned String

            // Extract priority
            let priority = line
                .find("prio=")
                .and_then(|start| line[start + 5..].split_whitespace().next())
                .map(|s| s.to_string()) // Convert to owned String
                .unwrap_or_else(|| "Unknown".to_string());

            // Extract CPU time
            let cpu_time = line
                .find("cpu=")
                .and_then(|start| line[start + 4..].split_whitespace().next())
                .map(|s| s.to_string()) // Convert to owned String
                .unwrap_or_else(|| "Unknown".to_string());

            // Extract elapsed time
            let elapsed_time = line
                .find("elapsed=")
                .and_then(|start| line[start + 8..].split_whitespace().next())
                .map(|s| s.to_string()) // Convert to owned String
                .unwrap_or_else(|| "Unknown".to_string());

            // Extract thread state
            let state = lines
                .iter()
                .skip(selected_index + 1) // Start looking after the current thread's line
                .find(|line| line.trim_start().starts_with("java.lang.Thread.State:"))
                .map(|line| {
                    let state_start = line.find("java.lang.Thread.State:").unwrap() + 23; // Length of "java.lang.Thread.State:"
                    line[state_start..].trim().to_string() // Convert to owned String
                })
                .unwrap_or_else(|| "UNKNOWN".to_string());

            // Populate rows for the table with styled cells
            let rows = vec![
                Row::new(vec![
                    Cell::from("Thread Name").style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)),
                    Cell::from(name).style(Style::default().fg(Color::White)),
                ]),
                Row::new(vec![
                    Cell::from("State").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Cell::from(state).style(Style::default().fg(Color::White)),
                ]),
                Row::new(vec![
                    Cell::from("Priority").style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Cell::from(priority).style(Style::default().fg(Color::White)),
                ]),
                Row::new(vec![
                    Cell::from("CPU Time").style(Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC)),
                    Cell::from(cpu_time).style(Style::default().fg(Color::White)),
                ]),
                Row::new(vec![
                    Cell::from("Elapsed Time").style(Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC)),
                    Cell::from(elapsed_time).style(Style::default().fg(Color::White)),
                ]),
            ];

            return Table::new(rows)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Thread Details")
                        .border_style(Style::default().fg(Color::White)),
                )
                .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)]);
        }
    }

    Table::new(vec![])
        .block(Block::default().borders(Borders::ALL).title("Thread Details"))
}
