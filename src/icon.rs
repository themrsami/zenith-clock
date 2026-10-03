use windows_sys::Win32::Graphics::GdiPlus::*;
use windows_sys::Win32::UI::WindowsAndMessaging::HICON;

const PIXEL_FORMAT_32BPP_ARGB: i32 = 0x26200A;

pub unsafe fn create_vector_clock_icon() -> HICON {
    let size = 64;
    let mut bitmap: *mut GpBitmap = std::ptr::null_mut();
    GdipCreateBitmapFromScan0(size, size, 0, PIXEL_FORMAT_32BPP_ARGB, std::ptr::null_mut(), &mut bitmap);
    if bitmap.is_null() {
        return std::ptr::null_mut();
    }

    let mut graphics: *mut GpGraphics = std::ptr::null_mut();
    GdipGetImageGraphicsContext(bitmap as *mut GpImage, &mut graphics);
    GdipSetSmoothingMode(graphics, SmoothingModeAntiAlias);

    let center = size as f32 / 2.0;
    let radius = (size as f32 / 2.0) - 4.0;

    // Dark sleek circle background
    let mut bg_brush: *mut GpSolidFill = std::ptr::null_mut();
    GdipCreateSolidFill(0xFF0F172A, &mut bg_brush);
    GdipFillEllipse(graphics, bg_brush as *mut GpBrush, center - radius, center - radius, radius * 2.0, radius * 2.0);
    GdipDeleteBrush(bg_brush as *mut GpBrush);

    // Cyan glowing ring
    let mut ring_pen: *mut GpPen = std::ptr::null_mut();
    GdipCreatePen1(0xFF38BDF8, 3.5, UnitPixel, &mut ring_pen);
    GdipDrawEllipse(graphics, ring_pen, center - radius, center - radius, radius * 2.0, radius * 2.0);
    GdipDeletePen(ring_pen);

    // Hour hand (pointing ~ 10:10)
    let mut hour_pen: *mut GpPen = std::ptr::null_mut();
    GdipCreatePen1(0xFFFFFFFF, 3.5, UnitPixel, &mut hour_pen);
    GdipDrawLine(graphics, hour_pen, center, center, center - 12.0, center - 10.0);
    GdipDeletePen(hour_pen);

    // Minute hand
    let mut min_pen: *mut GpPen = std::ptr::null_mut();
    GdipCreatePen1(0xFF38BDF8, 2.5, UnitPixel, &mut min_pen);
    GdipDrawLine(graphics, min_pen, center, center, center + 14.0, center - 15.0);
    GdipDeletePen(min_pen);

    // Center dot
    let mut dot_brush: *mut GpSolidFill = std::ptr::null_mut();
    GdipCreateSolidFill(0xFF38BDF8, &mut dot_brush);
    GdipFillEllipse(graphics, dot_brush as *mut GpBrush, center - 3.0, center - 3.0, 6.0, 6.0);
    GdipDeleteBrush(dot_brush as *mut GpBrush);

    let mut hicon: HICON = std::ptr::null_mut();
    GdipCreateHICONFromBitmap(bitmap, &mut hicon);

    GdipDeleteGraphics(graphics);
    GdipDisposeImage(bitmap as *mut GpImage);

    hicon
}
