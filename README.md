# JVM Monitoring Tool

This project is a terminal-based application developed in Rust using the TUI (Text-based User Interface) library. It provides a real-time monitoring solution for Java Virtual Machines (JVMs), focusing on essential metrics like heap memory, metaspace, and thread details.

## Features

1. **JVM List Navigation**:
   - View a list of running JVMs on your system.
   - Navigate through the JVM list using intuitive keyboard controls.

2. **Real-time JVM Metrics**:
   - Monitor heap memory usage, metaspace, and class space with real-time updates.
   - View memory metrics in a user-friendly format, including conversions to MB for better readability.

3. **Thread Monitoring**:
   - Access a detailed list of active threads for a selected JVM.
   - Inspect thread details such as state, priority, CPU time, and elapsed time.

4. **User-friendly Interface**:
   - A clean and interactive terminal interface built with the `tui` crate.
   - Organized display for JVMs, threads, and memory metrics for ease of navigation.

## Prerequisites

Ensure the following are installed on your system:

1. **Rust**:
   - Install Rust from the [official Rust website](https://www.rust-lang.org/tools/install).

2. **Java**:
   - Ensure Java is installed and added to your system's PATH.
   - The tool relies on the `jcmd` command-line utility, which is part of the Java Development Kit (JDK).

## Getting Started

### Clone the Repository

Clone the project repository to your local machine:

```bash
git clone https://github.com/HassanBouighamdane/JVM-Monitoring.git
cd JVM-Monitoring
```
## Build and Run
Build and run the application using the following commands:

  ```bash
  cargo build --release
  cargo run
  ```

## Locate the Executable
After a successful build, find the executable in the `target/release` directory within the project folder.

## Run the Executable
You can now run the executable from any location on your machine using the following command:

- On Unix-based systems:

  ```bash
  ./jvm_monitoring
  ```

- On Windows:

  ```bash
  .\jvm_monitoring.exe
  ```
