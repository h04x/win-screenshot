use regex::Regex;
use win_screenshot::prelude::*;

fn main() {
    // capture entire screen
    let s = capture_display().unwrap();

    // capture window by known id
    let s = capture_window(11996706, Area::Full).unwrap();

    // capture window client area
    let s = capture_window(11996706, Area::ClientOnly).unwrap();

    // capture window if you know the exact name
    let hwnd = find_window("Notepad").unwrap();
    let s = capture_window(hwnd, Area::Full).unwrap();

    // if you don't know the exact name, try to find it
    let re = Regex::new(r"Firefox").unwrap();
    let hwnd = window_list()
        .unwrap()
        .iter()
        .find(|i| re.is_match(&i.window_name))
        .unwrap()
        .hwnd;
    let s = capture_window(hwnd, Area::Full).unwrap();

    s.save("screenshot.jpg").unwrap();
}
