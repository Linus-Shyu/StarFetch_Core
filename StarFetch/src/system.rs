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
    
    // Brew Packages (check actual)
    if let Ok(output) = Command::new("brew").args(["list", "--formula"]).output() {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                max_len = max_len.max(format!("Packages: {} (brew)", count).len());
            }
        }
    }

    // APT Packages (check actual)
    if let Ok(output) = Command::new("dpkg").args(["-l"]).output() {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| line.starts_with("ii"))
                .count();
            if count > 0 {
                max_len = max_len.max(format!("Packages: {} (apt)", count).len());
            }
        }
    }

    // Winget Packages
    if let Ok(output) = Command::new("winget")
        .args(["list", "--accept-source-agreements"])
        .output() 
    {
        if output.status.success() {
            let full_output = String::from_utf8_lossy(&output.stdout);
                let line_count = full_output.lines()
                .filter(|l| !l.trim().is_empty())
                .count();
            if line_count > 2 {
                let count = line_count - 2;
                max_len = max_len.max(format!("Packages: {} (winget)", count).len());
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
    
    // Swap lines
    let total_swap = sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
    max_len = max_len.max(format!("Total Swap Memory: {:.2} GB", total_swap).len());
    
    let used_swap = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
    max_len = max_len.max(format!("Used Swap Memory: {:.2} GB", used_swap).len());
    
    // Disk lines (estimate maximum)
    max_len = max_len.max("Total Disk: 9999.99 GB".len());
    max_len = max_len.max("Used Disk: 9999.99 GB".len());
    max_len = max_len.max("Available Disk: 9999.99 GB".len());
    
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

// Format and print uptime from seconds (shared helper)
fn print_uptime_seconds(secs: u64) {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
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

// Max uptime to consider valid (~10 years) to reject boot_time/epoch bugs on some platforms
const MAX_UPTIME_SECS: u64 = 10 * 365 * 24 * 3600;

/// On macOS/BSD, try "sysctl -n kern.boottime" and parse secs for fallback.
fn uptime_from_sysctl_kern_boottime() -> Option<u64> {
    let output = Command::new("sysctl")
        .args(["-n", "kern.boottime"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    // e.g. "{ sec = 1234567890, usec = 123456 }"
    let s = String::from_utf8_lossy(&output.stdout);
    let s = s.trim();
    let prefix = "sec = ";
    let start = s.find(prefix).map(|i| i + prefix.len())?;
    let rest = &s[start..];
    let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    let secs_str = &rest[..end];
    let boot_secs: u64 = secs_str.parse().ok()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    let uptime = now.saturating_sub(boot_secs);
    if uptime <= MAX_UPTIME_SECS {
        Some(uptime)
    } else {
        None
    }
}

// System uptime: try systemstat first, then sysctl (macOS), then sysinfo, else N/A.
pub fn system_uptime() {
    let sys = System::new();
    let secs = match sys.uptime() {
        Ok(duration) => {
            let s = duration.as_secs();
            if s <= MAX_UPTIME_SECS {
                Some(s)
            } else {
                None
            }
        }
        Err(_) => None,
    };

    let secs = secs.or_else(uptime_from_sysctl_kern_boottime).or_else(|| {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let boot = SysInfoSystem::boot_time();
        let mut s = now.saturating_sub(boot);
        if s == 0 || s > MAX_UPTIME_SECS {
            s = SysInfoSystem::uptime();
        }
        if s > 0 && s <= MAX_UPTIME_SECS {
            Some(s)
        } else {
            None
        }
    });

    match secs {
        Some(s) => print_uptime_seconds(s),
        None => println!("{} {}", Green.paint("Uptime:"), Cyan.paint("N/A")),
    }
}

// Calculate package number
pub fn print_packages() {
    let mut package_managers = Vec::new();

    // macOS: Homebrew
    if let Ok(output) = Command::new("brew")
        .args(["list", "--formula"])
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

    // Linux: APT (try dpkg first)
    let mut apt_found = false;
    if let Ok(output) = Command::new("dpkg")
        .args(["-l"])
        .output()
    {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| line.starts_with("ii"))
                .count();
            if count > 0 {
                package_managers.push(("apt", count));
                apt_found = true;
            }
        }
    }

    // Fallback to apt list if dpkg fails
    if !apt_found {
        if let Ok(output) = Command::new("apt")
            .args(["list", "--installed"])
            .output()
        {
            if output.status.success() {
                let count = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter(|line| line.contains("/"))  // Filter out header lines
                    .count();
                if count > 0 {
                    package_managers.push(("apt", count));
                }
            }
        }
    }

    // Winget
    if let Ok(output) = Command::new("winget").args(["list", "--accept-source-agreements"]).output() {
        if output.status.success() {
            let full_output = String::from_utf8_lossy(&output.stdout);

            let line_count = full_output.lines().filter(|l| !l.trim().is_empty()).count();

            if line_count > 2 {
                package_managers.push(("winget", line_count - 2));
            }
        }
    }

    // Yum
    if let Ok(output) = Command::new("yum")
        .args(["list", "installed"])
            .output()
    {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| !line.trim().is_empty() && !line.contains("Installed packages"))
                .count();
            if count > 0 {
                package_managers.push(("yum", count));
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

pub fn print_system_help_info() {
    println!(
        "{} {} {} {} {} {} {} {}",
        Green.paint("All command for starfetch \n"),
        Green.paint("-p / --packages: only show package count, then exit \n"),
        Green.paint("-c / --cpu: show the cpu information \n"),
        Green.paint("-t / --time: show time information \n"),
        Green.paint("-k / --kernel: show system information \n"),
        Green.paint("-m / --memory: show memory information \n"),
        Green.paint("-s / --swap: show physical RAM information \n"),
        Green.paint("-d / --disk: show disk information \n"),

    );

}

pub fn print_about() {
    println!(
        "{} {}",
        Green.paint("Github Address:https://github.com/Linus-Shyu/StarFetch_Core \n"),
        Green.paint("Thanks all users! \n"),
    );
}
