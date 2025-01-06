use tui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

pub fn thread_list<'a>(metrics: &Option<(String, String)>, thread_state: &mut TableState, visible_height: usize) -> Table<'a> {
    if let Some((_, thread_metrics)) = metrics {
        let thread_count = thread_metrics.lines().filter(|line| line.contains("\"")).count();
        
        let header = Row::new(vec!["Thread ID", "Thread Name"])
            .style(Style::default().fg(Color::Yellow));

        let rows: Vec<Row> = thread_metrics
            .lines()
            .filter(|line| line.contains("\""))
            .enumerate()
            .map(|(id, line)| {
                let name_start = line.find('"').unwrap();
                let name_end = line[name_start + 1..].find('"').unwrap() + name_start + 1;
                let name = &line[name_start + 1..name_end];

                let style = if Some(id) == thread_state.selected() {
                    Style::default().fg(Color::Green).add_modifier(tui::style::Modifier::BOLD)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(id.to_string()).style(style),
                    Cell::from(name.to_string()).style(style),
                ])
            })
            .collect();

        let title = format!("Thread List - Total: {}", thread_count);

        Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title),
            )
            .highlight_symbol(">>")
            .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)])
    } else {
        Table::new(vec![])
            .block(Block::default().borders(Borders::ALL).title("Thread List - Total: 0"))
    }
}
