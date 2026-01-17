use ansi_term::Color::{Cyan, Green};
use sysinfo::System as SysInfoSystem;
use sysinfo::Disks;
use systemstat::{Platform, System};
use std::process::Command;

// Init the system library
pub fn init_system() -> SysInfoSystem {
    let mut sys = SysInfoSystem::new_all();
    sys.refresh_all();
    sys
}

pub fn print_hardware_info() {
    // Setting & Output host name
    let host_name = SysInfoSystem::host_name().unwrap_or_else(|| "Unknown".to_string());
    println!("{}", Cyan.paint(&host_name));

    // Setting & Output "-"
    let separator = "-".repeat(host_name.len());
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

pub fn print_swap_info() {
    let mut sys = SysInfoSystem::new_all();
    sys.refresh_all();
    println!("{} {:.2} GB",
        Green.paint("Total Swap Memory:"),
        Cyan.paint((sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0).to_string())
    );

    println!("{} {:.2} GB",
        Green.paint("Used Swap Memory:"),
        Cyan.paint((sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0).to_string())
    );
}
pub fn print_disk_info() {
    let disks = Disks::new_with_refreshed_list();

    let mut total_space = 0u64;
    let mut available_space = 0u64;
    let mut seen_devices = std::collections::HashSet::new();

    for disk in disks.list() {
        let mount_point = disk.mount_point().to_string_lossy();

        // macOS: 跳过 /System/Volumes/* 挂载点，只保留根目录
        if mount_point.starts_with("/System/Volumes/") {
            continue;
        }

        let fs = disk.file_system().to_string_lossy();
        if fs.contains("tmpfs") ||
            fs.contains("devfs") ||
            fs.contains("sysfs") ||
            mount_point.starts_with("/dev") ||
            mount_point.starts_with("/sys") ||
            mount_point.starts_with("/proc") {
            continue;
        }

        let device_name = disk.name().to_string_lossy().to_string();
        if !seen_devices.insert(device_name) {
            continue;
        }

        total_space += disk.total_space();
        available_space += disk.available_space();
    }

    let used_space = total_space - available_space;

    println!(
        "{} {:.2} GB",
        Green.paint("Total Disk:"),
        Cyan.paint((total_space as f64 / 1_073_741_824.0).to_string())
    );

    println!(
        "{} {:.2} GB",
        Green.paint("Used Disk:"),
        Cyan.paint((used_space as f64 / 1_073_741_824.0).to_string())
    );

    println!(
        "{} {:.2} GB",
        Green.paint("Available Disk:"),
        Cyan.paint((available_space as f64 / 1_073_741_824.0).to_string())
    );
}
