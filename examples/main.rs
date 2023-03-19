use image::{ImageBuffer, Rgb};
use regex::Regex;
use win_screenshot::prelude::*;

fn main() {
    // Capture entire screen
    //let buf = capture_display().unwrap();

    // Capture window by known id
    //let buf = capture_window(11996706).unwrap();

    // Capture window if you know the exact name
    //let hwnd = find_window("Notepad").unwrap();
    //let buf = capture_window(hwnd).unwrap();

    // If you don't know the exact name, try to find it
    let re = Regex::new(r"Firefox").unwrap();
    let hwnd = window_list()
        .unwrap()
        .iter()
        .find(|i| re.is_match(&i.window_name))
        .unwrap()
        .hwnd;

    // More complex func
    // Screenshot client area of window
    let area = Area::ClientOnly;
    // Build-in crop, faster on large window
    let crop = Some([100, 100, 300, 300]);
    let buf = capture_window_ex(hwnd, area, crop).unwrap();

    // convert to image and save
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    img.save("screenshot.jpg").unwrap();
}
