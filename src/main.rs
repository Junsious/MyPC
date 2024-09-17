use std::process::Command;
use std::str;
use std::io::{self, Write};

fn get_command_output(command: &str, args: &[&str]) -> Vec<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();
    stdout.lines().map(|line| line.trim().to_string()).collect()
}

fn format_size(bytes: u64) -> String {
    let gb = bytes as f64 / 1_073_741_824.0;
    format!("{:.2} GB", gb)
}

fn main() {
    // CPU
    println!("=== CPU Information ===\n");
    let cpu_info = get_command_output("powershell", &["Get-WmiObject", "Win32_Processor"]);
    for line in cpu_info {
        if line.contains("Name") || line.contains("MaxClockSpeed") {
            println!("{}", line);
        }
    }

    // Motherboard
    println!("\n=== Motherboard Information ===\n");
    let motherboard_info = get_command_output("powershell", &["Get-WmiObject", "Win32_BaseBoard"]);
    for line in motherboard_info {
        if line.contains("Manufacturer") || line.contains("Product") || line.contains("SerialNumber") {
            println!("{}", line);
        }
    }

    // GPU
    println!("\n=== GPU Information ===\n");
    let gpu_info = get_command_output("powershell", &["Get-WmiObject", "Win32_VideoController"]);
    for line in gpu_info {
        if line.contains("Name") || line.contains("AdapterRAM") {
            println!("{}", line);
        }
    }

    // Memory (RAM)
    println!("\n=== Memory (RAM) Information ===\n");
    let memory_info = get_command_output("powershell", &["Get-WmiObject", "Win32_PhysicalMemory"]);
    let mut total_memory: u64 = 0;
    for line in memory_info {
        if line.contains("Capacity") {
            let capacity_str = line.split(":").nth(1).unwrap().trim();
            let capacity: u64 = capacity_str.parse().unwrap_or(0);
            total_memory += capacity;
        } else if line.contains("Speed") || line.contains("Manufacturer") {
            println!("{}", line);
        }
    }
    println!("Total Memory: {}", format_size(total_memory));

    // Disks
    println!("\n=== Disk Information ===\n");
    let disk_info = get_command_output("powershell", &["Get-WmiObject", "Win32_DiskDrive"]);
    for line in disk_info {
        if line.contains("Model") {
            println!("{}", line);
        } else if line.contains("Size") {
            let size_str = line.split(":").nth(1).unwrap().trim();
            let size: u64 = size_str.parse().unwrap_or(0);
            println!("Size       : {}", format_size(size));
        }
    }

    // Keep the console window open
    print!("\nPress Enter to exit...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}
