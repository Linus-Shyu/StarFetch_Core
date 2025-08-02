use ansi_term::Style;
use ansi_term::Color::Green;  

pub fn hyperlink(text: &str, url: &str) -> String {
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
}

pub fn styled_developer_name() -> String {
    Style::new()
        .underline()
        .fg(Green)
        .paint("Linus Shyu")
        .to_string()
}
