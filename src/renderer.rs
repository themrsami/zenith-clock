use std::ptr;
use chrono::Local;
use windows_sys::Win32::Foundation::{HWND, POINT, SIZE};
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::Graphics::GdiPlus::*;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowRect, UpdateLayeredWindow, ULW_ALPHA};

use crate::config::Config;
use crate::theme::{get_scale_metrics, get_theme};

fn to_wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

unsafe fn create_rounded_rect_path(x: f32, y: f32, w: f32, h: f32, r: f32) -> *mut GpPath {
    let mut path: *mut GpPath = ptr::null_mut();
    GdipCreatePath(FillModeAlternate, &mut path);
    if path.is_null() {
        return ptr::null_mut();
    }

    let d = r * 2.0;
    // Top-left arc
    GdipAddPathArc(path, x, y, d, d, 180.0, 90.0);
    // Top-right arc
    GdipAddPathArc(path, x + w - d, y, d, d, 270.0, 90.0);
    // Bottom-right arc
    GdipAddPathArc(path, x + w - d, y + h - d, d, d, 0.0, 90.0);
    // Bottom-left arc
    GdipAddPathArc(path, x, y + h - d, d, d, 90.0, 90.0);
    GdipClosePathFigure(path);

    path
}

pub struct RenderResult {
    pub total_width: i32,
    pub total_height: i32,
}

