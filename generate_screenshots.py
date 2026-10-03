import os
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, ImageFilter

ASSETS_DIR = Path("assets/screenshots")
ASSETS_DIR.mkdir(parents=True, exist_ok=True)

FONT_REGULAR = "C:/Windows/Fonts/segoeui.ttf"
FONT_BOLD = "C:/Windows/Fonts/segoeuib.ttf"
FONT_SEMIBOLD = "C:/Windows/Fonts/seguisb.ttf" if os.path.exists("C:/Windows/Fonts/seguisb.ttf") else FONT_BOLD

THEMES = [
    {
        "key": "glass_dark",
        "name": "🌌 Deep Glass (Dark)",
        "bg": (18, 22, 32, 220),
        "border": (255, 255, 255, 38),
        "time": (248, 250, 252, 255),
        "sec": (56, 189, 248, 255),
        "badge_bg": (56, 189, 248, 45),
        "badge_txt": (56, 189, 248, 255),
        "date": (148, 163, 184, 255),
        "sep": (255, 255, 255, 35),
        "glow": (56, 189, 248, 40),
    },
    {
        "key": "midnight_gold",
        "name": "✨ Midnight Gold",
        "bg": (26, 21, 16, 225),
        "border": (245, 158, 11, 75),
        "time": (255, 251, 235, 255),
        "sec": (245, 158, 11, 255),
        "badge_bg": (245, 158, 11, 55),
        "badge_txt": (251, 191, 36, 255),
        "date": (217, 119, 6, 255),
        "sep": (245, 158, 11, 50),
        "glow": (245, 158, 11, 50),
    },
    {
        "key": "cyberpunk",
        "name": "🔮 Cyberpunk Neon",
        "bg": (12, 10, 26, 235),
        "border": (236, 72, 153, 115),
        "time": (0, 245, 255, 255),
        "sec": (255, 0, 127, 255),
        "badge_bg": (255, 0, 127, 65),
        "badge_txt": (255, 102, 178, 255),
        "date": (192, 132, 252, 255),
        "sep": (236, 72, 153, 70),
        "glow": (0, 245, 255, 55),
    },
    {
        "key": "emerald",
        "name": "🍃 Emerald Mist",
        "bg": (10, 28, 20, 225),
        "border": (52, 211, 153, 75),
        "time": (236, 253, 245, 255),
        "sec": (16, 185, 129, 255),
        "badge_bg": (16, 185, 129, 55),
        "badge_txt": (52, 211, 153, 255),
        "date": (110, 231, 183, 255),
        "sep": (52, 211, 153, 50),
        "glow": (16, 185, 129, 45),
    },
    {
        "key": "pure_frost",
        "name": "☁️ Pure Frost (Light)",
        "bg": (245, 247, 250, 235),
        "border": (0, 0, 0, 35),
        "time": (15, 23, 42, 255),
        "sec": (37, 99, 235, 255),
        "badge_bg": (37, 99, 235, 35),
        "badge_txt": (37, 99, 235, 255),
        "date": (71, 85, 105, 255),
        "sep": (0, 0, 0, 30),
        "glow": (37, 99, 235, 30),
    },
    {
        "key": "oled",
        "name": "🖤 OLED Stealth",
        "bg": (0, 0, 0, 245),
        "border": (255, 255, 255, 45),
        "time": (255, 255, 255, 255),
        "sec": (228, 228, 231, 255),
        "badge_bg": (255, 255, 255, 35),
        "badge_txt": (250, 250, 250, 255),
        "date": (161, 161, 170, 255),
        "sep": (255, 255, 255, 30),
        "glow": (255, 255, 255, 25),
    },
    {
        "key": "sunset",
        "name": "🌅 Sunset Glow",
        "bg": (28, 14, 28, 230),
        "border": (251, 113, 133, 95),
        "time": (255, 241, 242, 255),
        "sec": (244, 63, 94, 255),
        "badge_bg": (244, 63, 94, 55),
        "badge_txt": (251, 113, 133, 255),
        "date": (253, 164, 175, 255),
        "sep": (251, 113, 133, 50),
        "glow": (244, 63, 94, 50),
    },
]

