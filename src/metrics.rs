use std::process::Command;

pub fn fetch_metrics(pid: u32) -> (String, String) {
    let memory_output = Command::new("jcmd")
        .arg(pid.to_string())
        .arg("GC.heap_info")
        .output()
        .expect("Failed to execute jcmd");

    let threads_output = Command::new("jcmd")
        .arg(pid.to_string())
        .arg("Thread.print")
        .output()
        .expect("Failed to execute jcmd");

    let memory = String::from_utf8_lossy(&memory_output.stdout).to_string();
    let threads = String::from_utf8_lossy(&threads_output.stdout).to_string();

    (memory, threads)
}
