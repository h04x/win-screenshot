use windows::{
    core::{Error, IntoParam},
    Win32::{
        Foundation::{HWND, RECT},
        Graphics::Gdi::{
            CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, ReleaseDC,
            HBITMAP, HDC,
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
            match GetDC(hwnd.into()) {
                e if e.is_invalid() => Err(Error::from_win32()),
                hdc => Ok(Hdc { hdc }),
            }
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

impl From<&Hdc> for HDC {
    fn from(item: &Hdc) -> Self {
        item.hdc
    }
}

impl From<Hdc> for HDC {
    fn from(item: Hdc) -> Self {
        item.hdc
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Rect {
    //pub(crate) rect: RECT,
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl From<RECT> for Rect {
    fn from(rect: RECT) -> Self {
        Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
            width: rect.right - rect.left,
            height: rect.bottom - rect.top,
        }
    }
}

impl Rect {
    pub(crate) fn get_window_rect<P0>(hwnd: P0) -> Result<Rect, Error>
    where
        P0: Into<HWND>,
    {
        let mut rect = RECT::default();
        unsafe {
            match GetWindowRect(hwnd.into(), &mut rect) {
                Ok(_) => Ok(Rect::from(rect)),
                Err(e) => Err(e),
            }
        }
    }
    pub(crate) fn get_client_rect<P0>(hwnd: P0) -> Result<Rect, Error>
    where
        P0: Into<HWND>,
    {
        let mut rect = RECT::default();
        unsafe {
            match GetClientRect(hwnd.into(), &mut rect) {
                Ok(_) => Ok(Rect::from(rect)),
                Err(e) => Err(e),
            }
        }
    }
}

pub(crate) struct CreatedHdc {
    pub(crate) hdc: HDC,
}

impl CreatedHdc {
    pub(crate) fn create_compatible_dc<P0>(hdc: P0) -> Result<CreatedHdc, Error>
    where
        P0: IntoParam<HDC>,
    {
        unsafe {
            match CreateCompatibleDC(hdc) {
                e if e.is_invalid() => Err(Error::from_win32()),
                hdc => Ok(CreatedHdc { hdc }),
            }
        }
    }
}

impl From<&CreatedHdc> for HDC {
    fn from(item: &CreatedHdc) -> Self {
        HDC(item.hdc.0)
    }
}

impl From<CreatedHdc> for HDC {
    fn from(item: CreatedHdc) -> Self {
        HDC(item.hdc.0)
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
        P0: IntoParam<HDC>,
    {
        unsafe {
            match CreateCompatibleBitmap(hdc, w, h) {
                e if e.is_invalid() => Err(Error::from_win32()),
                hbitmap => Ok(Hbitmap { hbitmap }),
            }
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
