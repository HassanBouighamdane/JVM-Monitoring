use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::state::State;

pub fn thread_list<'a>(state: &State) -> Table<'a> {
    if let Some((_, thread_metrics)) = &state.metrics {
        let thread_count = thread_metrics.lines().filter(|line| line.contains("\"")).count();

        let header = Row::new(vec!["Thread ID", "Thread Name"])
            .style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD));

        let rows: Vec<Row> = thread_metrics
            .lines()
            .filter(|line| line.contains("\""))
            .enumerate()
            .map(|(id, line)| {
                let name_start = line.find('"').unwrap();
                let name_end = line[name_start + 1..].find('"').unwrap() + name_start + 1;
                let name = &line[name_start + 1..name_end];

                Row::new(vec![
                    Cell::from(id.to_string()),
                    Cell::from(name.to_string()),
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
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            //.highlight_symbol(">>") 
            .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)])
    } else {
        Table::new(vec![])
            .block(Block::default().borders(Borders::ALL).title("Thread List - Total: 0"))
    }
}
