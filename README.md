<div align="center">

# ⏱️ Zenith Clock (`zenith-clock`)

**A lightweight, glassmorphic, always-on-top desktop clock overlay for Windows written in 100% pure Rust.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Built%20With-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-0078D6.svg)](https://microsoft.com/windows)
[![Release](https://img.shields.io/github/v/release/themrsami/zenith-clock?color=green&include_prereleases)](https://github.com/themrsami/zenith-clock/releases)

<br />

*Floating effortlessly at the top of your screen like a sleek Dynamic Island capsule.*

</div>

---

## 🌟 Highlights

- ⚡ **Pure Native Rust**: Zero Electron, zero WebViews, zero Python runtimes. Single standalone binary under **400 KB**.
- 🚀 **Ultra-Low Resource Footprint**: Uses **~5 MB to 8 MB RAM** and **0.0% CPU** while idling.
- 🪟 **Glassmorphic Layered Window**: Built with native Win32 `UpdateLayeredWindow` & GDI+ for anti-aliased rounded corners, frosted acrylic effect, and soft gaussian drop shadow.
- 🧲 **Smooth Drag & Auto-Snap**: Click and drag anywhere on the widget to move. Snaps automatically to top-center when released near the edge.
- 🎨 **7 Curated Visual Themes**: Switch palettes on the fly via the right-click menu.
- ⏱️ **Flexible Time & Date Display**: 12-Hour (with AM/PM pill) or 24-Hour mode, optional seconds counter, and calendar date.
- 🔒 **Lock Position & Click-Through Mode**: Lock it in place to prevent accidental drags, or enable ghost mode so mouse clicks pass straight through to background apps.
- 🔔 **Windows System Tray Integration**: Minimize to tray, toggle visibility with a single click, or access controls anytime.
- 💾 **Persistent Configuration**: Remembers your screen coordinates $(X, Y)$, selected theme, scale, and options in `config.json`.

---

## 🎨 Themes Catalog

| Theme | Description | Accent |
| :--- | :--- | :--- |
| 🌌 **Deep Glass (Dark)** | Frosted dark slate acrylic with cyan highlights *(Default)* | Sky Cyan |
| ✨ **Midnight Gold** | Obsidian glass with luxury champagne gold typography | Amber Gold |
| 🔮 **Cyberpunk Neon** | Synthwave aesthetic with electric neon cyan and hot pink | Neon Pink |
| 🍃 **Emerald Mist** | Dark forest glass with fresh mint green numerals | Emerald Mint |
| ☁️ **Pure Frost (Light)** | Crisp high-contrast frosted light glass for dark wallpapers | Royal Blue |
| 🖤 **OLED Stealth** | Pure jet-black borderless minimal look | Platinum White |
| 🌅 **Sunset Glow** | Deep plum acrylic with warm coral and rose tones | Coral Rose |

---

## 🕹️ Controls & Context Menu

Right-click anywhere on the clock to open the control menu:

| Option | Description |
| :--- | :--- |
| 🎨 **Themes** | Choose between all 7 themes. |
| 📐 **Size** | Select **Compact**, **Normal**, or **Large** scaling. |
| 👁️ **Opacity** | Set opacity to **100% Solid**, **92% Glass**, **75% Translucent**, or **50% Ghost**. |
| 🕒 **24-Hour Format** | Toggle between 12-hour AM/PM and 24-hour military time. |
| ⏱️ **Show Seconds** | Toggle display of real-time seconds. |
| 📅 **Show Date** | Toggle display of weekday and date (`Sun, Oct 04`). |
| 🔒 **Lock Position** | Lock window in place to disable mouse dragging. |
| 🧲 **Snap to Top Center** | Reposition the clock neatly to the top center of your monitor. |
| 📌 **Always on Top** | Pin the overlay above all standard and full-screen windows. |
| 👻 **Click-Through Mode** | Pass mouse clicks through to whatever application is behind it. |
| ❌ **Exit Clock** | Closes the application and removes the tray icon. |

> **Tip:** If you enable **Click-Through Mode**, you can disable it at any time by right-clicking the clock icon in your Windows Taskbar System Tray and selecting **🔓 Disable Click-Through**.

---

## 📥 Installation & Running

### Option 1: Download Pre-built Binary (Recommended)
1. Download the latest `zenith-clock-windows-x64.zip` from [GitHub Releases](https://github.com/themrsami/zenith-clock/releases).
2. Extract the folder anywhere on your computer.
3. Double-click `zenith-clock.exe` to run. (It launches silently without any command prompt window!).

### Option 2: Run with Windows Startup
To make Zenith Clock launch automatically when your PC boots:
- Double-click `install_autostart.bat`.
- To remove it from startup, double-click `uninstall_autostart.bat`.

---

## 🛠️ Building from Source

### Prerequisites
- [Rust toolchain](https://www.rust-lang.org/tools/install) (1.70+ recommended).
- Windows 10 or Windows 11.

### Build Steps

```powershell
# 1. Clone the repository
git clone https://github.com/themrsami/zenith-clock.git
cd zenith-clock

# 2. Run in debug mode
cargo run

# 3. Build optimized release executable (~390 KB)
cargo build --release
```

The compiled binary will be located at `target/release/zenith-clock.exe`.

---

## 📂 Project Structure

```
zenith-clock/
├── .github/
│   └── workflows/
│       └── release.yml        # Automated GitHub Actions release builder
├── src/
│   ├── config.rs              # Configuration loader & JSON persistence
│   ├── menu.rs                # Win32 context menu & tray menu handlers
│   ├── renderer.rs            # GDI+ layered window alpha rendering engine
│   ├── theme.rs               # Color palettes & scale metrics definitions
│   └── main.rs                # Windows entry point, message pump & DPI setup
├── .gitignore
├── Cargo.toml
├── install_autostart.bat       # Startup shortcut installer
├── uninstall_autostart.bat     # Startup shortcut uninstaller
├── LICENSE                    # MIT License
└── README.md
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute!
