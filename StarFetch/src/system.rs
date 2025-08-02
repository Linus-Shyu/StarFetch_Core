use ansi_term::Color::{Green, Cyan};
use sysinfo::System;

// Init the system library
pub fn init_system() -> System {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys
}

pub fn print_hardware_info() {

    // Setting & Output host name
    let host_name = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    println!("{}", Cyan.paint(&host_name));
    
    // Setting & Output "-"
    let separator = "-".repeat(host_name.len());
    println!("{}", separator);
    
    // Setting & Output OS name
    if let Some(os_name) = System::name() {
        println!("{}{}", Green.paint("OS:"), Cyan.paint(os_name));
    }
    
    // Setting & Output kernel information
    if let Some(kernel) = System::kernel_version() {
        println!("{}{}", Green.paint("Kernel:"), Cyan.paint(kernel));
    }
}
