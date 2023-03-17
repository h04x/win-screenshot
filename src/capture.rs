use std::mem::size_of;
use std::time::Instant;
use windows::Win32::Foundation::{ERROR_INVALID_PARAMETER, HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
    ReleaseDC, SelectObject, StretchBlt, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    SRCCOPY,
};
use windows::Win32::Storage::Xps::{PrintWindow, PRINT_WINDOW_FLAGS, PW_CLIENTONLY};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClientRect, GetSystemMetrics, GetWindowRect, PW_RENDERFULLCONTENT, SM_CXVIRTUALSCREEN,
    SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
};

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
    StretchBltIsZero,
    BitBltError,
}

pub enum Area {
    Full,
    ClientOnly,
}

#[derive(Debug)]
pub struct RgbBuf {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn capture_window(hwnd: isize) -> Result<RgbBuf, WSError> {
    capture_window_ex(hwnd, Area::Full, None)
}

pub fn capture_window_ex(
    hwnd: isize,
    area: Area,
    user_rect: Option<[i32; 4]>,
) -> Result<RgbBuf, WSError> {
    let hwnd = HWND(hwnd);

    //let [x, y, w, h] = rect.unwrap();

    unsafe {
        let mut rect = RECT::default();

        let hdc_screen = GetDC(hwnd);
        if hdc_screen.is_invalid() {
            return Err(WSError::GetDCIsNull);
        }

        let get_cr = match area {
            Area::Full => GetWindowRect(hwnd, &mut rect),
            Area::ClientOnly => GetClientRect(hwnd, &mut rect),
        };
        if get_cr == false {
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::GetClientRectIsZero);
        }

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        let hdc = CreateCompatibleDC(hdc_screen);
        if hdc.is_invalid() {
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::CreateCompatibleDCIsNull);
        }

        let hbmp = CreateCompatibleBitmap(hdc_screen, width, height);
        if hbmp.is_invalid() {
            DeleteDC(hdc);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::CreateCompatibleBitmapIsNull);
        }

        let so = SelectObject(hdc, hbmp);
        if so.is_invalid() {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::SelectObjectError);
        }

        let flags = match area {
            Area::Full => PRINT_WINDOW_FLAGS(PW_RENDERFULLCONTENT),
            Area::ClientOnly => PRINT_WINDOW_FLAGS(PW_CLIENTONLY.0 | PW_RENDERFULLCONTENT),
        };

        let pw = PrintWindow(hwnd, hdc, flags);
        if pw == false {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::PrintWindowIsZero);
        }

        let (w, h, hdc, hbmp) = match user_rect {
            Some(rect) => {
                let [x, y, w, h] = rect;
                let hdc2 = CreateCompatibleDC(hdc);
                if hdc2.is_invalid() {
                    ReleaseDC(HWND::default(), hdc_screen);
                    return Err(WSError::CreateCompatibleDCIsNull);
                }

                let hbmp2 = CreateCompatibleBitmap(hdc, w, h);
                if hbmp2.is_invalid() {
                    DeleteDC(hdc);
                    DeleteDC(hdc2);
                    ReleaseDC(HWND::default(), hdc_screen);
                    return Err(WSError::CreateCompatibleBitmapIsNull);
                }

                let so = SelectObject(hdc2, hbmp2);
                if so.is_invalid() {
                    DeleteDC(hdc);
                    DeleteDC(hdc2);
                    DeleteObject(hbmp);
                    DeleteObject(hbmp2);
                    ReleaseDC(HWND::default(), hdc_screen);
                    return Err(WSError::SelectObjectError);
                }

                let bb = BitBlt(hdc2, 0, 0, w, h, hdc, x, y, SRCCOPY);
                if bb == false {
                    DeleteDC(hdc);
                    DeleteDC(hdc2);
                    DeleteObject(hbmp);
                    DeleteObject(hbmp2);
                    ReleaseDC(HWND::default(), hdc_screen);
                    return Err(WSError::BitBltError);
                }

                if SelectObject(hdc2, so).is_invalid() {
                    DeleteDC(hdc);
                    DeleteDC(hdc2);
                    DeleteObject(hbmp);
                    DeleteObject(hbmp2);
                    ReleaseDC(HWND::default(), hdc_screen);
                    return Err(WSError::SelectObjectError);
                }
                DeleteDC(hdc);
                DeleteObject(hbmp);
                (w, h, hdc2, hbmp2)
            }
            None => (width, height, hdc, hbmp),
        };

        let bmih = BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biPlanes: 1,
            biBitCount: 24,
            biWidth: w,
            biHeight: -h,
            biCompression: BI_RGB,
            ..Default::default()
        };

        let mut bmi = BITMAPINFO {
            bmiHeader: bmih,
            ..Default::default()
        };

        let mut buf: Vec<u8> = vec![0; (3 * w * h) as usize];

        let gdb = GetDIBits(
            hdc,
            hbmp,
            0,
            h as u32,
            Some(buf.as_mut_ptr() as *mut core::ffi::c_void),
            &mut bmi,
            DIB_RGB_COLORS,
        );
        if gdb == 0 || gdb == ERROR_INVALID_PARAMETER.0 as i32 {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::GetDIBitsError);
        }

        buf.chunks_exact_mut(3).for_each(|c| c.swap(0, 2));

        DeleteDC(hdc);
        DeleteObject(hbmp);
        ReleaseDC(HWND::default(), hdc_screen);

        Ok(RgbBuf {
            pixels: buf,
            width: w as u32,
            height: h as u32,
        })
    }
}

pub fn capture_display() -> Result<RgbBuf, WSError> {
    unsafe {
        let hdc_screen = GetDC(HWND::default());
        if hdc_screen.is_invalid() {
            return Err(WSError::GetDCIsNull);
        }

        let hdc = CreateCompatibleDC(hdc_screen);
        if hdc.is_invalid() {
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::CreateCompatibleDCIsNull);
        }

        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);

        let hbmp = CreateCompatibleBitmap(hdc_screen, width, height);
        if hbmp.is_invalid() {
            DeleteDC(hdc);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::CreateCompatibleBitmapIsNull);
        }

        let so = SelectObject(hdc, hbmp);
        if so.is_invalid() {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::SelectObjectError);
        }

        let sb = StretchBlt(
            hdc, 0, 0, width, height, hdc_screen, x, y, width, height, SRCCOPY,
        );
        if sb == false {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::StretchBltIsZero);
        }

        let bmih = BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biPlanes: 1,
            biBitCount: 24,
            biWidth: width,
            biHeight: -height,
            biCompression: BI_RGB,
            ..Default::default()
        };

        let mut bmi = BITMAPINFO {
            bmiHeader: bmih,
            ..Default::default()
        };

        let mut buf: Vec<u8> = vec![0; (4 * width * height) as usize];

        let gdb = GetDIBits(
            hdc,
            hbmp,
            0,
            height as u32,
            Some(buf.as_mut_ptr() as *mut core::ffi::c_void),
            &mut bmi,
            DIB_RGB_COLORS,
        );
        if gdb == 0 || gdb == ERROR_INVALID_PARAMETER.0 as i32 {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err(WSError::GetDIBitsError);
        }

        buf.chunks_exact_mut(3).for_each(|c| c.swap(0, 2));

        DeleteDC(hdc);
        DeleteObject(hbmp);
        ReleaseDC(HWND::default(), hdc_screen);

        Ok(RgbBuf {
            pixels: buf,
            width: width as u32,
            height: height as u32,
        })
    }
}
