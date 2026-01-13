use ansi_term::Color::{Cyan, Green};
use sysinfo::System as SysInfoSystem;
use systemstat::{Platform, System};

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
        println!("{}{}", Green.paint("OS:"), Cyan.paint(os_name));
    }

    // Setting & Output kernel information
    if let Some(kernel) = SysInfoSystem::kernel_version() {
        println!("{}{}", Green.paint("Kernel:"), Cyan.paint(kernel));
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
