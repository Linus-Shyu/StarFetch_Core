// Import the function you need
mod art;
mod hyperlink;
mod system;

use clap::Parser;

#[derive(Parser)]
#[command(name = "starfetch")]
#[command(about = "A Beauty & fast system information tool", long_about = None)]
struct Args {
    /// Show installed package count (brew, apt, winget, etc.)
    #[arg(short, long, alias = "p")]
    packages: bool,
    /// Show System HardWare information
    #[arg(short = 'c', long, alias = "c")]
    hardware: bool,
}

fn main() {
    let args = Args::parse();

    // -p / --packages: only show package count, then exit
    if args.packages {
        system::print_packages();
        return;
    }

    if args.hardware {
        system::print_cpu_info();
        return;
    }

    // Full interface
    println!("{}", art::adaptive_art());

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

    println!();

    let _sys = system::init_system();
    system::print_hardware_info();
    system::system_uptime();
    system::print_packages();
    system::print_cpu_info();
    system::print_memory_info();
    system::print_swap_info();
    system::print_disk_info();
}
