use std::process::Command;
use std::str;
use std::io::{self, Write};

fn get_command_output(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Command failed with status: {:?}, stderr: {}",
            output.status,
            str::from_utf8(&output.stderr).unwrap_or("Unable to read stderr")
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn format_size(bytes: u64) -> String {
    let gb = bytes as f64 / 1_073_741_824.0;
    format!("{:.2} GB", gb)
}

fn main() {
    // CPU
    println!("=== CPU Information ===\n");
    match get_command_output("powershell", &["Get-WmiObject", "Win32_Processor"]) {
        Ok(cpu_info) => {
            for line in cpu_info.lines() {
                if line.contains("Name") || line.contains("MaxClockSpeed") {
                    println!("{}", line.trim());
                }
            }
        }
        Err(e) => eprintln!("Error retrieving CPU info: {}", e),
    }

    // Motherboard
    println!("\n=== Motherboard Information ===\n");
    match get_command_output("powershell", &["Get-WmiObject", "Win32_BaseBoard"]) {
        Ok(motherboard_info) => {
            for line in motherboard_info.lines() {
                if line.contains("Manufacturer") || line.contains("Product") || line.contains("SerialNumber") {
                    println!("{}", line.trim());
                }
            }
        }
        Err(e) => eprintln!("Error retrieving Motherboard info: {}", e),
    }

    // GPU
    println!("\n=== GPU Information ===\n");
    match get_command_output("powershell", &["Get-WmiObject", "Win32_VideoController"]) {
        Ok(gpu_info) => {
            for line in gpu_info.lines() {
                if line.contains("Name") || line.contains("AdapterRAM") {
                    println!("{}", line.trim());
                }
            }
        }
        Err(e) => eprintln!("Error retrieving GPU info: {}", e),
    }

    // Memory (RAM)
    println!("\n=== Memory (RAM) Information ===\n");
    match get_command_output("powershell", &["Get-WmiObject", "Win32_PhysicalMemory"]) {
        Ok(memory_info) => {
            let mut total_memory: u64 = 0;
            for line in memory_info.lines() {
                if line.contains("Capacity") {
                    let capacity_str = line.split(":").nth(1).unwrap_or("0").trim();
                    match capacity_str.parse::<u64>() {
                        Ok(capacity) => total_memory += capacity,
                        Err(_) => eprintln!("Error parsing memory capacity: {}", capacity_str),
                    }
                } else if line.contains("Speed") || line.contains("Manufacturer") {
                    println!("{}", line.trim());
                }
            }
            println!("Total Memory: {}", format_size(total_memory));
        }
        Err(e) => eprintln!("Error retrieving Memory info: {}", e),
    }

    // Disks
    println!("\n=== Disk Information ===\n");
    match get_command_output("powershell", &["Get-WmiObject", "Win32_DiskDrive"]) {
        Ok(disk_info) => {
            for line in disk_info.lines() {
                if line.contains("Model") {
                    println!("{}", line.trim());
                } else if line.contains("Size") {
                    let size_str = line.split(":").nth(1).unwrap_or("0").trim();
                    match size_str.parse::<u64>() {
                        Ok(size) => println!("Size       : {}", format_size(size)),
                        Err(_) => eprintln!("Error parsing disk size: {}", size_str),
                    }
                }
            }
        }
        Err(e) => eprintln!("Error retrieving Disk info: {}", e),
    }

    // Keep the console window open
    print!("\nPress Enter to exit...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}
