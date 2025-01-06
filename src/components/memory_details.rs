
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

use crate::state::State;

pub fn memory_details(state: &State) -> List {
    if let Some((memory_metrics, _)) = &state.metrics {
        let texts: Vec<String> = memory_metrics
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();

        let spans = texts.iter().map(|line| {
            Spans::from(Span::styled(
                line.clone(),
                if line.contains("heap") || line.contains("Metaspace") {
                    Style::default().fg(Color::Cyan)
                } else if line.contains("used") || line.contains("committed") {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                },
            ))
        });

        let items: Vec<ListItem> = spans.map(|span| ListItem::new(span)).collect();
        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory Details"),
            )
    } else {
        List::new(vec![ListItem::new("No JVM selected")])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory Details"),
            )
    }
}
