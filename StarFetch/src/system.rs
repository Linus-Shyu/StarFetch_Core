use ansi_term::Color::{Cyan, Green};
use sysinfo::System as SysInfoSystem;
use systemstat::{Platform, System};
use std::process::Command;

// Init the system library
pub fn init_system() -> SysInfoSystem {
    let mut sys = SysInfoSystem::new_all();
    sys.refresh_all();
    sys
}

// Helper function to calculate maximum width across all system info lines
fn calculate_max_info_width() -> usize {
    let mut max_len = 0;
    let mut sys = SysInfoSystem::new_all();
    sys.refresh_all();
    
    // Hostname
    let host_name = SysInfoSystem::host_name().unwrap_or_else(|| "Unknown".to_string());
    max_len = max_len.max(host_name.len());
    
    // OS line
    if let Some(os_name) = SysInfoSystem::name() {
        max_len = max_len.max(format!("OS: {}", os_name).len());
    }
    
    // Kernel line
    if let Some(kernel) = SysInfoSystem::kernel_version() {
        max_len = max_len.max(format!("Kernel: {}", kernel).len());
    }
    
    // Uptime line (max estimated)
    max_len = max_len.max("Uptime: 999 Days 99 Hours 99 Minutes".len());
    
    // Packages (check actual)
    if let Ok(output) = Command::new("brew").args(&["list", "--formula"]).output() {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                max_len = max_len.max(format!("Packages: {} (brew)", count).len());
            }
        }
    }
    
    // CPU lines
    max_len = max_len.max(format!("CPU Cores: {}", sys.cpus().len()).len());
    
    if let Some(cpu) = sys.cpus().first() {
        max_len = max_len.max(format!("CPU Brand: {}", cpu.brand()).len());
        max_len = max_len.max(format!("CPU Frequency: {} MHz", cpu.frequency()).len());
    }
    
    max_len = max_len.max("CPU Usage: 100.00%".len());
    
    // Memory lines
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    max_len = max_len.max(format!("Total Memory: {:.2} GB", total_mem).len());
    
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    max_len = max_len.max(format!("Used Memory: {:.2} GB", used_mem).len());
    
    max_len
}

pub fn print_hardware_info() {
    // Setting & Output host name
    let host_name = SysInfoSystem::host_name().unwrap_or_else(|| "Unknown".to_string());
    println!("{}", Cyan.paint(&host_name));

    // Calculate maximum width across ALL system info lines
    let max_len = calculate_max_info_width();

    // Output separator with calculated width
    let separator = "-".repeat(max_len);
    println!("{}", separator);

    // Setting & Output OS name
    if let Some(os_name) = SysInfoSystem::name() {
        println!("{} {}", Green.paint("OS:"), Cyan.paint(os_name));
    }

    // Setting & Output kernel information
    if let Some(kernel) = SysInfoSystem::kernel_version() {
        println!("{} {}", Green.paint("Kernel:"), Cyan.paint(kernel));
    }
}

// System uptime
pub fn system_uptime() {
    // Init the sys
    let sys = System::new();

    // Match the result of uptime retrieval
    match sys.uptime() {
        // Successful uptime retrieval
        Ok(uptime) => {
            // Calculate the time
            let days = uptime.as_secs() / 86400;
            let hours = (uptime.as_secs() % 86400) / 3600;
            let minutes = (uptime.as_secs() % 3600) / 60;

            // Output the Data
            println!(
                "{} {} {} {} {} {} {}",
                Green.paint("Uptime:"),
                Cyan.paint(days.to_string()),
                Green.paint("Days"),
                Cyan.paint(hours.to_string()),
                Green.paint("Hours"),
                Cyan.paint(minutes.to_string()),
                Green.paint("Minutes")
            );
        }
        // Error case for uptime retrieval
        Err(e) => eprintln!("Error getting uptime: {}", e),
    }
}

// Calculate package number
pub fn print_packages() {
    let mut package_managers = Vec::new();

    // macOS: Homebrew
    if let Ok(output) = Command::new("brew")
        .args(&["list", "--formula"])
        .output()
    {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .count();
            if count > 0 {
                package_managers.push(("brew", count));
            }
        }
    }

    // Output information about packages
    if !package_managers.is_empty() {
        let packages_str: Vec<String> = package_managers
            .iter()
            .map(|(name, count)| format!("{} ({})", count, name))
            .collect();

        println!(
            "{}{}",
            Green.paint("Packages: "),
            Cyan.paint(packages_str.join(", "))
        );
    }
}

// CPU
pub fn print_cpu_info() {
    let mut sys = SysInfoSystem::new_all();
    sys.refresh_all();

    // CPU core
    println!(
        "{} {}",
        Green.paint("CPU Cores:"),
        Cyan.paint(sys.cpus().len().to_string())
    );

    // CPU brand
    if let Some(cpu) = sys.cpus().first() {
        println!(
            "{} {}",
            Green.paint("CPU Brand:"),
            Cyan.paint(cpu.brand())
        );

        println!(
            "{} {} MHz",
            Green.paint("CPU Frequency:"),
            Cyan.paint(cpu.frequency().to_string())
        );
    }

    // CPU usage
    println!(
        "{} {:.2}%",
        Green.paint("CPU Usage:"),
        Cyan.paint(sys.global_cpu_usage().to_string())
    );
}

pub fn print_memory_info() {
    let mut sys = SysInfoSystem::new_all();
    sys.refresh_all();
    println!(
        "{} {:.2} GB",
        Green.paint("Total Memory:"),
        Cyan.paint((sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0).to_string())
    );

    println!(
        "{} {:.2} GB",
        Green.paint("Used Memory:"),
        Cyan.paint((sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0).to_string())
    );
}
