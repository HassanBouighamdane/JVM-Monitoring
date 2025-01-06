use tui::{
    layout::Constraint,
    widgets::{Block, Borders, Row, Table},
};

pub fn thread_details(metrics: &Option<(String, String)>, selected_thread: usize) -> Table {
    if let Some((_, thread_metrics)) = metrics {
        let lines: Vec<&str> = thread_metrics
            .lines()
            .filter(|line| line.contains("\""))
            .collect();

        if let Some(line) = lines.get(selected_thread) {
            let name_start = line.find('"').unwrap();
            let name_end = line[name_start + 1..].find('"').unwrap() + name_start + 1;
            let name = &line[name_start + 1..name_end];

            let state_start = line.find("java.lang.Thread.State:").unwrap_or(0);
            let state = line[state_start..]
                .split_whitespace()
                .nth(3)
                .unwrap_or("UNKNOWN");

            let rows = vec![
                Row::new(vec!["Thread Id".to_string(), name.to_string()]),
                Row::new(vec!["Thread Name".to_string(), name.to_string()]),
                Row::new(vec!["State".to_string(), state.to_string()]),
            ];

            return Table::new(rows)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Thread Details"),
                )
                .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)]);
        }
    }

    Table::new(vec![])
        .block(Block::default().borders(Borders::ALL).title("Thread Details"))
}
