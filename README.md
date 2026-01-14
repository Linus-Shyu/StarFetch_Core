# StarFetch

A beautiful system information tool written in Rust, inspired by neofetch. StarFetch displays your system information with an elegant ASCII art banner.

## Features

- 🎨 **Adaptive ASCII Art**: Automatically selects between full and compact ASCII art based on terminal width
- 🖥️ **System Information**: Displays hostname, OS, kernel version, and uptime
- 🔗 **Clickable Links**: Developer name with clickable hyperlink support
- 🌈 **Colorful Output**: Beautiful colored terminal output using ANSI colors
- ⚡ **Fast & Lightweight**: Written in Rust for optimal performance

## Screenshot

```
_____/\\\\\\\\\\\_______________________________________________/\\\\\\\\\\\\\\\_____________________________________________/\\\_________
 ___/\\\/////////\\\____________________________________________\/\\\///////////_____________________________________________\/\\\_________
  __\//\\\______\///______/\\\___________________________________\/\\\________________________________/\\\____________________\/\\\_________
   ___\////\\\__________/\\\\\\\\\\\__/\\\\\\\\\_____/\\/\\\\\\\__\/\\\\\\\\\\\_________/\\\\\\\\___/\\\\\\\\\\\_____/\\\\\\\\_\/\\\_________
    ______\////\\\______\////\\\////__\////////\\\___\/\\\/////\\\_\/\\\///////________/\\\/////\\\_\////\\\////____/\\\//////__\/\\\\\\\\\\__
     _________\////\\\______\/\\\________/\\\\\\\\\\__\/\\\___\///__\/\\\______________/\\\\\\\\\\\_____\/\\\_______/\\\_________\/\\\/////\\\_
      __/\\\______\//\\\_____\/\\\_/\\___/\\\/////\\\__\/\\\_________\/\\\_____________\//\\///////______\/\\\_/\\__\//\\\________\/\\\___\/\\\_
       _\///\\\\\\\\\\\/______\//\\\\\___\//\\\\\\\\/\\_\/\\\_________\/\\\______________\//\\\\\\\\\\____\//\\\\\____\///\\\\\\\\_\/\\\___\/\\\_
        ___\///////////_________\/////_____\////////\//__\///__________\///________________\//////////______\/////_______\////////__\///____\///__

Developed by Linus Shyu

hostname
--------
OS: macOS
Kernel: 25.2.0
Uptime: 2 Days 5 Hours 30 Minutes
```

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/Linus-Shyu/StarFetch_Core.git
cd StarFetch_Core/StarFetch
```

2. Build the project:
```bash
cargo build --release
```

3. Run the executable:
```bash
cargo run --release
```

Or install it globally:
```bash
cargo install --path .
```

## Usage

Simply run:
```bash
starfetch
```

Or if you built it locally:
```bash
cargo run
```

## Project Structure

```
StarFetch/
├── src/
│   ├── main.rs      # Main entry point
│   ├── art.rs       # ASCII art generation and terminal width detection
│   ├── system.rs    # System information retrieval
│   └── hyperlink.rs # Hyperlink and text styling utilities
├── Cargo.toml       # Project dependencies and metadata
└── README.md        # This file
```

## Dependencies

- `ansi_term` - Terminal colors and styling
- `sysinfo` - System information retrieval
- `systemstat` - System statistics (uptime, etc.)
- `terminal_size` - Terminal width detection

## Features in Detail

### Adaptive ASCII Art

StarFetch automatically detects your terminal width:
- **Width ≥ 120 characters**: Displays full "STARFETCH" ASCII art
- **Width < 120 characters**: Displays compact box art
- **Unable to detect**: Falls back to full ASCII art

### System Information

Displays:
- **Hostname**: Your system's hostname
- **OS**: Operating system name
- **Kernel**: Kernel version
- **Uptime**: System uptime in days, hours, and minutes

### Clickable Links

The developer name is displayed as a clickable hyperlink (supported in modern terminals like iTerm2, Windows Terminal, etc.)

## Development

### Format Code

```bash
cargo fmt
```

### Check for Issues

```bash
cargo check
```

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

See [LICENSE](LICENSE) file for details.

## Author

**Linus Shyu**

- GitHub: [@Linus-Shyu](https://github.com/Linus-Shyu)

## Acknowledgments

- Inspired by [neofetch](https://github.com/dylanaraps/neofetch)
- Built with ❤️ using Rust
