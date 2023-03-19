use windows::{
    core::Error,
    Win32::{
        Foundation::{HWND, RECT},
        Graphics::Gdi::{
            BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
            GetDIBits, ReleaseDC, SelectObject, StretchBlt, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
            DIB_RGB_COLORS, HDC, SRCCOPY, CreatedHDC, HBITMAP,
        },
        UI::WindowsAndMessaging::{GetClientRect, GetWindowRect},
    },
};

#[derive(Clone)]
pub(crate) struct Hdc {
    pub(crate) hdc: HDC,
}

impl Hdc {
    pub(crate) fn get_dc<P0>(hwnd: P0) -> Result<Hdc, Error>
    where
        P0: Into<HWND>,
    {
        unsafe {
            return match GetDC(hwnd) {
                e if e.is_invalid() => Err(Error::from_win32()),
                hdc => Ok(Hdc { hdc }),
            };
        }
    }
}

impl Drop for Hdc {
    fn drop(&mut self) {
        unsafe {
            ReleaseDC(HWND::default(), self.hdc);
        }
    }
}

impl From<Hdc> for HDC {
    fn from(item: Hdc) -> Self {
        item.hdc
    }
}

pub(crate) struct Rect {
    //pub(crate) rect: RECT,
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl Rect {
    pub(crate) fn get_window_rect<P0>(hwnd: P0) -> Result<Rect, Error>
    where
        P0: Into<HWND>,
    {
        let mut rect = RECT::default();
        unsafe {
            return match GetWindowRect(hwnd, &mut rect).as_bool() {
                true => Ok(Rect {
                    left: rect.left,
                    top: rect.top,
                    right: rect.right,
                    bottom: rect.bottom,
                    width: rect.right - rect.left,
                    height: rect.bottom - rect.top,
                }),
                false => Err(Error::from_win32()),
            };
        }
    }
    pub(crate) fn get_client_rect<P0>(hwnd: P0) -> Result<Rect, Error>
    where
        P0: Into<HWND>,
    {
        let mut rect = RECT::default();
        unsafe {
            return match GetClientRect(hwnd, &mut rect).as_bool() {
                true => Ok(Rect {
                    left: rect.left,
                    top: rect.top,
                    right: rect.right,
                    bottom: rect.bottom,
                    width: rect.right - rect.left,
                    height: rect.bottom - rect.top,
                }),
                false => Err(Error::from_win32()),
            };
        }
    }
}

pub(crate) struct CreatedHdc {
    pub(crate) hdc: CreatedHDC,
}

impl CreatedHdc {
    pub(crate) fn create_compatible_dc<P0>(hdc: P0) -> Result<CreatedHdc, Error>
    where
        P0: Into<HDC>
    {
        unsafe {
            return match CreateCompatibleDC(hdc.into()) {
                e if e.is_invalid() => Err(Error::from_win32()),
                hdc => Ok(CreatedHdc { hdc }),

            };
        }
    }
}

impl Drop for CreatedHdc {
    fn drop(&mut self) {
        unsafe {
            DeleteDC(self.hdc);
        }
    }
}

pub(crate) struct Hbitmap {
    pub(crate) hbitmap: HBITMAP,
}

impl Hbitmap {
    pub(crate) fn create_compatible_bitmap<P0>(hdc: P0, w: i32, h: i32) -> Result<Hbitmap, Error>
    where
        P0: Into<HDC>
    {
        unsafe {
            return match CreateCompatibleBitmap(hdc.into(), w, h) {
                e if e.is_invalid() => Err(Error::from_win32()),
                hbitmap => Ok(Hbitmap { hbitmap }),
            };
        }
    }
}

impl Drop for Hbitmap {
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.hbitmap);
        }
    }
}

impl From<Hbitmap> for HBITMAP {
    fn from(item: Hbitmap) -> Self {
        item.hbitmap
    }
}
