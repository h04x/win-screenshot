use win_screenshot::*;

fn main() {
    // capture entire screen
    capture_display().unwrap().save("screenshot.jpg").unwrap();

    // capture window by known id
    capture_window(26936680)
        .unwrap()
        .save("screenshot.jpg")
        .unwrap();

    // capture window by Name
    capture_window(find_window("WindowName").unwrap())
        .unwrap()
        .save("screenshot.jpg")
        .unwrap();
}