def draw_clock_widget(theme, scale=1.0, time_str="10:45", sec_str=":28", ampm="AM", date_str="Sun, Oct 04"):
    # High-res 2x multiplier for crisp retina output
    m = 2.0 * scale
    
    font_time = ImageFont.truetype(FONT_BOLD, int(26 * m))
    font_sec = ImageFont.truetype(FONT_BOLD, int(15 * m))
    font_ampm = ImageFont.truetype(FONT_BOLD, int(11 * m))
    font_date = ImageFont.truetype(FONT_REGULAR, int(13 * m))

    # Measure texts
    dummy_img = Image.new("RGBA", (1, 1))
    draw_d = ImageDraw.Draw(dummy_img)

    time_bbox = draw_d.textbbox((0, 0), time_str, font=font_time)
    sec_bbox = draw_d.textbbox((0, 0), sec_str, font=font_sec)
    ampm_bbox = draw_d.textbbox((0, 0), ampm, font=font_ampm)
    date_bbox = draw_d.textbbox((0, 0), date_str, font=font_date)

    time_w = time_bbox[2] - time_bbox[0]
    sec_w = sec_bbox[2] - sec_bbox[0]
    ampm_w = ampm_bbox[2] - ampm_bbox[0]
    date_w = date_bbox[2] - date_bbox[0]

    pad_x = int(22 * m)
    pad_y = int(10 * m)
    spacing = int(8 * m)
    pill_pad_x = int(6 * m)
    pill_pad_y = int(2 * m)

    ampm_pill_w = ampm_w + pill_pad_x * 2
    ampm_pill_h = (ampm_bbox[3] - ampm_bbox[1]) + pill_pad_y * 2 + int(2 * m)
    sep_w = int(14 * m)

    content_w = time_w + spacing + sec_w + spacing + ampm_pill_w + sep_w + date_w
    content_h = max(time_bbox[3] - time_bbox[1], date_bbox[3] - date_bbox[1])

    card_w = content_w + pad_x * 2
    card_h = content_h + pad_y * 2
    radius = int(22 * m)

    # Shadow padding
    shadow_margin = int(24 * m)
    img_w = card_w + shadow_margin * 2
    img_h = card_h + shadow_margin * 2

    # Draw shadow on separate layer
    shadow_layer = Image.new("RGBA", (img_w, img_h), (0, 0, 0, 0))
    sdraw = ImageDraw.Draw(shadow_layer)
    card_x0 = shadow_margin
    card_y0 = shadow_margin
    card_x1 = card_x0 + card_w
    card_y1 = card_y0 + card_h

    # Soft gaussian drop shadow
    shadow_color = (0, 0, 0, 140) if theme["key"] != "pure_frost" else (0, 0, 0, 60)
    sdraw.rounded_rectangle(
        [card_x0 - int(2*m), card_y0 + int(4*m), card_x1 + int(2*m), card_y1 + int(8*m)],
        radius=radius + int(2*m),
        fill=shadow_color
    )
    # Optional colored glow
    if "glow" in theme:
        sdraw.rounded_rectangle(
            [card_x0 - int(4*m), card_y0 + int(2*m), card_x1 + int(4*m), card_y1 + int(6*m)],
            radius=radius + int(4*m),
            fill=theme["glow"]
        )
    shadow_layer = shadow_layer.filter(ImageFilter.GaussianBlur(radius=int(10 * m)))

    # Main card layer
    card_layer = Image.new("RGBA", (img_w, img_h), (0, 0, 0, 0))
    cdraw = ImageDraw.Draw(card_layer)

    # Card background
    cdraw.rounded_rectangle(
        [card_x0, card_y0, card_x1, card_y1],
        radius=radius,
        fill=theme["bg"],
        outline=theme["border"],
        width=max(1, int(1.5 * m))
    )

    # Content layout
    center_y = card_y0 + card_h / 2.0
    cur_x = card_x0 + pad_x

    # Hours : Minutes
    time_h = time_bbox[3] - time_bbox[1]
    time_top = center_y - time_h / 2.0 - time_bbox[1]
    cdraw.text((cur_x, time_top), time_str, font=font_time, fill=theme["time"])
    cur_x += time_w + int(3 * m)

    # Seconds
    sec_h = sec_bbox[3] - sec_bbox[1]
    sec_top = center_y - sec_h / 2.0 - sec_bbox[1] + int(1 * m)
    cdraw.text((cur_x, sec_top), sec_str, font=font_sec, fill=theme["sec"])
    cur_x += sec_w + int(6 * m)

    # AM/PM Pill
    pill_top = center_y - ampm_pill_h / 2.0
    pill_rect = [cur_x, pill_top, cur_x + ampm_pill_w, pill_top + ampm_pill_h]
    cdraw.rounded_rectangle(pill_rect, radius=int(6 * m), fill=theme["badge_bg"])
    
    txt_x = cur_x + pill_pad_x
    txt_y = pill_top + (ampm_pill_h - (ampm_bbox[3] - ampm_bbox[1])) / 2.0 - ampm_bbox[1]
    cdraw.text((txt_x, txt_y), ampm, font=font_ampm, fill=theme["badge_txt"])
    cur_x += ampm_pill_w + int(8 * m)

    # Separator
    sep_h = int(18 * m)
    cdraw.line(
        [(cur_x, center_y - sep_h / 2.0), (cur_x, center_y + sep_h / 2.0)],
        fill=theme["sep"],
        width=max(1, int(1.2 * m))
    )
    cur_x += int(10 * m)

    # Date
    date_h = date_bbox[3] - date_bbox[1]
    date_top = center_y - date_h / 2.0 - date_bbox[1]
    cdraw.text((cur_x, date_top), date_str, font=font_date, fill=theme["date"])

    # Composite shadow and card
    final_img = Image.alpha_composite(shadow_layer, card_layer)
    return final_img

