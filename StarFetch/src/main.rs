use sysinfo::{System};
use ansi_term::{Style};

// Exchange url link function
fn hyperlink(text: &str, url: &str) -> String
{
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
}


fn main()
{
    // Logo design
    let ascii_art = r#"
_____/\\\\\\\\\\\_______________________________________________/\\\\\\\\\\\\\\\_____________________________________________/\\\_________        
 ___/\\\/////////\\\____________________________________________\/\\\///////////_____________________________________________\/\\\_________       
  __\//\\\______\///______/\\\___________________________________\/\\\________________________________/\\\____________________\/\\\_________      
   ___\////\\\__________/\\\\\\\\\\\__/\\\\\\\\\_____/\\/\\\\\\\__\/\\\\\\\\\\\_________/\\\\\\\\___/\\\\\\\\\\\_____/\\\\\\\\_\/\\\_________     
    ______\////\\\______\////\\\////__\////////\\\___\/\\\/////\\\_\/\\\///////________/\\\/////\\\_\////\\\////____/\\\//////__\/\\\\\\\\\\__    
     _________\////\\\______\/\\\________/\\\\\\\\\\__\/\\\___\///__\/\\\______________/\\\\\\\\\\\_____\/\\\_______/\\\_________\/\\\/////\\\_   
      __/\\\______\//\\\_____\/\\\_/\\___/\\\/////\\\__\/\\\_________\/\\\_____________\//\\///////______\/\\\_/\\__\//\\\________\/\\\___\/\\\_  
       _\///\\\\\\\\\\\/______\//\\\\\___\//\\\\\\\\/\\_\/\\\_________\/\\\______________\//\\\\\\\\\\____\//\\\\\____\///\\\\\\\\_\/\\\___\/\\\_ 
        ___\///////////_________\/////_____\////////\//__\///__________\///________________\//////////______\/////_______\////////__\///____\///__

    "#;

    // Colored logo
    let _colored_art = Style::new()
        .fg(ansi_term::Color::Cyan)
        .paint(ascii_art);

    // Output colored logo
    println!("{}",_colored_art);

    // Developer information
    print!("Developed by ");

    // Colored developer's name
    let linus_shyu = Style::new()
        .underline()
        .fg(ansi_term::Color::Green)
        .paint("Linus Shyu");

    // input url & text to the hyperlink function
    let linked_text = hyperlink(&linus_shyu.to_string(), "https://github.com/Linus-Shyu");

    //Output developer name
    println!("{}", linked_text);

    // init sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();

    //System information
    println!("System:{}",System::name().unwrap_or_default());
    println!("Kernel:{}",System::kernel_version().unwrap_or_default());
    println!("Hostname:{}",System::host_name().unwrap_or_default());

    //Hardware information
    
}

