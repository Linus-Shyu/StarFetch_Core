# StarFetch ⭐

[![Star History Chart](https://api.star-history.com/svg?repos=Linus-Shyu/StarFetch_Core&type=date&legend=top-left)](https://www.star-history.com/#Linus-Shyu/StarFetch_Core&type=date&legend=top-left)

[![Written in Rust](https://img.shields.io/badge/Written%20in-Rust-CE412B?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Mascot](https://img.shields.io/badge/Mascot-Ferris-orange?style=for-the-badge)](https://rustacean.net/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)

A beautiful and fast system information tool written in Rust, inspired by neofetch. StarFetch displays your system information with elegant ASCII art and smart terminal adaptation.

## 💡 Inspiration & Encouragement

StarFetch was born from a deep respect for the legacy of command-line tools. We are incredibly honored to have received these words of encouragement from **Dylan Araps**, the creator of [neofetch](https://github.com/dylanaraps/neofetch):

> "Starfetch looks cool. It looks like a lot of care has gone into it. ... I wish you all the best and I hope you succeed in your goals."
> — **Dylan Araps**

His reminder that "writing software is fun but can also be very draining" and to "look after yourselves" is a core value we carry forward in this project.

---

## ✨ Features

- 🎨 **Adaptive ASCII Art** - Automatically adjusts display based on terminal width.
- 🖥️ **Comprehensive System Info** - Hostname, OS, kernel, uptime, CPU, memory, and packages.
- 🔗 **Smart Hyperlinks** - Clickable developer links with terminal detection.
- 🌈 **Beautiful Colors** - ANSI color support for elegant terminal output.
- ⚡ **Lightning Fast** - Written in Rust for optimal performance.
- 🔧 **Cross-Platform** - Works on macOS, Linux, and Windows.

## 📸 Screenshot

```text
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

### HomeBrew

```bash
brew tap Linus-Shyu/tap
brew install starfetch
```

### Prerequisites

* **Rust** (latest stable version) - [Install Rust](https://www.rust-lang.org/tools/install)
* **Cargo** (comes with Rust)

### Build from Source

```bash
git clone [https://github.com/Linus-Shyu/StarFetch_Core.git](https://github.com/Linus-Shyu/StarFetch_Core.git)
cd StarFetch_Core/StarFetch
cargo build --release

```

### Install Globally

```bash
cargo install --path .

```

## 📦 Dependencies

* `ansi_term` - Terminal colors and styling.
* `sysinfo` - Cross-platform system info.
* `systemstat` - System statistics (uptime, etc.).
* `terminal_size` - Terminal width detection.

## 👥 Authors

* **Linus Shyu** ([@Linus-Shyu](https://github.com/Linus-Shyu))
* **Dylan Su** ([@xs10l3](https://github.com/xs10l3))

## 🙏 Acknowledgments

* **Dylan Araps** - For the original inspiration and kind words.
* **Rust Foundation** - For guidance on trademark compliance. We use **Ferris the Crab** (the unofficial-official mascot) to represent our love for the Rust community. 🦀
* **The Open Source Community** - For the amazing crates that make this project possible.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.

---

⭐ If you find StarFetch useful, please consider giving it a star on GitHub!
