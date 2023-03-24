use core::fmt;

use image::RgbaImage;
use regex::Regex;
use crate::prelude::*;

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Area::Full =>  write!(f, "f"),
            Area::ClientOnly => write!(f, "co")
        }
    }
}



fn cutr(hwnd: isize, area: Area, crop_xy: Option<[i32; 2]>, crop_wh: Option<[i32; 2]>) {
    let name = format!("{}-{:?}-{:?}", area, crop_xy, crop_wh);
    let b = capture_window_ex(hwnd, Using::PrintWindow, area, crop_xy, crop_wh).unwrap();
    RgbaImage::from_raw(b.width, b.height, b.pixels).unwrap().save(format!("tests_output/{}-pw.jpg", name)).unwrap();
    let b = capture_window_ex(hwnd, Using::BitBlt, area, crop_xy, crop_wh).unwrap();
    RgbaImage::from_raw(b.width, b.height, b.pixels).unwrap().save(format!("tests_output/{}-bb.jpg", name)).unwrap();
}

#[test]
fn enumerate_params() {
    let re = Regex::new(r"Sublime").unwrap();
    let hwnd = window_list()
        .unwrap()
        .iter()
        .find(|i| re.is_match(&i.window_name))
        .unwrap()
        .hwnd;

    std::fs::remove_dir_all("tests_output");
    std::fs::create_dir("tests_output").unwrap();

    cutr(hwnd, Area::ClientOnly, None, None);
    cutr(hwnd, Area::Full, None, None);

    cutr(hwnd, Area::ClientOnly, Some([100, 100]), None);
    cutr(hwnd, Area::ClientOnly, None, Some([100, 100]));
    cutr(hwnd, Area::ClientOnly, Some([100, 100]), Some([100, 100]));

    cutr(hwnd, Area::Full, Some([100, 100]), None);
    cutr(hwnd, Area::Full, None, Some([100, 100]));
    cutr(hwnd, Area::Full, Some([100, 100]), Some([100, 100]));
}
