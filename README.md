# win-screenshot
Take a screenshot from specified window or entire screen on Windows platform

## Known Issues
capture_window() cannot correctly capture the hardware accelerated window

## Examples
```rust
use win_screenshot::*;

fn main() {
    // capture entire screen
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
```