# StarFetch ⭐

[![Star历史图表](https://api.star-history.com/svg?repos=Linus-Shyu/StarFetch_Core&type=date&legend=top-left)](https://www.star-history.com/#Linus-Shyu/StarFetch_Core&type=date&legend=top-left)

[![用Rust编写](https://img.shields.io/badge/Written%20in-Rust-CE412B?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![吉祥物](https://img.shields.io/badge/Mascot-Ferris-orange?style=for-the-badge)](https://rustacean.net/)
[![许可证](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)

<p align="center">
<a href="README.md">English</a>
<a href="README.cn.md">简体中文</a>
</p>

一个用Rust编写的美观且快速的系统信息工具，灵感来自neofetch。StarFetch使用优雅的ASCII艺术和智能的终端自适应来展示您的系统信息。

## 💡 灵感与鼓励

StarFetch 的诞生源于对命令行工具传承的深切尊重。我们非常荣幸能够收到**Dylan Araps**（[neofetch](https://github.com/dylanaraps/neofetch)的作者）的这些鼓励之词：

> "Starfetch很酷。看得出投入了很多心血。…祝你一切顺利，希望你能实现自己的目标。"
> — **Dylan Araps**

他的提醒"编写软件很有趣，但也可能非常耗费精力"以及"照顾好自己"是我们在这个项目中坚持的核心价值观。

---

## ✨ 功能特性

- 🎨 **自适应ASCII艺术** - 根据终端宽度自动调整显示。
- 🖥️ **全面的系统信息** - 主机名、操作系统、内核、运行时间、CPU、内存和软件包。
- 🔗 **智能超链接** - 带有终端检测的可点击开发者链接。
- 🌈 **美丽的颜色** - 支持ANSI颜色以获得优雅的终端输出。
- ⚡ **闪电般快速** - 用Rust编写以获得最佳性能。
- 🔧 **跨平台** - 适用于macOS、Linux和Windows。

## 📸 截图

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

## 🚀 安装

### 环境

- **Rust** (最新稳定版) - [安装Rust](https://www.rust-lang.org/tools/install)
- **Cargo** (Rust附带)

### 从源代码构建

```bash
git clone https://github.com/Linus-Shyu/StarFetch_Core.git
cd StarFetch_Core/StarFetch
cargo build --release

```

### 全局安装

```bash
cargo install --path .

```

## 📦 依赖

- `ansi_term` - 终端颜色和样式。
- `sysinfo` - 跨平台系统信息。
- `systemstat` - 系统统计信息（运行时间等）。
- `terminal_size` - 终端宽度检测。

## 👥 作者

- **Linus Shyu** ([@Linus-Shyu](https://github.com/Linus-Shyu))
- **Dylan Su** ([@xs10l3](https://github.com/xs10l3))

## 🙏 致谢

- **Dylan Araps** - 感谢原始灵感和友善的鼓励。
- **Rust基金会** - 感谢商标合规性指导。我们使用**Ferris the Crab**（非官方但被官方认可的吉祥物）来代表我们对Rust社区的热爱。🦀
- **开源社区** - 感谢那些使这个项目成为可能的优秀的开源库。

## 📄 许可证

本项目采用MIT许可证 - 详见[LICENSE](https://www.bing.com/search?q=LICENSE)文件。

---

⭐ 如果您觉得StarFetch有用，请考虑在GitHub上给它一个星标！
