// Check the Terminal type
fn detect_terminal() -> TerminalType {
    // Terminal variable
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        match term_program.as_str() {
            "iTerm.app" => return TerminalType::ITerm2,
            "Apple_Terminal" => return TerminalType::TerminalApp,
            "vscode" => return TerminalType::VSCode,
            _ => {}
        }
    }

    if let Ok(wt_session) = std::env::var("WT_SESSION") {
        if !wt_session.is_empty() {
            return TerminalType::WindowsTerminal;
        }
    }

    // Check Terminal
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("xterm") || term.contains("screen") {
            return TerminalType::XTerm;
        }
    }
    
    TerminalType::Unknown
}

enum TerminalType {
    ITerm2,
    TerminalApp,
    WindowsTerminal,
    VSCode,
    XTerm,
    Unknown,
}

// Passing url & text with smart terminal detection
pub fn hyperlink(text: &str, url: &str) -> String {
    let green_code = "\x1b[32m";
    let underline_code = "\x1b[4m";
    let reset_code = "\x1b[0m";
    
    let terminal = detect_terminal();
    
    match terminal {
        TerminalType::ITerm2 | TerminalType::TerminalApp | 
        TerminalType::WindowsTerminal | TerminalType::VSCode => {
            // OSC
            format!(
                "\x1b]8;;{}\x07{}{}{}{}\x1b]8;;\x07",
                url, green_code, underline_code, text, reset_code
            )
        }
        TerminalType::XTerm => {
            // xterm
            format!(
                "\x1b]8;;{}\x1b\\{}{}{}{}\x1b]8;;\x1b\\",
                url, green_code, underline_code, text, reset_code
            )
        }
        TerminalType::Unknown => {
            // Unknown
            format!(
                "\x1b]8;;{}\x07{}{}{}{}\x1b]8;;\x07",
                url, green_code, underline_code, text, reset_code
            )
        }
    }
}

// Exchange data to string text and rendering
pub fn styled_developer_name() -> String {
    "Linus Shyu".to_string()
}

pub fn styled_developer_name_dylan() -> String {
    "Dylan Su".to_string()
}
