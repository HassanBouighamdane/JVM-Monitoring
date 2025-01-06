use std::process::Command;

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
    // Fetch memory metrics
    let memory_output = Command::new("jcmd")
        .arg(pid.to_string())
        .arg("GC.heap_info")
        .output()
        .expect("Failed to execute jcmd for memory metrics");

    let memory_metrics = String::from_utf8_lossy(&memory_output.stdout).to_string();

    // Fetch thread metrics
    let thread_output = Command::new("jcmd")
        .arg(pid.to_string())
        .arg("Thread.print")
        .output()
        .expect("Failed to execute jcmd for thread metrics");

    let thread_metrics = String::from_utf8_lossy(&thread_output.stdout).to_string();

    (memory_metrics, thread_metrics)
}
