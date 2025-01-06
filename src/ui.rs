use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    app::{App, ViewState},
    components::{jvm_list::jvm_list, jvm_summary::jvm_summary, thread_details::thread_details, thread_list::thread_list},
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.view_state {
        ViewState::JVMList => {
            let size = f.size();
            f.render_stateful_widget(
                jvm_list(&app.state.jvms, &app.state.selected_jvm),
                size,
                &mut app.state.selected_jvm,
            );
        }
        ViewState::JVMDetails => {
            let size = f.size();

            // Split layout into memory (top) and threads (bottom)
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(size);

            // Split the bottom section into thread list (left) and thread details (right)
            let detail_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            // Render Memory Metrics
            f.render_widget(jvm_summary(&app.state.metrics), chunks[0]);

            // Render Thread List
            let thread_list_area = detail_chunks[0];
            {
                let thread_state = &mut app.state.thread_state;
                let visible_height = thread_list_area.height as usize - 2; // Adjust height as needed

                f.render_stateful_widget(
                    thread_list(&app.state.metrics, thread_state, visible_height),
                    thread_list_area,
                    thread_state,
                );
            }

            // Render Thread Details after thread_state borrow ends
            if let Some(selected_thread) = app.state.thread_state.selected() {
                f.render_widget(
                    thread_details(&app.state.metrics, selected_thread),
                    detail_chunks[1],
                );
            }
            
        }
    }
}
