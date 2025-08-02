// Import the function you need
mod art;
mod hyperlink;
mod system;

fn main() {
    // Output the ascii painting
    println!("{}", art::colored_art());
    
    // Output link & text
    print!("Developed by ");
    println!("{}", hyperlink::hyperlink(
        &hyperlink::styled_developer_name(),  
        "https://github.com/Linus-Shyu"
    ));
    
    // Opimization the UI
    println!();
    
    // Output system information
    let _sys = system::init_system(); // Init the library
    system::print_hardware_info();
}
