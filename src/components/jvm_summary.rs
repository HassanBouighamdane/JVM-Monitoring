use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

pub fn jvm_summary(metrics: &Option<(String, String)>) -> List {
    if let Some((memory_metrics, _)) = metrics {
        // Parse and format the memory metrics
        let lines: Vec<&str> = memory_metrics.lines().collect();
        let formatted_lines = vec![
            "Garbage-First Heap:".to_string(),
            lines.get(1).unwrap_or(&"").trim().to_string(), // Total heap info
            lines.get(2).unwrap_or(&"").trim().to_string(), // Region size and survivors
            "".to_string(),
            "Metaspace:".to_string(),
            lines.get(3).unwrap_or(&"").trim().to_string(), // Metaspace usage
            lines.get(4).unwrap_or(&"").trim().to_string(), // Class space usage
        ];

        // Style the lines
        let items: Vec<ListItem> = formatted_lines
            .into_iter()
            .map(|line| {
                let styled_line = if line.starts_with("Garbage-First Heap:")
                    || line.starts_with("Metaspace:")
                {
                    Span::styled(line, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                } else if line.contains("used") || line.contains("committed") || line.contains("reserved") {
                    Span::styled(line, Style::default().fg(Color::Green))
                } else {
                    Span::raw(line)
                };
                ListItem::new(Spans::from(styled_line))
            })
            .collect();

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
