use std::process::Command;
use tui::widgets::TableState;

pub struct State {
    pub jvms: Vec<(u32, String)>,          // List of JVMs
    pub selected_jvm: TableState,          // Tracks the selected JVM
    pub metrics: Option<(String, String)>, // Memory and thread metrics for the selected JVM
    pub thread_state: TableState,          // Tracks the selected thread in the thread list
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

    /// Refresh data for the currently selected JVM
    pub fn refresh(&mut self) {
        if let Some(selected) = self.selected_jvm.selected() {
            if let Some((pid, _)) = self.jvms.get(selected) {
                self.metrics = Some(fetch_jvm_metrics(*pid));
            }
        }
    }

    /// Select the next JVM in the list
    pub fn select_next_jvm(&mut self) {
        let i = match self.selected_jvm.selected() {
            Some(i) => (i + 1) % self.jvms.len(),
            None => 0,
        };
        self.selected_jvm.select(Some(i));
        self.metrics = None; // Clear metrics when switching JVMs
    }

    /// Select the previous JVM in the list
    pub fn select_previous_jvm(&mut self) {
        let i = match self.selected_jvm.selected() {
            Some(i) => if i == 0 { self.jvms.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.selected_jvm.select(Some(i));
        self.metrics = None; // Clear metrics when switching JVMs
    }

    /// Select the next thread in the thread list
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

    /// Select the previous thread in the thread list
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

pub fn fetch_jvms() -> Vec<(u32, String)> {
    let output = Command::new("jcmd")
        .output()
        .expect("Failed to execute jcmd command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                Some((parts[0].parse::<u32>().unwrap_or(0), parts[1..].join(" ")))
            } else {
                None
            }
        })
        .collect()
}

pub fn fetch_jvm_metrics(pid: u32) -> (String, String) {
    let memory_output = Command::new("jcmd")
        .arg(pid.to_string())
        .arg("GC.heap_info")
        .output()
        .expect("Failed to execute jcmd for memory metrics");

    let memory_metrics = String::from_utf8_lossy(&memory_output.stdout).to_string();

    let thread_output = Command::new("jcmd")
        .arg(pid.to_string())
        .arg("Thread.print")
        .output()
        .expect("Failed to execute jcmd for thread metrics");

    let thread_metrics = String::from_utf8_lossy(&thread_output.stdout).to_string();

    (memory_metrics, thread_metrics)
}
