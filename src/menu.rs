use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::Foundation::HWND;
use crate::config::Config;
use crate::theme::THEMES;

pub const CMD_THEME_START: usize = 101;
pub const CMD_SCALE_START: usize = 201;
pub const CMD_OPACITY_START: usize = 301;

pub const CMD_TOGGLE_24H: usize = 401;
pub const CMD_TOGGLE_SECONDS: usize = 402;
pub const CMD_TOGGLE_DATE: usize = 403;

pub const CMD_TOGGLE_LOCK: usize = 501;
pub const CMD_SNAP_CENTER: usize = 502;
pub const CMD_TOGGLE_TOPMOST: usize = 503;
pub const CMD_ENABLE_CLICKTHROUGH: usize = 504;
pub const CMD_DISABLE_CLICKTHROUGH: usize = 505;

pub const CMD_TOGGLE_VISIBILITY: usize = 601;
pub const CMD_EXIT: usize = 999;

fn to_wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub unsafe fn show_context_menu(hwnd: HWND, x: i32, y: i32, config: &Config) {
    let menu = CreatePopupMenu();

    // 1. Themes Submenu
    let theme_menu = CreatePopupMenu();
    for (i, t) in THEMES.iter().enumerate() {
        let cmd = CMD_THEME_START + i;
        let mut flags = MF_STRING;
        if t.key == config.theme {
            flags |= MF_CHECKED;
        }
        let w_name = to_wstr(t.name);
        AppendMenuW(theme_menu, flags, cmd, w_name.as_ptr());
    }
    let themes_title = to_wstr("🎨 Themes");
    AppendMenuW(menu, MF_POPUP, theme_menu as usize, themes_title.as_ptr());

    // 2. Size Submenu
    let size_menu = CreatePopupMenu();
    let sizes = [
        (CMD_SCALE_START, "Compact (Small)", "compact"),
        (CMD_SCALE_START + 1, "Normal (Standard)", "normal"),
        (CMD_SCALE_START + 2, "Large (Prominent)", "large"),
    ];
    for (cmd, label, key) in sizes {
        let mut flags = MF_STRING;
        if config.size_scale == key {
            flags |= MF_CHECKED;
        }
        let w_label = to_wstr(label);
        AppendMenuW(size_menu, flags, cmd, w_label.as_ptr());
    }
    let size_title = to_wstr("📐 Size");
    AppendMenuW(menu, MF_POPUP, size_menu as usize, size_title.as_ptr());

    // 3. Opacity Submenu
    let opacity_menu = CreatePopupMenu();
    let opacities = [
        (CMD_OPACITY_START, "100% Solid", 1.0f32),
        (CMD_OPACITY_START + 1, "92% Glass (Recommended)", 0.92f32),
        (CMD_OPACITY_START + 2, "75% Translucent", 0.75f32),
        (CMD_OPACITY_START + 3, "50% Ghost", 0.50f32),
    ];
    for (cmd, label, val) in opacities {
        let mut flags = MF_STRING;
        if (config.opacity - val).abs() < 0.05 {
            flags |= MF_CHECKED;
        }
        let w_label = to_wstr(label);
        AppendMenuW(opacity_menu, flags, cmd, w_label.as_ptr());
    }
    let op_title = to_wstr("👁️ Opacity");
    AppendMenuW(menu, MF_POPUP, opacity_menu as usize, op_title.as_ptr());

    AppendMenuW(menu, MF_SEPARATOR, 0, std::ptr::null());

    // Time Format
    let mut flags_24h = MF_STRING;
    if config.time_format_24h { flags_24h |= MF_CHECKED; }
    let w_24h = to_wstr("24-Hour Format");
    AppendMenuW(menu, flags_24h, CMD_TOGGLE_24H, w_24h.as_ptr());

    // Show Seconds
    let mut flags_sec = MF_STRING;
    if config.show_seconds { flags_sec |= MF_CHECKED; }
    let w_sec = to_wstr("Show Seconds");
    AppendMenuW(menu, flags_sec, CMD_TOGGLE_SECONDS, w_sec.as_ptr());

    // Show Date
    let mut flags_date = MF_STRING;
    if config.show_date { flags_date |= MF_CHECKED; }
    let w_date = to_wstr("Show Date");
    AppendMenuW(menu, flags_date, CMD_TOGGLE_DATE, w_date.as_ptr());

    AppendMenuW(menu, MF_SEPARATOR, 0, std::ptr::null());

    // Lock Position
    let mut flags_lock = MF_STRING;
    if config.locked { flags_lock |= MF_CHECKED; }
    let w_lock = to_wstr("🔒 Lock Position");
    AppendMenuW(menu, flags_lock, CMD_TOGGLE_LOCK, w_lock.as_ptr());

    // Snap Center
    let w_snap = to_wstr("🧲 Snap to Top Center");
    AppendMenuW(menu, MF_STRING, CMD_SNAP_CENTER, w_snap.as_ptr());

    // Always on Top
    let mut flags_top = MF_STRING;
    if config.always_on_top { flags_top |= MF_CHECKED; }
    let w_top = to_wstr("📌 Always on Top");
    AppendMenuW(menu, flags_top, CMD_TOGGLE_TOPMOST, w_top.as_ptr());

    // Click Through Mode
    let w_click = to_wstr("👻 Ghost / Click-Through Mode");
    AppendMenuW(menu, MF_STRING, CMD_ENABLE_CLICKTHROUGH, w_click.as_ptr());

    AppendMenuW(menu, MF_SEPARATOR, 0, std::ptr::null());

    // Exit
    let w_exit = to_wstr("❌ Exit Clock");
    AppendMenuW(menu, MF_STRING, CMD_EXIT, w_exit.as_ptr());

    // Set foreground window before TrackPopupMenu to dismiss properly
    SetForegroundWindow(hwnd);
    TrackPopupMenuEx(menu, TPM_RIGHTBUTTON, x, y, hwnd, std::ptr::null());
    DestroyMenu(menu);
}

pub unsafe fn show_tray_menu(hwnd: HWND, x: i32, y: i32, is_visible: bool, is_clickthrough: bool) {
    let menu = CreatePopupMenu();

    let toggle_text = if is_visible { "👁️ Hide Clock" } else { "👁️ Show Clock" };
    let w_toggle = to_wstr(toggle_text);
    AppendMenuW(menu, MF_STRING, CMD_TOGGLE_VISIBILITY, w_toggle.as_ptr());

    let w_snap = to_wstr("🧲 Snap to Top Center");
    AppendMenuW(menu, MF_STRING, CMD_SNAP_CENTER, w_snap.as_ptr());

    if is_clickthrough {
        let w_disable = to_wstr("🔓 Disable Click-Through");
        AppendMenuW(menu, MF_STRING, CMD_DISABLE_CLICKTHROUGH, w_disable.as_ptr());
    }

    AppendMenuW(menu, MF_SEPARATOR, 0, std::ptr::null());

    let w_exit = to_wstr("❌ Exit Clock");
    AppendMenuW(menu, MF_STRING, CMD_EXIT, w_exit.as_ptr());

    SetForegroundWindow(hwnd);
    TrackPopupMenuEx(menu, TPM_RIGHTBUTTON, x, y, hwnd, std::ptr::null());
    DestroyMenu(menu);
}
