// Import the function you need
mod art;
mod hyperlink;
mod system;

fn main() {
    // Output the ascii painting
    println!("{}", art::adaptive_art());

    // Output link & text
    print!("Developed by ");
    print!(
        "{}",
        hyperlink::hyperlink(
            &hyperlink::styled_developer_name(),
            "https://github.com/Linus-Shyu"
        )
    );
    print!(" and ");
    print!(
        "{}",
        hyperlink::hyperlink(
            &hyperlink::styled_developer_name_dylan(),
            "https://github.com/xs10l3"
        )
    );

    // Optimization the UI
    println!();

    // Output system information
    let _sys = system::init_system(); // Init the library
    system::print_hardware_info();
    system::system_uptime();
    system::print_packages();
    system::print_cpu_info();
    system::print_memory_info();
    system::print_swap_info();
    system::print_disk_info();
}
