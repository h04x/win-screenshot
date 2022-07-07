# win-screenshot
Take a screenshot from specified window or entire screen on Windows platform

## Known Issues
capture_window() draws black border for some windows

## Minimum requirements
capture_window() uses undocumented PW_RENDERFULLCONTENT which first appeared in Windows 8.1

## Examples
```rust
use regex::Regex;
use win_screenshot::addon::*;
use win_screenshot::capture::*;

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
    let s = capture_window(hwnd, Area::Full)
        .unwrap();

    s.save("screenshot.jpg").unwrap();
}

```