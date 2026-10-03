import os
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, ImageFilter

ASSETS_DIR = Path("assets/screenshots")
FONT_REGULAR = "C:/Windows/Fonts/segoeui.ttf"
FONT_BOLD = "C:/Windows/Fonts/segoeuib.ttf"
FONT_SEMIBOLD = "C:/Windows/Fonts/seguisb.ttf" if os.path.exists("C:/Windows/Fonts/seguisb.ttf") else FONT_BOLD

W = 2560
H = 1000

# Base dark canvas with subtle deep blue ambient gradient
img = Image.new("RGBA", (W, H), (11, 14, 22, 255))
draw = ImageDraw.Draw(img)

for y in range(H):
    t = y / float(H)
    r = int(10 + 6 * t)
    g = int(14 + 8 * t)
    b = int(24 + 14 * t)
    draw.line([(0, y), (W, y)], fill=(r, g, b, 255))

# Top screen frame / subtle bezel
draw.line([(0, 0), (W, 0)], fill=(255, 255, 255, 60), width=4)
draw.line([(0, 4), (W, 4)], fill=(56, 189, 248, 80), width=2)

# Soft tasteful background aura behind clock
aura = Image.new("RGBA", (W, H), (0, 0, 0, 0))
adraw = ImageDraw.Draw(aura)
adraw.ellipse([W // 2 - 550, 40, W // 2 + 550, 360], fill=(56, 189, 248, 30))
aura = aura.filter(ImageFilter.GaussianBlur(radius=100))
img = Image.alpha_composite(img, aura)

# Large Prominent Clock Widget
def draw_large_hero_widget():
    m = 2.4  # Large scale for hero prominence
    font_time = ImageFont.truetype(FONT_BOLD, int(32 * m))
    font_sec = ImageFont.truetype(FONT_BOLD, int(19 * m))
    font_ampm = ImageFont.truetype(FONT_BOLD, int(14 * m))
    font_date = ImageFont.truetype(FONT_REGULAR, int(16 * m))

    time_str = "10:45"
    sec_str = ":28"
    ampm_str = "AM"
    date_str = "Sun, Oct 04"

    d_test = ImageDraw.Draw(Image.new("RGBA", (1, 1)))
    tb = d_test.textbbox((0, 0), time_str, font=font_time)
    sb = d_test.textbbox((0, 0), sec_str, font=font_sec)
    ab = d_test.textbbox((0, 0), ampm_str, font=font_ampm)
    db = d_test.textbbox((0, 0), date_str, font=font_date)

    tw = tb[2] - tb[0]
    sw = sb[2] - sb[0]
    aw = ab[2] - ab[0]
    dw = db[2] - db[0]

    pad_x = int(32 * m)
    pill_px = int(10 * m)
    pill_py = int(4 * m)
    pill_w = aw + pill_px * 2
    pill_h = (ab[3] - ab[1]) + pill_py * 2 + int(4 * m)

    total_w = tw + int(12 * m) + sw + int(18 * m) + pill_w + int(24 * m) + dw
    card_w = total_w + pad_x * 2
    card_h = int(64 * m)
    radius = int(card_h / 2)

    shadow_m = int(50 * m)
    full_w = card_w + shadow_m * 2
    full_h = card_h + shadow_m * 2

    # Drop shadow
    sh_layer = Image.new("RGBA", (full_w, full_h), (0, 0, 0, 0))
    sdraw = ImageDraw.Draw(sh_layer)
    sdraw.rounded_rectangle(
        [shadow_m, shadow_m + int(10 * m), shadow_m + card_w, shadow_m + card_h + int(16 * m)],
        radius=radius,
        fill=(0, 0, 0, 180)
    )
    sdraw.rounded_rectangle(
        [shadow_m - int(6*m), shadow_m + int(4 * m), shadow_m + card_w + int(6*m), shadow_m + card_h + int(10 * m)],
        radius=radius + 4,
        fill=(56, 189, 248, 55)
    )
    sh_layer = sh_layer.filter(ImageFilter.GaussianBlur(radius=int(22 * m)))

    # Card
    c_layer = Image.new("RGBA", (full_w, full_h), (0, 0, 0, 0))
    cdraw = ImageDraw.Draw(c_layer)
    cx0, cy0 = shadow_m, shadow_m
    cx1, cy1 = cx0 + card_w, cy0 + card_h

    # Acrylic dark background with dual border
    cdraw.rounded_rectangle([cx0, cy0, cx1, cy1], radius=radius, fill=(18, 24, 36, 235), outline=(255, 255, 255, 55), width=int(1.8 * m))

    center_y = cy0 + card_h / 2.0
    cur_x = cx0 + pad_x

    # Hours : Minutes
    th = tb[3] - tb[1]
    cdraw.text((cur_x, center_y - th / 2.0 - tb[1]), time_str, font=font_time, fill=(248, 250, 252, 255))
    cur_x += tw + int(6 * m)

    # Seconds
    sh = sb[3] - sb[1]
    cdraw.text((cur_x, center_y - sh / 2.0 - sb[1] + int(2 * m)), sec_str, font=font_sec, fill=(56, 189, 248, 255))
    cur_x += sw + int(18 * m)

    # AM/PM Pill
    py0 = center_y - pill_h / 2.0
    cdraw.rounded_rectangle([cur_x, py0, cur_x + pill_w, py0 + pill_h], radius=int(pill_h * 0.4), fill=(56, 189, 248, 50))
    cdraw.text((cur_x + pill_px, py0 + (pill_h - (ab[3]-ab[1])) / 2.0 - ab[1]), ampm_str, font=font_ampm, fill=(56, 189, 248, 255))
    cur_x += pill_w + int(20 * m)

    # Separator
    cdraw.line([(cur_x, center_y - int(15 * m)), (cur_x, center_y + int(15 * m))], fill=(255, 255, 255, 50), width=max(1, int(1.5 * m)))
    cur_x += int(20 * m)

    # Date
    dh = db[3] - db[1]
    cdraw.text((cur_x, center_y - dh / 2.0 - db[1]), date_str, font=font_date, fill=(148, 163, 184, 255))

    return Image.alpha_composite(sh_layer, c_layer)

widget = draw_large_hero_widget()
wx = (W - widget.width) // 2
wy = 60
img.paste(widget, (wx, wy), widget)
draw = ImageDraw.Draw(img)

# Typography Section
font_badge = ImageFont.truetype(FONT_BOLD, 24)
font_title = ImageFont.truetype(FONT_BOLD, 74)
font_sub = ImageFont.truetype(FONT_REGULAR, 32)
font_chip = ImageFont.truetype(FONT_BOLD, 22)

# Badge: 100% PURE RUST • NATIVE WIN32
badge_text = "100% PURE RUST  •  NATIVE WIN32  •  ZERO RUNTIME"
bb = draw.textbbox((0, 0), badge_text, font=font_badge)
bw = bb[2] - bb[0]
bh = bb[3] - bb[1]
b_pad_x = 30
b_pad_y = 12
b_w = bw + b_pad_x * 2
b_h = bh + b_pad_y * 2
b_x = (W - b_w) // 2
b_y = 480

draw.rounded_rectangle([b_x, b_y, b_x + b_w, b_y + b_h], radius=b_h // 2, fill=(56, 189, 248, 22), outline=(56, 189, 248, 80), width=2)
draw.text((b_x + b_pad_x, b_y + b_pad_y - bb[1]), badge_text, font=font_badge, fill=(56, 189, 248, 255))

# Title: Zenith Clock
title_text = "Zenith Clock"
tb = draw.textbbox((0, 0), title_text, font=font_title)
tw = tb[2] - tb[0]
draw.text(((W - tw) // 2, 570), title_text, font=font_title, fill=(248, 250, 252, 255))

# Subtitle
sub_text = "Minimalist, glassmorphic always-on-top desktop clock overlay for Windows 10 & 11"
sb = draw.textbbox((0, 0), sub_text, font=font_sub)
sw = sb[2] - sb[0]
draw.text(((W - sw) // 2, 675), sub_text, font=font_sub, fill=(148, 163, 184, 255))

# Feature Chips
chips = [
    ("0.0% CPU", (56, 189, 248)),
    ("~5 MB RAM", (168, 85, 247)),
    ("390 KB Binary", (52, 211, 153)),
    ("Windows 11 Acrylic", (251, 191, 36)),
    ("7 Live Themes", (244, 63, 94)),
]

chip_y = 780
chip_boxes = []
total_chips_w = 0

for text, dot_col in chips:
    cb = draw.textbbox((0, 0), text, font=font_chip)
    dot_space = 24
    cw = cb[2] - cb[0] + 54 + dot_space
    ch = cb[3] - cb[1] + 28
    chip_boxes.append((text, dot_col, cw, ch, cb))
    total_chips_w += cw

spacing_chip = 24
total_chips_w += spacing_chip * (len(chips) - 1)
cur_x = (W - total_chips_w) // 2

for text, dot_col, cw, ch, cb in chip_boxes:
    # Chip pill with dark translucent background
    draw.rounded_rectangle(
        [cur_x, chip_y, cur_x + cw, chip_y + ch],
        radius=ch // 2,
        fill=(22, 28, 44, 230),
        outline=(255, 255, 255, 35),
        width=2
    )
    # Colored indicator dot
    dot_y = chip_y + ch // 2
    dot_x = cur_x + 24
    dot_r = 6
    draw.ellipse([dot_x - dot_r, dot_y - dot_r, dot_x + dot_r, dot_y + dot_r], fill=dot_col)
    # Crisp white text
    draw.text((dot_x + 18, chip_y + (ch - (cb[3] - cb[1])) // 2 - cb[1]), text, font=font_chip, fill=(248, 250, 252, 255))
    cur_x += cw + spacing_chip

# Downsample to 1280x500 for crisp retina quality
final = img.resize((1280, 500), Image.Resampling.LANCZOS)
final.save(ASSETS_DIR / "hero_banner.png", "PNG")
print("New hero_banner.png saved successfully!")
