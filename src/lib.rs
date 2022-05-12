// https://stackoverflow.com/questions/7292757/how-to-get-screenshot-of-a-window-as-bitmap-object-in-c
// https://stackoverflow.com/questions/37132196/multi-monitor-screenshots-only-2-monitors-in-c-with-winapi
// https://superkogito.github.io/blog/2020/09/28/loop_monitors_details_in_cplusplus.html

use std::ffi::OsString;
use image::imageops::flip_vertical;
use image::{ImageBuffer, Rgba};
use winapi::shared::windef::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use winapi::um::wingdi::SRCCOPY;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::ptr;
use winapi::um::winuser::GetSystemMetrics;
use winapi::shared::winerror::ERROR_INVALID_PARAMETER;

#[derive(Debug)]
pub enum WSError {
    GetDCIsNull,
    GetClientRectIsZero,
    CreateCompatibleDCIsNull,
    CreateCompatibleBitmapIsNull,
    SelectObjectError,
    PrintWindowIsZero,
    GetDIBitsError,
    GetSystemMetricsIsZero,
    StretchBltIsZero
}

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn find_window(window_name: &str) -> Result<usize, ()> {
    unsafe {
        let w = FindWindowW(ptr::null_mut(), OsString::from(window_name)
            .encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr()) as usize;
        match w {
            0 => Err(()),
            p => Ok(p)
        }
    }
}

pub fn capture_window(hwnd: usize) -> Result<Image, WSError> {
    unsafe {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        let rc: LPRECT = &mut rect;

        let hdc_screen = GetDC(hwnd as HWND);
        if hdc_screen == ptr::null_mut() {
            return Err(WSError::GetDCIsNull);
        }

        let get_cr = GetClientRect(hwnd as HWND, rc);
        if get_cr == 0 {
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::GetClientRectIsZero);
        }

        let width = (*rc).right - (*rc).left;
        let height = (*rc).bottom - (*rc).top;

        let hdc = CreateCompatibleDC(hdc_screen);
        if hdc == ptr::null_mut() {
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::CreateCompatibleDCIsNull);
        }

        let hbmp = CreateCompatibleBitmap(hdc_screen, width, height);
        if hbmp == ptr::null_mut() {
            DeleteDC(hdc);
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::CreateCompatibleBitmapIsNull);
        }

        let so = SelectObject(hdc, hbmp as HGDIOBJ);
        if so == HGDI_ERROR || so == ptr::null_mut() {
            DeleteDC(hdc);
            DeleteObject(hbmp as HGDIOBJ);
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::SelectObjectError);
        }

        let bmih = BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biPlanes: 1,
            biBitCount: 32,
            biWidth: width,
            biHeight: height,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };

        let mut bmi = BITMAPINFO {
            bmiHeader: bmih,
            ..Default::default()
        };

        let mut buf: Vec<u8> = vec![0; 4 * width as usize * height as usize];

        let pw = PrintWindow(hwnd as HWND, hdc, 0);
        if pw == 0 {
            DeleteDC(hdc);
            DeleteObject(hbmp as HGDIOBJ);
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::PrintWindowIsZero);
        }

        let gdb = GetDIBits(
            hdc,
            hbmp,
            0,
            height as u32,
            buf.as_mut_ptr() as *mut winapi::ctypes::c_void,
            &mut bmi,
            DIB_RGB_COLORS,
        );
        if gdb == 0 || gdb == ERROR_INVALID_PARAMETER as i32 {
            DeleteDC(hdc);
            DeleteObject(hbmp as HGDIOBJ);
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::GetDIBitsError);
        }

        buf.chunks_exact_mut(4).for_each(|c| {
            let t = c[0];
            c[0] = c[2];
            c[2] = t;
        });

        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width as u32, height as u32, buf).unwrap();

        DeleteDC(hdc);
        DeleteObject(hbmp as HGDIOBJ);
        ReleaseDC(null_mut(), hdc_screen);

        Ok(flip_vertical(&img))
    }
}

pub fn capture_display() -> Result<Image, WSError> {
    unsafe {
        let hdc_screen = GetDC(null_mut());
        if hdc_screen == ptr::null_mut() {
            return Err(WSError::GetDCIsNull);
        }

        let hdc = CreateCompatibleDC(hdc_screen);
        if hdc == ptr::null_mut() {
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::CreateCompatibleDCIsNull);
        }

        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);

        let hbmp = CreateCompatibleBitmap(hdc_screen, width, height);
        if hbmp == ptr::null_mut() {
            DeleteDC(hdc);
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::CreateCompatibleBitmapIsNull);
        }

        let so = SelectObject(hdc, hbmp as HGDIOBJ);
        if so == HGDI_ERROR || so == ptr::null_mut() {
            DeleteDC(hdc);
            DeleteObject(hbmp as HGDIOBJ);
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::SelectObjectError);
        }

        let sb = StretchBlt(hdc, 0, 0, width, height, hdc_screen, x, y, width, height, SRCCOPY);
        if sb == 0 {
            DeleteDC(hdc);
            DeleteObject(hbmp as HGDIOBJ);
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::StretchBltIsZero);
        }

        let bmih = BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biPlanes: 1,
            biBitCount: 32,
            biWidth: width,
            biHeight: height,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };

        let mut bmi = BITMAPINFO {
            bmiHeader: bmih,
            ..Default::default()
        };

        let mut buf: Vec<u8> = vec![0; 4 * width as usize * height as usize];

        let gdb = GetDIBits(
            hdc,
            hbmp,
            0,
            height as u32,
            buf.as_mut_ptr() as *mut winapi::ctypes::c_void,
            &mut bmi,
            DIB_RGB_COLORS,
        );
        if gdb == 0 || gdb == ERROR_INVALID_PARAMETER as i32 {
            DeleteDC(hdc);
            DeleteObject(hbmp as HGDIOBJ);
            ReleaseDC(null_mut(), hdc_screen);
            return Err(WSError::GetDIBitsError);
        }

        buf.chunks_exact_mut(4).for_each(|c| {
            let t = c[0];
            c[0] = c[2];
            c[2] = t;
        });

        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width as u32, height as u32, buf).unwrap();

        DeleteDC(hdc);
        DeleteDC(hdc_screen);
        ReleaseDC(null_mut(), hdc_screen);

        Ok(flip_vertical(&img))
    }
}

