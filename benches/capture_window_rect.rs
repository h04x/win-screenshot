use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{imageops::crop_imm, ImageBuffer, Rgba};
use regex::Regex;
use win_screenshot::prelude::*;

fn using_image_crate(hwnd: isize) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let buf = capture_window(hwnd, Area::Full).unwrap();
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    let img = crop_imm(&img, 100, 100, 200, 200).to_image();
    img
}

fn using_capture_window_ex(hwnd: isize) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let buf = capture_window_ex(hwnd, Area::Full, Some([100, 100, 200, 200])).unwrap();
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    img
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let re = Regex::new(r"Sublime").unwrap();
    let hwnd = window_list()
        .unwrap()
        .iter()
        .find(|i| re.is_match(&i.window_name))
        .unwrap()
        .hwnd;

    let mut group = c.benchmark_group("crop");

    group.bench_function("using_image_crate", |b| {
        b.iter(|| using_image_crate(black_box(hwnd)))
    });
    group.bench_function("using_capture_window_ex", |b| {
        b.iter(|| using_capture_window_ex(black_box(hwnd)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
