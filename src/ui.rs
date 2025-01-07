use tui::{
    backend::Backend, layout::{Constraint, Direction, Layout}, 
    text::Span,
    widgets::Block, Frame
};

use crate::{
    app::{App, ViewState},
    components::{
        jvm_list::jvm_list, 
        jvm_summary::jvm_summary, 
        thread_details::thread_details, 
        thread_list::thread_list},
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let app_version=Block::default().title(vec![Span::from(
        "JVM monitoring v0.1.0"
    )]);
    
    match app.view_state {
        ViewState::JVMList => {
            let size = f.size();

            let list_controls_area = Block::default().title(vec![Span::from(
                "[q]quit   [⬆ ⬇]navigate  [r] refresh ",
            )]);
           
            let jvm_chunks=Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(99),
                Constraint::Percentage(1),
            ].as_ref())
            .split(size);


            f.render_stateful_widget(
                jvm_list(&app.state),
                jvm_chunks[0],
                &mut app.state.selected_jvm,
            );
            
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(jvm_chunks[1]);
            let bottom_left = bottom_chunks[0];
            let bottom_right=bottom_chunks[1];

            f.render_widget(list_controls_area, bottom_left);
            f.render_widget(app_version, bottom_right); 
            
            
        }
        ViewState::JVMDetails => {
            let size = f.size();

            // Split layout into memory (top) and threads (bottom) and left the bottom for the navigation buttons
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40), 
                    Constraint::Percentage(59),
                    Constraint::Min(1)]
                    .as_ref())
                .split(size);

            // Split the bottom section into thread list (left) and thread details (right)
            let detail_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(60), 
                    Constraint::Percentage(40)]
                    .as_ref())
                .split(chunks[1]);

            // Render Memory Metrics
            f.render_widget(jvm_summary(&app.state), chunks[0]);

            // Render Thread List
            let thread_list_area = detail_chunks[0];
            {
            f.render_stateful_widget(
                thread_list(&app.state),
                thread_list_area,
                &mut app.state.thread_state,
            );
        }

            
            // Render Thread Details after thread_state borrow ends
            if let Some(selected_thread) = app.state.thread_state.selected() {
                f.render_widget(
                    thread_details(&app.state.metrics, selected_thread),
                    detail_chunks[1],
                );
            }
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(chunks[2]);
            let bottom_left = bottom_chunks[0];
            let bottom_right=bottom_chunks[1];

            // Controls
            let controls_area = Block::default().title(vec![Span::from(
                "[q]quit   [⬆ ⬇]navigate   [b]back ",
            )]);
       
        f.render_widget(controls_area, bottom_left);
        f.render_widget(app_version, bottom_right); 
        }
    }
}
