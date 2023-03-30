use image::RgbaImage;
use regex::Regex;
use win_screenshot::prelude::*;

fn main() {
    // Capture entire screen
    let buf = capture_display().unwrap();

    // Capture window by known id
    let buf = capture_window(11996706).unwrap();

    // Capture window if you know the exact name
    let hwnd = find_window("Notepad").unwrap();
    let buf = capture_window(hwnd).unwrap();

    // If you don't know the exact name, try to find it
    let re = Regex::new(r"Firefox").unwrap();
    let hwnd = window_list()
        .unwrap()
        .iter()
        .find(|i| re.is_match(&i.window_name))
        .unwrap()
        .hwnd;
    let buf = capture_window(hwnd).unwrap();

    // convert to image and save
    let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    img.save("screenshot.jpg").unwrap();

    // Fine tuning

    // BitBlt dramatically faster, often fails
    // (e.g. firefox, steam, 3d accelerated windows)
    let using = Using::BitBlt;
    // PrintWindow much slower, much more reliable
    let using = Using::PrintWindow;

    // Capture client area of window
    let area = Area::ClientOnly;
    // Capture whole window (not supported with BitBlt)
    let area = Area::Full;

    // Build-in crop, faster on large windows
    let crop_xy = None; //Some([100, 100]);
    let crop_wh = None; //Some([300, 300]);
    let buf = capture_window_ex(hwnd, using, area, crop_xy, crop_wh).unwrap();
}
