# win-screenshot
Take a screenshot of a specific window or entire screen on Windows platform

## Known Issues
`capture_window()` draws black border for some windows  
If you call `capture_window()` and got `GetDCIsNull` make sure captured window is not minimized

## Minimum requirements
`capture_window()` uses undocumented `PW_RENDERFULLCONTENT` which first appeared in Windows 8.1

## Examples
```rust
use image::{ImageBuffer, Rgba};
use regex::Regex;
use win_screenshot::prelude::*;

fn main() {
    // capture entire screen
    let buf = capture_display().unwrap();

    // capture window by known id
    let buf = capture_window(11996706, Area::Full).unwrap();

    // capture window client area
    let buf = capture_window(11996706, Area::ClientOnly).unwrap();

    // capture window if you know the exact name
    let hwnd = find_window("Notepad").unwrap();
    let buf = capture_window(hwnd, Area::Full).unwrap();

    // if you don't know the exact name, try to find it
    let re = Regex::new(r"Firefox").unwrap();
    let hwnd = window_list()
        .unwrap()
        .iter()
        .find(|i| re.is_match(&i.window_name))
        .unwrap()
        .hwnd;
    let buf = capture_window(hwnd, Area::Full).unwrap();

    // convert to image and save
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    img.save("screenshot.jpg").unwrap();
}
```