# 1. Generate individual theme showcases with presentation cards
print("Generating individual theme showcases...")
for t in THEMES:
    widget_img = draw_clock_widget(t, scale=1.0)
    
    # Backdrop card for GitHub display (so transparent borders & shadows look amazing on both dark & light GitHub themes)
    card_bg_color = (13, 17, 23, 255) # GitHub dark mode background
    card_w = max(680, widget_img.width + 40)
    card_h = widget_img.height + 30
    
    canvas = Image.new("RGBA", (card_w, card_h), (0, 0, 0, 0))
    cdraw = ImageDraw.Draw(canvas)
    
    # Modern card backdrop
    cdraw.rounded_rectangle([10, 8, card_w - 10, card_h - 8], radius=16, fill=(18, 22, 30, 240), outline=(255, 255, 255, 25), width=1)
    
    # Paste widget centered
    pos_x = (card_w - widget_img.width) // 2
    pos_y = (card_h - widget_img.height) // 2
    canvas.paste(widget_img, (pos_x, pos_y), widget_img)
    
    # Downsample with Lanczos to 50% for super crisp anti-aliasing
    final_card = canvas.resize((canvas.width // 2, canvas.height // 2), Image.Resampling.LANCZOS)
    filename = ASSETS_DIR / f"theme_{t['key']}.png"
    final_card.save(filename, "PNG")
    print(f"Saved: {filename}")

# 2. Generate Size Scale comparison (Compact, Normal, Large)
print("Generating size variants comparison...")
scales = [
    ("Compact (Small)", 0.78, "10:45", ":28", "AM", "Sun, Oct 04"),
    ("Normal (Standard)", 1.0, "10:45", ":28", "AM", "Sun, Oct 04"),
    ("Large (Prominent)", 1.25, "10:45", ":28", "AM", "Sun, Oct 04"),
]

scale_imgs = []
max_w = 0
for label, sc, t_str, s_str, ap_str, d_str in scales:
    w_img = draw_clock_widget(THEMES[0], scale=sc, time_str=t_str, sec_str=s_str, ampm=ap_str, date_str=d_str)
    scale_imgs.append((label, w_img))
    if w_img.width > max_w:
        max_w = w_img.width

var_canvas_w = max_w + 100
total_height = sum(img.height + 50 for _, img in scale_imgs) + 60
var_canvas = Image.new("RGBA", (var_canvas_w, total_height), (0, 0, 0, 0))
vdraw = ImageDraw.Draw(var_canvas)
vdraw.rounded_rectangle([10, 10, var_canvas_w - 10, total_height - 10], radius=20, fill=(16, 20, 28, 245), outline=(255, 255, 255, 30), width=1)

lbl_font = ImageFont.truetype(FONT_BOLD, 22)

curr_y = 35
for label, w_img in scale_imgs:
    # Draw label
    vdraw.text((40, curr_y), f"●  {label}", font=lbl_font, fill=(148, 163, 184, 255))
    curr_y += 35
    
    # Paste widget centered
    wx = (var_canvas_w - w_img.width) // 2
    var_canvas.paste(w_img, (wx, curr_y), w_img)
    curr_y += w_img.height + 25

final_var = var_canvas.resize((var_canvas.width // 2, var_canvas.height // 2), Image.Resampling.LANCZOS)
final_var.save(ASSETS_DIR / "variants_sizes.png", "PNG")
print("Saved: assets/screenshots/variants_sizes.png")

# 3. Generate Hero Banner showcasing floating desktop placement
print("Generating hero banner...")
banner_w = 1200
banner_h = 520
hero = Image.new("RGBA", (banner_w * 2, banner_h * 2), (10, 12, 18, 255))
hdraw = ImageDraw.Draw(hero)

# Draw subtle ambient background gradient lights (synthwave / cyber bloom)
for r in range(400, 0, -10):
    alpha = int((1.0 - r / 400.0) * 45)
    hdraw.ellipse([banner_w * 2 // 2 - r * 2, 80 - r, banner_w * 2 // 2 + r * 2, 80 + r], fill=(56, 189, 248, alpha))
    hdraw.ellipse([banner_w * 2 // 4 - r, 300 - r, banner_w * 2 // 4 + r, 300 + r], fill=(139, 92, 246, int(alpha * 0.7)))
    hdraw.ellipse([banner_w * 2 * 3 // 4 - r, 320 - r, banner_w * 2 * 3 // 4 + r, 320 + r], fill=(236, 72, 153, int(alpha * 0.6)))

# Top monitor bezel line
hdraw.line([(0, 4), (banner_w * 2, 4)], fill=(255, 255, 255, 30), width=4)

# Render widget at top center (just like on actual desktop)
hero_widget = draw_clock_widget(THEMES[0], scale=1.1)
hero_wx = (banner_w * 2 - hero_widget.width) // 2
hero_wy = 50
hero.paste(hero_widget, (hero_wx, hero_wy), hero_widget)

# Add sleek typography underneath
font_hero_title = ImageFont.truetype(FONT_BOLD, 54)
font_hero_sub = ImageFont.truetype(FONT_REGULAR, 26)
font_badge = ImageFont.truetype(FONT_BOLD, 20)

title_text = "Zenith Clock Overlay"
sub_text = "A lightweight, glassmorphic always-on-top desktop clock for Windows in 100% Rust"

tbbox = hdraw.textbbox((0, 0), title_text, font=font_hero_title)
tw = tbbox[2] - tbbox[0]
sbbox = hdraw.textbbox((0, 0), sub_text, font=font_hero_sub)
sw = sbbox[2] - sbbox[0]

hdraw.text(((banner_w * 2 - tw) // 2, 440), title_text, font=font_hero_title, fill=(248, 250, 252, 255))
hdraw.text(((banner_w * 2 - sw) // 2, 530), sub_text, font=font_hero_sub, fill=(148, 163, 184, 255))

# Draw feature pills at bottom
pills = ["🦀 100% Pure Rust", "⚡ 0.0% CPU & ~5 MB RAM", "🪟 Windows 11 Layered Glass", "🎨 7 Dynamic Themes", "💾 ~390 KB Binary"]
pill_y = 630
total_pills_w = len(pills) * 200
curr_px = (banner_w * 2 - total_pills_w) // 2

for p in pills:
    pbbox = hdraw.textbbox((0, 0), p, font=font_badge)
    pw = pbbox[2] - pbbox[0] + 36
    ph = (pbbox[3] - pbbox[1]) + 22
    hdraw.rounded_rectangle([curr_px, pill_y, curr_px + pw, pill_y + ph], radius=ph // 2, fill=(255, 255, 255, 18), outline=(255, 255, 255, 35), width=2)
    hdraw.text((curr_px + 18, pill_y + 9), p, font=font_badge, fill=(226, 232, 240, 255))
    curr_px += pw + 20

# Final downscale for razor sharpness
final_hero = hero.resize((banner_w, banner_h), Image.Resampling.LANCZOS)
final_hero.save(ASSETS_DIR / "hero_banner.png", "PNG")
print("Saved: assets/screenshots/hero_banner.png")
print("All assets successfully generated!")