pub unsafe fn render_clock(hwnd: HWND, config: &Config) -> RenderResult {
    let now = Local::now();
    let theme = get_theme(&config.theme);
    let scale = get_scale_metrics(&config.size_scale);

    // Prepare time strings
    let time_str = if config.time_format_24h {
        now.format("%H:%M").to_string()
    } else {
        let s = now.format("%I:%M").to_string();
        if s.starts_with('0') {
            s[1..].to_string()
        } else {
            s
        }
    };
    let seconds_str = now.format(":%S").to_string();
    let ampm_str = now.format("%p").to_string();
    let date_str = now.format("%a, %b %d").to_string();

    let w_time = to_wstr(&time_str);
    let w_seconds = to_wstr(&seconds_str);
    let w_ampm = to_wstr(&ampm_str);
    let w_date = to_wstr(&date_str);

    // Temporary screen DC to measure and create bitmap
    let screen_dc = GetDC(ptr::null_mut());
    let mem_dc = CreateCompatibleDC(screen_dc);

    // Font families
    let mut font_family: *mut GpFontFamily = ptr::null_mut();
    let font_name: Vec<u16> = "Segoe UI Variable Display\0".encode_utf16().collect();
    let status = GdipCreateFontFamilyFromName(font_name.as_ptr(), ptr::null_mut(), &mut font_family);
    if status != 0 || font_family.is_null() {
        let fallback: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
        GdipCreateFontFamilyFromName(fallback.as_ptr(), ptr::null_mut(), &mut font_family);
    }

    // Fonts
    let mut time_font: *mut GpFont = ptr::null_mut();
    let mut sec_font: *mut GpFont = ptr::null_mut();
    let mut ampm_font: *mut GpFont = ptr::null_mut();
    let mut date_font: *mut GpFont = ptr::null_mut();

    GdipCreateFont(font_family, scale.time_font_size, FontStyleBold, UnitPixel, &mut time_font);
    GdipCreateFont(font_family, scale.seconds_font_size, FontStyleBold, UnitPixel, &mut sec_font);
    GdipCreateFont(font_family, scale.ampm_font_size, FontStyleBold, UnitPixel, &mut ampm_font);
    GdipCreateFont(font_family, scale.date_font_size, FontStyleRegular, UnitPixel, &mut date_font);

    // Measure sizes using a dummy 1x1 graphics
    let mut measure_graphics: *mut GpGraphics = ptr::null_mut();
    GdipCreateFromHDC(mem_dc, &mut measure_graphics);

    let layout_rect = RectF { X: 0.0, Y: 0.0, Width: 1000.0, Height: 200.0 };
    let mut time_box = RectF { X: 0.0, Y: 0.0, Width: 0.0, Height: 0.0 };
    let mut sec_box = RectF { X: 0.0, Y: 0.0, Width: 0.0, Height: 0.0 };
    let mut ampm_box = RectF { X: 0.0, Y: 0.0, Width: 0.0, Height: 0.0 };
    let mut date_box = RectF { X: 0.0, Y: 0.0, Width: 0.0, Height: 0.0 };

    GdipMeasureString(measure_graphics, w_time.as_ptr(), w_time.len() as i32, time_font, &layout_rect, ptr::null_mut(), &mut time_box, ptr::null_mut(), ptr::null_mut());

    if config.show_seconds {
        GdipMeasureString(measure_graphics, w_seconds.as_ptr(), w_seconds.len() as i32, sec_font, &layout_rect, ptr::null_mut(), &mut sec_box, ptr::null_mut(), ptr::null_mut());
    }

    if !config.time_format_24h {
        GdipMeasureString(measure_graphics, w_ampm.as_ptr(), w_ampm.len() as i32, ampm_font, &layout_rect, ptr::null_mut(), &mut ampm_box, ptr::null_mut(), ptr::null_mut());
    }

    if config.show_date {
        GdipMeasureString(measure_graphics, w_date.as_ptr(), w_date.len() as i32, date_font, &layout_rect, ptr::null_mut(), &mut date_box, ptr::null_mut(), ptr::null_mut());
    }

    GdipDeleteGraphics(measure_graphics);

    // Calculate layout measurements
    let time_w = time_box.Width;
    let sec_w = if config.show_seconds { sec_box.Width + 4.0 } else { 0.0 };
    let ampm_pill_pad_x = 6.0;
    let ampm_w = if !config.time_format_24h { ampm_box.Width + ampm_pill_pad_x * 2.0 + 4.0 } else { 0.0 };
    let sep_w = if config.show_date { 16.0 } else { 0.0 };
    let date_w = if config.show_date { date_box.Width } else { 0.0 };

    let content_w = time_w + sec_w + ampm_w + sep_w + date_w;
    let content_h = time_box.Height.max(date_box.Height);

    let card_w = content_w + scale.pad_x * 2.0;
    let card_h = content_h + scale.pad_y * 2.0;

    let margin = scale.shadow_margin as f32;
    let total_w = (card_w + margin * 2.0).ceil() as i32;
    let total_h = (card_h + margin * 2.0).ceil() as i32;

    // Create 32-bit ARGB DIB Section for per-pixel alpha
    let mut bmi: BITMAPINFO = std::mem::zeroed();
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = total_w;
    bmi.bmiHeader.biHeight = -total_h; // top-down
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;

    let mut bits: *mut std::ffi::c_void = ptr::null_mut();
    let hbitmap = CreateDIBSection(screen_dc, &bmi, DIB_RGB_COLORS, &mut bits, ptr::null_mut(), 0);
    let old_bmp = SelectObject(mem_dc, hbitmap);

    let mut graphics: *mut GpGraphics = ptr::null_mut();
    GdipCreateFromHDC(mem_dc, &mut graphics);
    GdipSetSmoothingMode(graphics, SmoothingModeAntiAlias);
    GdipSetTextRenderingHint(graphics, TextRenderingHintAntiAliasGridFit);

    let card_x = margin;
    let card_y = margin;

    // 1. Draw soft cinematic drop shadow (multi-pass expanding rounded rects)
    let shadow_color = theme.shadow_color;
    let shadow_base_a = ((shadow_color >> 24) & 0xFF) as f32;
    let shadow_rgb = shadow_color & 0x00FFFFFF;

    for i in (1..=4).rev() {
        let expand = i as f32 * 2.0;
        let pass_alpha = (shadow_base_a / (5.0 * i as f32)) as u32;
        let pass_color = (pass_alpha << 24) | shadow_rgb;

        let mut shadow_brush: *mut GpSolidFill = ptr::null_mut();
        GdipCreateSolidFill(pass_color, &mut shadow_brush);

        let sh_path = create_rounded_rect_path(
            card_x - expand * 0.5,
            card_y + 3.0,
            card_w + expand,
            card_h + expand,
            scale.radius + expand * 0.5,
        );

        if !sh_path.is_null() {
            GdipFillPath(graphics, shadow_brush as *mut GpBrush, sh_path);
            GdipDeletePath(sh_path);
        }
        GdipDeleteBrush(shadow_brush as *mut GpBrush);
    }

    // 2. Draw card background capsule
    let mut bg_brush: *mut GpSolidFill = ptr::null_mut();
    GdipCreateSolidFill(theme.bg_color, &mut bg_brush);

    let card_path = create_rounded_rect_path(card_x, card_y, card_w, card_h, scale.radius);
    if !card_path.is_null() {
        GdipFillPath(graphics, bg_brush as *mut GpBrush, card_path);

        // Card border stroke
        let mut border_pen: *mut GpPen = ptr::null_mut();
        GdipCreatePen1(theme.border_color, 1.2, UnitPixel, &mut border_pen);
        GdipDrawPath(graphics, border_pen, card_path);
        GdipDeletePen(border_pen);

        GdipDeletePath(card_path);
    }
    GdipDeleteBrush(bg_brush as *mut GpBrush);

    // 3. String format for vertically centered text
    let mut str_format: *mut GpStringFormat = ptr::null_mut();
    GdipCreateStringFormat(0, 0, &mut str_format);
    GdipSetStringFormatLineAlign(str_format, StringAlignmentCenter);

    let center_y = card_y + card_h * 0.5;
    let mut current_x = card_x + scale.pad_x;

    // Draw main Hours:Minutes
    let mut text_brush: *mut GpSolidFill = ptr::null_mut();
    GdipCreateSolidFill(theme.text_primary, &mut text_brush);
    let time_rect = RectF {
        X: current_x,
        Y: card_y,
        Width: time_w + 2.0,
        Height: card_h,
    };
    GdipDrawString(graphics, w_time.as_ptr(), w_time.len() as i32, time_font, &time_rect, str_format, text_brush as *mut GpBrush);
    GdipDeleteBrush(text_brush as *mut GpBrush);
    current_x += time_w;

    // Draw Seconds
    if config.show_seconds {
        let mut sec_brush: *mut GpSolidFill = ptr::null_mut();
        GdipCreateSolidFill(theme.accent, &mut sec_brush);
        let sec_rect = RectF {
            X: current_x,
            Y: card_y + (card_h - sec_box.Height) * 0.5 + 1.0,
            Width: sec_w,
            Height: sec_box.Height,
        };
        GdipDrawString(graphics, w_seconds.as_ptr(), w_seconds.len() as i32, sec_font, &sec_rect, ptr::null_mut(), sec_brush as *mut GpBrush);
        GdipDeleteBrush(sec_brush as *mut GpBrush);
        current_x += sec_w;
    }

    // Draw AM/PM pill badge
    if !config.time_format_24h {
        current_x += 4.0;
        let pill_h = ampm_box.Height + 4.0;
        let pill_w = ampm_box.Width + ampm_pill_pad_x * 2.0;
        let pill_y = center_y - pill_h * 0.5;

        // Badge pill background
        let mut badge_bg_brush: *mut GpSolidFill = ptr::null_mut();
        GdipCreateSolidFill(theme.badge_bg, &mut badge_bg_brush);
        let pill_path = create_rounded_rect_path(current_x, pill_y, pill_w, pill_h, pill_h * 0.4);
        if !pill_path.is_null() {
            GdipFillPath(graphics, badge_bg_brush as *mut GpBrush, pill_path);
            GdipDeletePath(pill_path);
        }
        GdipDeleteBrush(badge_bg_brush as *mut GpBrush);

        // Badge text
        let mut badge_txt_brush: *mut GpSolidFill = ptr::null_mut();
        GdipCreateSolidFill(theme.badge_text, &mut badge_txt_brush);
        let ampm_rect = RectF {
            X: current_x,
            Y: pill_y,
            Width: pill_w,
            Height: pill_h,
        };
        let mut ampm_format: *mut GpStringFormat = ptr::null_mut();
        GdipCreateStringFormat(0, 0, &mut ampm_format);
        GdipSetStringFormatAlign(ampm_format, StringAlignmentCenter);
        GdipSetStringFormatLineAlign(ampm_format, StringAlignmentCenter);

        GdipDrawString(graphics, w_ampm.as_ptr(), w_ampm.len() as i32, ampm_font, &ampm_rect, ampm_format, badge_txt_brush as *mut GpBrush);
        GdipDeleteStringFormat(ampm_format);
        GdipDeleteBrush(badge_txt_brush as *mut GpBrush);

        current_x += pill_w;
    }

    // Draw Separator & Date
    if config.show_date {
        current_x += 8.0;
        // Subtle vertical separator line
        let mut sep_pen: *mut GpPen = ptr::null_mut();
        GdipCreatePen1(theme.border_color, 1.0, UnitPixel, &mut sep_pen);
        let sep_h = (card_h * 0.45).max(14.0);
        GdipDrawLineI(graphics, sep_pen, current_x as i32, (center_y - sep_h * 0.5) as i32, current_x as i32, (center_y + sep_h * 0.5) as i32);
        GdipDeletePen(sep_pen);

        current_x += 9.0;

        // Date text
        let mut date_brush: *mut GpSolidFill = ptr::null_mut();
        GdipCreateSolidFill(theme.text_secondary, &mut date_brush);
        let date_rect = RectF {
            X: current_x,
            Y: card_y,
            Width: date_w + 2.0,
            Height: card_h,
        };
        GdipDrawString(graphics, w_date.as_ptr(), w_date.len() as i32, date_font, &date_rect, str_format, date_brush as *mut GpBrush);
        GdipDeleteBrush(date_brush as *mut GpBrush);
    }

    GdipDeleteStringFormat(str_format);
    GdipDeleteFont(time_font);
    GdipDeleteFont(sec_font);
    GdipDeleteFont(ampm_font);
    GdipDeleteFont(date_font);
    GdipDeleteFontFamily(font_family);
    GdipDeleteGraphics(graphics);

    // Push frame to layered window via UpdateLayeredWindow
    let mut win_rect: windows_sys::Win32::Foundation::RECT = std::mem::zeroed();
    GetWindowRect(hwnd, &mut win_rect);

    let pt_dst = POINT { x: win_rect.left, y: win_rect.top };
    let size = SIZE { cx: total_w, cy: total_h };
    let pt_src = POINT { x: 0, y: 0 };

    let alpha_byte = (config.opacity.clamp(0.2, 1.0) * 255.0).round() as u8;
    let blend = BLENDFUNCTION {
        BlendOp: AC_SRC_OVER as u8,
        BlendFlags: 0,
        SourceConstantAlpha: alpha_byte,
        AlphaFormat: AC_SRC_ALPHA as u8,
    };

    UpdateLayeredWindow(
        hwnd,
        screen_dc,
        &pt_dst,
        &size,
        mem_dc,
        &pt_src,
        0,
        &blend,
        ULW_ALPHA,
    );

    SelectObject(mem_dc, old_bmp);
    DeleteObject(hbitmap);
    DeleteDC(mem_dc);
    ReleaseDC(ptr::null_mut(), screen_dc);

    RenderResult {
        total_width: total_w,
        total_height: total_h,
    }
}
