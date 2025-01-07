use crossterm::event::{self, Event, KeyCode};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{backend::Backend, Terminal};

use crate::{state::State, ui::ui};

pub enum ViewState {
    JVMList,
    JVMDetails,
}

pub struct App {
    pub state: State,
    pub view_state: ViewState,
}

impl App {
    pub fn new() -> App {
        App {
            state: State::new(),
            view_state: ViewState::JVMList,
        }
    }

    /// Refresh state on tick
    pub fn on_tick(&mut self) {
        // Preserve the currently selected thread to avoid losing it during updates
        let current_selected_thread = self.state.thread_state.selected();
    
        // Refresh the state (fetch new memory and thread metrics for the selected JVM)
        self.state.refresh();
    
        // Restore the previously selected thread if still valid
        if let Some((_, thread_metrics)) = &self.state.metrics {
            let thread_count = thread_metrics.lines().filter(|line| line.contains("\"")).count();
            if let Some(selected_thread) = current_selected_thread {
                if selected_thread < thread_count {
                    self.state.thread_state.select(Some(selected_thread));
                } else {
                    self.state.thread_state.select(Some(thread_count - 1));
                }
            }
        }
    }
    

    /// Run the application loop
    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            // Draw the UI
            terminal.draw(|f| ui(f, self))?;
    
            // Wait for a key event or timeout
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
    
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match self.view_state {
                        ViewState::JVMList => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Down => self.state.select_next_jvm(), 
                            KeyCode::Up => self.state.select_previous_jvm(), 
                            KeyCode::Enter => {
                                self.state.refresh(); 
                                self.view_state = ViewState::JVMDetails;
                            },
                            KeyCode::Char('r') => {
                                self.state.refresh_jvm_list();
                            }
                            _ => {}
                        },
                        ViewState::JVMDetails => match key.code {
                            KeyCode::Char('q') => return Ok(()), // Quit
                            KeyCode::Down => self.state.select_next_thread(), // Navigate thread list
                            KeyCode::Up => self.state.select_previous_thread(), // Navigate thread list
                            KeyCode::Char('b') => {
                                self.state.refresh_jvm_list();
                                self.view_state = ViewState::JVMList; }
                            _ => {}
                        },
                    }
                }
            }
    
            // Refresh the state periodically based on the tick rate
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }
    
}
