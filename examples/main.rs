use std::time::Instant;

use image::{ImageBuffer, Rgba, Rgb};
use qshot::CaptureManager;
use regex::Regex;
use win_screenshot::prelude::*;

fn main() {
    // capture entire screen
    //let buf = capture_display().unwrap();

    // capture window by known id
    //let buf = capture_window(11996706, Area::Full).unwrap();

    // capture window client area
    //let buf = capture_window(11996706, Area::ClientOnly).unwrap();

    // capture window if you know the exact name
    //let hwnd = find_window("Notepad").unwrap();
    //let buf = capture_window(hwnd, Area::Full).unwrap();

    // if you don't know the exact name, try to find it
    let re = Regex::new(r"Firefox").unwrap();
    let hwnd = window_list()
        .unwrap()
        .iter()
        .find(|i| re.is_match(&i.window_name))
        .unwrap()
        .hwnd;

    //let buf = capture_window(hwnd, Area::Full).unwrap();
    let buf = capture_window_ex(hwnd, Area::Full, Some([100, 100, 200, 200])).unwrap();


    // convert to image and save
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    //let img = imageops::crop_imm(&img, 100, 100, 200, 200).to_image();
    img.save("screenshot.jpg").unwrap();


    /*let manager = CaptureManager::new(hwnd, (100, 100), (200, 200)).unwrap();
    let res = manager.capture().unwrap();
    let img: ImageBuffer<Rgb<u8>, &[u8]> =
        ImageBuffer::from_raw(200, 200, res.get_bits()).unwrap();
    img.save("screenshot.jpg").unwrap();*/
}
