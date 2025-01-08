use tui::widgets::TableState;
use crate::metrics::{fetch_jvms,fetch_jvm_metrics};
pub struct State {
    pub jvms: Vec<(u32, String)>,          
    pub selected_jvm: TableState,          
    pub metrics: Option<(String, String)>,
    pub thread_state: TableState,         
}

impl State {
    pub fn new() -> State {
        State {
            jvms: fetch_jvms(),
            selected_jvm: TableState::default(),
            metrics: None,
            thread_state: TableState::default(),
        }
    }

    pub fn refresh_jvm_list(&mut self) {
        self.jvms = fetch_jvms();
        self.selected_jvm.select(None); 
    }
    
    pub fn refresh_jvm_details(&mut self) {
        if let Some(selected) = self.selected_jvm.selected() {
            if let Some((pid, _)) = self.jvms.get(selected) {
                self.metrics = Some(fetch_jvm_metrics(*pid));
            }
        }
    }

    pub fn select_next_jvm(&mut self) {
        let i = match self.selected_jvm.selected() {
            Some(i) => {
                if i >= self.jvms.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_jvm.select(Some(i));
        self.metrics = None;
    }

    pub fn select_previous_jvm(&mut self) {
        let i = match self.selected_jvm.selected() {
            Some(i) => if i == 0 { self.jvms.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.selected_jvm.select(Some(i));
        self.metrics = None; 
    }

    pub fn select_next_thread(&mut self) {
        if let Some((_, thread_metrics)) = &self.metrics {
            let thread_count = thread_metrics.lines().filter(|line| line.contains("\"")).count();
            let i = match self.thread_state.selected() {
                Some(i) => (i + 1) % thread_count,
                None => 0,
            };
            self.thread_state.select(Some(i));
        }
    }

    pub fn select_previous_thread(&mut self) {
        if let Some((_, thread_metrics)) = &self.metrics {
            let thread_count = thread_metrics.lines().filter(|line| line.contains("\"")).count();
            let i = match self.thread_state.selected() {
                Some(i) => if i == 0 { thread_count - 1 } else { i - 1 },
                None => 0,
            };
            self.thread_state.select(Some(i));
        }
    }
}

