use tui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::state::State;

pub fn jvm_list<'a>(state: &State) -> Table<'a> {
    let header = Row::new(vec!["PID", "Description"])
        .style(Style::default().fg(Color::LightRed));

    let rows: Vec<Row> = state.jvms
        .iter()
        .enumerate()
        .map(|(index, (pid, desc))| {
            let style = if state.selected_jvm.selected() == Some(index) {
                Style::default().fg(Color::Black).bg(Color::White).add_modifier(tui::style::Modifier::BOLD)
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(pid.to_string()).style(style),
                Cell::from(desc.clone()).style(style),
            ])
        })
        .collect();

    Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("JVM List"),
        )
        //.highlight_symbol(">>")
        .widths(&[Constraint::Percentage(20), Constraint::Percentage(80)])
}
