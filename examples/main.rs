use win_screenshot::*;

fn main() {
    // capture whole display
    capture_display().unwrap().save("screenshot.jpg").unwrap();

    // capture window by known id
    capture_window(67584)
        .unwrap()
        .save("screenshot.jpg")
        .unwrap();

    // capture window by Name
    capture_window(find_window("WindowName").unwrap())
        .unwrap()
        .save("screenshot.jpg")
        .unwrap();
}
