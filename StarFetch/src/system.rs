use ansi_term::Color::{Green, Cyan};
use sysinfo::System;

pub fn init_system() -> System {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys
}

pub fn print_hardware_info() {
    let host_name = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    println!("{}", Cyan.paint(&host_name));
    
    let separator = "-".repeat(host_name.len());
    println!("{}", separator);
    
    if let Some(os_name) = System::name() {
        println!("{}{}", Green.paint("OS:"), Cyan.paint(os_name));
    }
    
    if let Some(kernel) = System::kernel_version() {
        println!("{}{}", Green.paint("Kernel:"), Cyan.paint(kernel));
    }
}
