#![windows_subsystem = "windows"]

mod config;
mod icon;
mod menu;
mod renderer;
mod theme;

use std::mem::zeroed;
use std::ptr;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::GdiPlus::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows_sys::Win32::UI::Shell::{
    Shell_NotifyIconW, NOTIFYICONDATAW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE,
};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

use config::{load_config, save_config, Config};
use menu::*;
use renderer::render_clock;
use theme::THEMES;

const WM_TRAY: u32 = WM_APP + 1;
const TRAY_ID: u32 = 1001;

struct AppState {
    config: Config,
    width: i32,
    height: i32,
    gdiplus_token: usize,
    app_icon: windows_sys::Win32::UI::WindowsAndMessaging::HICON,
}

fn to_wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            let cs = lparam as *const CREATESTRUCTW;
            let state = (*cs).lpCreateParams as *mut AppState;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, state as isize);
            SetTimer(hwnd, 1, 200, None);
            0
        }
        WM_TIMER => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
            if !state_ptr.is_null() {
                let state = &mut *state_ptr;
                let res = render_clock(hwnd, &state.config);
                if res.total_width != state.width || res.total_height != state.height {
                    state.width = res.total_width;
                    state.height = res.total_height;
                    SetWindowPos(
                        hwnd,
                        ptr::null_mut(),
                        0,
                        0,
                        state.width,
                        state.height,
                        SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                }
            }
            0
        }
        WM_LBUTTONDOWN => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
            if !state_ptr.is_null() {
                let state = &*state_ptr;
                if !state.config.locked {
                    ReleaseCapture();
                    SendMessageW(hwnd, WM_NCLBUTTONDOWN, HTCAPTION as WPARAM, 0);
                }
            }
            0
        }
        WM_EXITSIZEMOVE => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
            if !state_ptr.is_null() {
                let state = &mut *state_ptr;
                let mut rect: RECT = zeroed();
                GetWindowRect(hwnd, &mut rect);

                let screen_w = GetSystemMetrics(SM_CXSCREEN);
                let mut new_x = rect.left;
                let mut new_y = rect.top;

                // Snap near top edge
                if new_y < 35 {
                    new_y = 14;
                }

                // Snap near center
                let center_x = (screen_w - state.width) / 2;
                if (new_x - center_x).abs() < 80 {
                    new_x = center_x;
                }

                if new_x != rect.left || new_y != rect.top {
                    SetWindowPos(
                        hwnd,
                        ptr::null_mut(),
                        new_x,
                        new_y,
                        state.width,
                        state.height,
                        SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                }

                state.config.pos_x = Some(new_x);
                state.config.pos_y = new_y;
                save_config(&state.config);
            }
            0
        }
        WM_RBUTTONUP => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
            if !state_ptr.is_null() {
                let state = &*state_ptr;
                let mut pt: POINT = zeroed();
                GetCursorPos(&mut pt);
                show_context_menu(hwnd, pt.x, pt.y, &state.config);
            }
            0
        }
        WM_COMMAND => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
            if !state_ptr.is_null() {
                let state = &mut *state_ptr;
                let cmd = wparam as usize;

                if cmd >= CMD_THEME_START && cmd < CMD_THEME_START + THEMES.len() {
                    let idx = cmd - CMD_THEME_START;
                    state.config.theme = THEMES[idx].key.to_string();
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_SCALE_START {
                    state.config.size_scale = "compact".to_string();
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_SCALE_START + 1 {
                    state.config.size_scale = "normal".to_string();
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_SCALE_START + 2 {
                    state.config.size_scale = "large".to_string();
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_OPACITY_START {
                    state.config.opacity = 1.0;
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_OPACITY_START + 1 {
                    state.config.opacity = 0.92;
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_OPACITY_START + 2 {
                    state.config.opacity = 0.75;
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_OPACITY_START + 3 {
                    state.config.opacity = 0.50;
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_TOGGLE_24H {
                    state.config.time_format_24h = !state.config.time_format_24h;
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_TOGGLE_SECONDS {
                    state.config.show_seconds = !state.config.show_seconds;
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_TOGGLE_DATE {
                    state.config.show_date = !state.config.show_date;
                    save_config(&state.config);
                    render_clock(hwnd, &state.config);
                } else if cmd == CMD_TOGGLE_LOCK {
                    state.config.locked = !state.config.locked;
                    save_config(&state.config);
                } else if cmd == CMD_SNAP_CENTER {
                    let screen_w = GetSystemMetrics(SM_CXSCREEN);
                    let center_x = (screen_w - state.width) / 2;
                    let center_y = 14;
                    SetWindowPos(
                        hwnd,
                        ptr::null_mut(),
                        center_x,
                        center_y,
                        state.width,
                        state.height,
                        SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                    state.config.pos_x = Some(center_x);
                    state.config.pos_y = center_y;
                    save_config(&state.config);
                } else if cmd == CMD_TOGGLE_TOPMOST {
                    state.config.always_on_top = !state.config.always_on_top;
                    save_config(&state.config);
                    let z_order = if state.config.always_on_top {
                        HWND_TOPMOST
                    } else {
                        HWND_NOTOPMOST
                    };
                    SetWindowPos(hwnd, z_order, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
                } else if cmd == CMD_ENABLE_CLICKTHROUGH {
                    state.config.click_through = true;
                    save_config(&state.config);
                    let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex | WS_EX_TRANSPARENT as isize);
                } else if cmd == CMD_DISABLE_CLICKTHROUGH {
                    state.config.click_through = false;
                    save_config(&state.config);
                    let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex & !(WS_EX_TRANSPARENT as isize));
                } else if cmd == CMD_TOGGLE_VISIBILITY {
                    let visible = IsWindowVisible(hwnd) != 0;
                    ShowWindow(hwnd, if visible { SW_HIDE } else { SW_SHOWNOACTIVATE });
                } else if cmd == CMD_EXIT {
                    DestroyWindow(hwnd);
                }
            }
            0
        }
        WM_TRAY => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
            if !state_ptr.is_null() {
                let state = &*state_ptr;
                let event = lparam as u32;
                if event == WM_LBUTTONUP {
                    let visible = IsWindowVisible(hwnd) != 0;
                    ShowWindow(hwnd, if visible { SW_HIDE } else { SW_SHOWNOACTIVATE });
                } else if event == WM_RBUTTONUP {
                    let mut pt: POINT = zeroed();
                    GetCursorPos(&mut pt);
                    show_tray_menu(hwnd, pt.x, pt.y, IsWindowVisible(hwnd) != 0, state.config.click_through);
                }
            }
            0
        }
        WM_DESTROY => {
            let state_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
            if !state_ptr.is_null() {
                let state = Box::from_raw(state_ptr);
                // Remove Tray Icon
                let mut nid: NOTIFYICONDATAW = zeroed();
                nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
                nid.hWnd = hwnd;
                nid.uID = TRAY_ID;
                Shell_NotifyIconW(NIM_DELETE, &nid);

                if !state.app_icon.is_null() {
                    DestroyIcon(state.app_icon);
                }
                GdiplusShutdown(state.gdiplus_token);
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn main() {
    unsafe {
        // High DPI setup
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

        // GDI+ startup
        let startup_input = GdiplusStartupInput {
            GdiplusVersion: 1,
            DebugEventCallback: 0,
            SuppressBackgroundThread: 0,
            SuppressExternalCodecs: 0,
        };
        let mut gdiplus_token: usize = 0;
        let gdi_status = GdiplusStartup(&mut gdiplus_token, &startup_input, ptr::null_mut());
        if gdi_status != 0 {
            return;
        }

        let config = load_config();
        let h_instance = GetModuleHandleW(ptr::null());

        // Create sleek custom vector clock icon
        let app_icon = icon::create_vector_clock_icon();
        let h_icon_to_use = if !app_icon.is_null() {
            app_icon
        } else {
            LoadIconW(ptr::null_mut(), IDI_APPLICATION)
        };

        let class_name = to_wstr("RustClockOverlayClass");
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: h_icon_to_use,
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
        };
        RegisterClassW(&wc);

        let mut ex_style = WS_EX_LAYERED | WS_EX_TOOLWINDOW;
        if config.always_on_top {
            ex_style |= WS_EX_TOPMOST;
        }
        if config.click_through {
            ex_style |= WS_EX_TRANSPARENT;
        }

        let state = Box::new(AppState {
            config: config.clone(),
            width: 320,
            height: 70,
            gdiplus_token,
            app_icon,
        });
        let state_raw = Box::into_raw(state);

        let window_title = to_wstr("Zenith Clock");
        let hwnd = CreateWindowExW(
            ex_style,
            class_name.as_ptr(),
            window_title.as_ptr(),
            WS_POPUP,
            0,
            0,
            320,
            70,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            state_raw as *mut std::ffi::c_void,
        );

        if hwnd.is_null() {
            if !app_icon.is_null() {
                DestroyIcon(app_icon);
            }
            GdiplusShutdown(gdiplus_token);
            return;
        }

        if !app_icon.is_null() {
            SendMessageW(hwnd, WM_SETICON, ICON_BIG as WPARAM, app_icon as LPARAM);
            SendMessageW(hwnd, WM_SETICON, ICON_SMALL as WPARAM, app_icon as LPARAM);
        }

        // Add System Tray Icon with vector clock
        let mut nid: NOTIFYICONDATAW = zeroed();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = TRAY_ID;
        nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        nid.uCallbackMessage = WM_TRAY;
        nid.hIcon = h_icon_to_use;
        let tip_str = to_wstr("Zenith Clock (Always-On-Top)");
        let copy_len = tip_str.len().min(nid.szTip.len());
        nid.szTip[..copy_len].copy_from_slice(&tip_str[..copy_len]);
        Shell_NotifyIconW(NIM_ADD, &nid);

        // Initial render to compute exact width & height
        let res = render_clock(hwnd, &config);
        (*state_raw).width = res.total_width;
        (*state_raw).height = res.total_height;

        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let init_x = match config.pos_x {
            Some(x) => x,
            None => (screen_w - res.total_width) / 2,
        };
        let init_y = config.pos_y;

        SetWindowPos(
            hwnd,
            if config.always_on_top { HWND_TOPMOST } else { HWND_NOTOPMOST },
            init_x,
            init_y,
            res.total_width,
            res.total_height,
            SWP_NOACTIVATE,
        );

        ShowWindow(hwnd, SW_SHOWNOACTIVATE);

        // Main message loop
        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
