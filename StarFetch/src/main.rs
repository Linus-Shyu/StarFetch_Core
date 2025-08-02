use ansi_term::Style;
use sysinfo::System;

// Exchange url link function
fn hyperlink(text: &str, url: &str) -> String {
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
}

// Logo design
fn ascii_function() -> &'static str {
    r#"
_____/\\\\\\\\\\\_______________________________________________/\\\\\\\\\\\\\\\_____________________________________________/\\\_________
 ___/\\\/////////\\\____________________________________________\/\\\///////////_____________________________________________\/\\\_________
  __\//\\\______\///______/\\\___________________________________\/\\\________________________________/\\\____________________\/\\\_________
   ___\////\\\__________/\\\\\\\\\\\__/\\\\\\\\\_____/\\/\\\\\\\__\/\\\\\\\\\\\_________/\\\\\\\\___/\\\\\\\\\\\_____/\\\\\\\\_\/\\\_________
    ______\////\\\______\////\\\////__\////////\\\___\/\\\/////\\\_\/\\\///////________/\\\/////\\\_\////\\\////____/\\\//////__\/\\\\\\\\\\__
     _________\////\\\______\/\\\________/\\\\\\\\\\__\/\\\___\///__\/\\\______________/\\\\\\\\\\\_____\/\\\_______/\\\_________\/\\\/////\\\_
      __/\\\______\//\\\_____\/\\\_/\\___/\\\/////\\\__\/\\\_________\/\\\_____________\//\\///////______\/\\\_/\\__\//\\\________\/\\\___\/\\\_
       _\///\\\\\\\\\\\/______\//\\\\\___\//\\\\\\\\/\\_\/\\\_________\/\\\______________\//\\\\\\\\\\____\//\\\\\____\///\\\\\\\\_\/\\\___\/\\\_
        ___\///////////_________\/////_____\////////\//__\///__________\///________________\//////////______\/////_______\////////__\///____\///__

    "#
}

// Colored logo
fn colored_function() -> String {
    Style::new()
        .fg(ansi_term::Color::Cyan)
        .paint(ascii_function())
        .to_string()
}

// Colored developer's name
fn linus_shyu() -> String {
    Style::new()
        .underline()
        .fg(ansi_term::Color::Green)
        .paint("Linus Shyu")
        .to_string()
}

// System information
fn hard_ware() {
    println!("System:{}", System::name().unwrap_or_default());
    println!("Kernel:{}", System::kernel_version().unwrap_or_default());
    println!("Hostname:{}", System::host_name().unwrap_or_default());
}

// init sysinfo
fn init_system() {
    let mut sys = System::new_all();
    sys.refresh_all();
}

fn linked_text() -> String {
    hyperlink(&linus_shyu(), "https://github.com/Linus-Shyu")
}

fn main() {
    // Output colored logo
    println!("{}", colored_function());

    // Developer information
    print!("Developed by ");

    // input url & text to the hyperlink function
    linked_text();

    // Output developer name
    println!("{}", linked_text());

    // init sysinfo
    init_system();

    // Output system information
    hard_ware();

    // Hardware information
}
