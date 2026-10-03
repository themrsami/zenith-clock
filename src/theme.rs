#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub key: &'static str,
    pub name: &'static str,
    pub bg_color: u32,
    pub border_color: u32,
    pub shadow_color: u32,
    pub text_primary: u32,
    pub text_secondary: u32,
    pub accent: u32,
    pub badge_bg: u32,
    pub badge_text: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ScaleMetrics {
    pub time_font_size: f32,
    pub seconds_font_size: f32,
    pub ampm_font_size: f32,
    pub date_font_size: f32,
    pub pad_x: f32,
    pub pad_y: f32,
    pub radius: f32,
    pub shadow_margin: i32,
}

pub const THEMES: &[Theme] = &[
    Theme {
        key: "glass_dark",
        name: "🌌 Deep Glass (Dark)",
        bg_color: 0xDD121620,
        border_color: 0x33FFFFFF,
        shadow_color: 0xA0000000,
        text_primary: 0xFFF8FAFC,
        text_secondary: 0xFF94A3B8,
        accent: 0xFF38BDF8,
        badge_bg: 0x3338BDF8,
        badge_text: 0xFF38BDF8,
    },
    Theme {
        key: "midnight_gold",
        name: "✨ Midnight Gold",
        bg_color: 0xE01A1510,
        border_color: 0x55F59E0B,
        shadow_color: 0xB0201005,
        text_primary: 0xFFFFFBEB,
        text_secondary: 0xFFD97706,
        accent: 0xFFF59E0B,
        badge_bg: 0x40F59E0B,
        badge_text: 0xFFFBBF24,
    },
    Theme {
        key: "cyberpunk",
        name: "🔮 Cyberpunk Neon",
        bg_color: 0xE80C0A1A,
        border_color: 0x77EC4899,
        shadow_color: 0xC0140028,
        text_primary: 0xFF00F5FF,
        text_secondary: 0xFFC084FC,
        accent: 0xFFFF007F,
        badge_bg: 0x40FF007F,
        badge_text: 0xFFFF66B2,
    },
    Theme {
        key: "emerald",
        name: "🍃 Emerald Mist",
        bg_color: 0xE00A1C14,
        border_color: 0x5534D399,
        shadow_color: 0xB005180F,
        text_primary: 0xFFECFDF5,
        text_secondary: 0xFF6EE7B7,
        accent: 0xFF10B981,
        badge_bg: 0x4010B981,
        badge_text: 0xFF34D399,
    },
    Theme {
        key: "pure_frost",
        name: "☁️ Pure Frost (Light)",
        bg_color: 0xEEF8FAFC,
        border_color: 0x30000000,
        shadow_color: 0x50000000,
        text_primary: 0xFF0F172A,
        text_secondary: 0xFF475569,
        accent: 0xFF2563EB,
        badge_bg: 0x252563EB,
        badge_text: 0xFF2563EB,
    },
    Theme {
        key: "oled",
        name: "🖤 OLED Stealth",
        bg_color: 0xF4000000,
        border_color: 0x26FFFFFF,
        shadow_color: 0xD0000000,
        text_primary: 0xFFFFFFFF,
        text_secondary: 0xFFA1A1AA,
        accent: 0xFFE4E4E7,
        badge_bg: 0x22FFFFFF,
        badge_text: 0xFFFAFAFA,
    },
    Theme {
        key: "sunset",
        name: "🌅 Sunset Glow",
        bg_color: 0xE01C0E1C,
        border_color: 0x60FB7185,
        shadow_color: 0xB0250A14,
        text_primary: 0xFFFFF1F2,
        text_secondary: 0xFFFDA4AF,
        accent: 0xFFF43F5E,
        badge_bg: 0x40F43F5E,
        badge_text: 0xFFFB7185,
    },
];

pub fn get_theme(key: &str) -> Theme {
    for t in THEMES {
        if t.key == key {
            return *t;
        }
    }
    THEMES[0]
}

pub fn get_scale_metrics(scale: &str) -> ScaleMetrics {
    match scale {
        "compact" => ScaleMetrics {
            time_font_size: 21.0,
            seconds_font_size: 13.0,
            ampm_font_size: 10.0,
            date_font_size: 12.0,
            pad_x: 16.0,
            pad_y: 8.0,
            radius: 18.0,
            shadow_margin: 14,
        },
        "large" => ScaleMetrics {
            time_font_size: 34.0,
            seconds_font_size: 18.0,
            ampm_font_size: 14.0,
            date_font_size: 15.0,
            pad_x: 28.0,
            pad_y: 12.0,
            radius: 26.0,
            shadow_margin: 18,
        },
        _ => ScaleMetrics {
            time_font_size: 27.0,
            seconds_font_size: 15.0,
            ampm_font_size: 12.0,
            date_font_size: 13.0,
            pad_x: 22.0,
            pad_y: 10.0,
            radius: 22.0,
            shadow_margin: 16,
        },
    }
}
