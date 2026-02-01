// Import the function you need
mod art;
mod hyperlink;
mod system;

use clap::Parser;

#[derive(Parser)]
#[command(
    disable_help_flag = true,
    name = "starfetch",
    about = "A Beauty & fast system information tool",
    long_about = None,
    version
)]
struct Args {
    /// Show installed package count (brew, apt, winget, etc.)
    #[arg(short, long, alias = "p")]
    packages: bool,
    /// Show cpu information
    #[arg(short = 'c', long, alias = "c")]
    cpu: bool,
    /// Show time information
    #[arg(short = 't', long, alias = "t")]
    time: bool,
    /// Hardware information
    #[arg(short = 'k', long, alias = "k")]
    hardware: bool,
    /// memory information
    #[arg(short = 'm', long, alias = "m")]
    memory: bool,
    /// swap information
    #[arg(short = 's', long, alias = "s")]
    swap: bool,
    /// disk information
    #[arg(short = 'd', long, alias = "d")]
    disk: bool,
    #[arg(short = 'h', long, alias = "h")]
    help: bool,
    #[arg(short = 'a', long, alias = "a")]
    about: bool,
}

fn main() {
    let args = Args::parse();

    // -p / --packages: only show package count, then exit
    if args.packages {
        system::print_packages();
        return;
    }

    // -c / --cpu: show the cpu information
    if args.cpu {
        system::print_cpu_info();
        return;
    }

    // -t / --time: show time information
    if args.time {
        system::system_uptime();
        return ;
    }
    
    // -k / --kernel: show system information
    if args.hardware {
        system::print_hardware_info();
        return ;
    }

    // -m / --memory: show memory information
    if args.memory {
        system::print_memory_info();
        return ;
    }

    // -s / --swap: show physical RAM information
    if args.swap {
        system::print_swap_info();
        return ;
    }

    // -d / --disk:show disk information
    if args.disk {
        system::print_disk_info();
        return ;
    }

    // -h / --help
    if args.help {
        system::print_system_help_info();
        return ;

    }
    // -a / --about
    if args.about {
        system::print_about();
        return ;
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

// Just wanna test homebrew.
