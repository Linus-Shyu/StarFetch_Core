# StarFetch ⭐

A beautiful and fast system information tool written in Rust, inspired by neofetch. StarFetch displays your system information with elegant ASCII art and smart terminal adaptation.

![Rust](https://img.shields.io/badge/rust-stable-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

- 🎨 **Adaptive ASCII Art** - Automatically adjusts display based on terminal width
- 🖥️ **Comprehensive System Info** - Shows hostname, OS, kernel, uptime, CPU, memory, and packages
- 🔗 **Smart Hyperlinks** - Clickable developer links with terminal detection
- 🌈 **Beautiful Colors** - ANSI color support for elegant terminal output
- ⚡ **Lightning Fast** - Written in Rust for optimal performance
- 🔧 **Cross-Platform** - Works on macOS, Linux, and Windows

## 📸 Screenshot

```
    ╔════════════════════════════════╗
    ║   ★  STARFETCH  ★            ║
    ╚════════════════════════════════╝
    
Developed by Linus Shyu

hostname
--------
OS: macOS
Kernel: 25.2.0
Uptime: 6 Days 14 Hours 32 Minutes
Packages: 30 (brew)
CPU Cores: 10
CPU Brand: Apple M5
CPU Frequency: 4608 MHz
CPU Usage: 10.24%
Total Memory: 16 GB
Used Memory: 10.79 GB
```

## 🚀 Installation

### Prerequisites

- **Rust** (latest stable version) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo** (comes with Rust)

### Build from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Linus-Shyu/StarFetch_Core.git
   cd StarFetch_Core/StarFetch
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run it:**
   ```bash
   ./target/release/starfetch
   ```

### Install Globally

Install StarFetch globally using Cargo:

```bash
cd StarFetch_Core/StarFetch
cargo install --path .
```

After installation, you can run `starfetch` from anywhere in your terminal.

## 📖 Usage

Simply run:

```bash
starfetch
```

Or if running from source:

```bash
cargo run
```

Or from release build:

```bash
cargo run --release
```

## 🎯 What It Shows

StarFetch displays the following system information:

- **Hostname** - Your system's hostname
- **OS** - Operating system name (macOS, Linux, Windows, etc.)
- **Kernel** - Kernel version
- **Uptime** - System uptime in days, hours, and minutes
- **Packages** - Installed packages count (Homebrew on macOS)
- **CPU** - CPU cores, brand, frequency, and usage
- **Memory** - Total and used memory in GB

## 🛠️ Features in Detail

### Adaptive ASCII Art

StarFetch intelligently adapts to your terminal width:
- **Wide terminals (≥147 columns)**: Displays full "STARFETCH" ASCII art
- **Narrow terminals (<147 columns)**: Displays compact box-style art
- **Unknown width**: Falls back to full ASCII art

### Smart Hyperlinks

The developer name is displayed as a clickable hyperlink in modern terminals:
- ✅ iTerm2
- ✅ Apple Terminal
- ✅ Windows Terminal
- ✅ VSCode Integrated Terminal
- ✅ xterm-compatible terminals

### Colorful Output

Beautiful colored output using ANSI escape codes for better readability and aesthetics.

## 📁 Project Structure

```
StarFetch_Core/
├── StarFetch/
│   ├── src/
│   │   ├── main.rs       # Main entry point
│   │   ├── art.rs        # ASCII art and terminal width detection
│   │   ├── system.rs     # System information retrieval
│   │   ├── hyperlink.rs  # Hyperlink and terminal detection
│   │   └── lib.rs        # Library module exports
│   ├── Cargo.toml        # Project dependencies and metadata
│   └── README.md         # Documentation
├── LICENSE
└── README.md            # This file
```

## 📦 Dependencies

- **[ansi_term](https://crates.io/crates/ansi_term)** - Terminal colors and styling
- **[sysinfo](https://crates.io/crates/sysinfo)** - Cross-platform system information retrieval
- **[systemstat](https://crates.io/crates/systemstat)** - System statistics (uptime, etc.)
- **[terminal_size](https://crates.io/crates/terminal_size)** - Terminal width detection

## 🔧 Development

### Prerequisites

Make sure you have Rust installed:

```bash
rustc --version
cargo --version
```

### Common Commands

**Format code:**
```bash
cargo fmt
```

**Check for errors:**
```bash
cargo check
```

**Build:**
```bash
cargo build
```

**Run:**
```bash
cargo run
```

**Run tests:**
```bash
cargo test
```

**Build for release:**
```bash
cargo build --release
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please make sure your code follows Rust conventions and passes `cargo fmt` and `cargo clippy`.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Authors

- **Linus Shyu**
  - GitHub: [@Linus-Shyu](https://github.com/Linus-Shyu)

- **Dylan Su**
  - GitHub: [@xs10l3](https://github.com/xs10l3)

## 🙏 Acknowledgments

- Inspired by [neofetch](https://github.com/dylanaraps/neofetch) - A command-line system information tool
- Built with ❤️ using Rust

## 📝 Version

Current version: **0.1.2**

---

⭐ If you find StarFetch useful, please consider giving it a star on GitHub!