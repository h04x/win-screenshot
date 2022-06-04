use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::*;

#[derive(Debug)]
pub struct HwndName {
    pub hwnd: usize,
    pub window_name: String,
}

#[derive(Debug)]
pub enum FWError {
    NotFoundOrFault,
}

pub fn find_window(window_name: &str) -> Result<usize, FWError> {
    unsafe {
        let w = FindWindowW(
            ptr::null_mut(),
            OsString::from(window_name)
                .encode_wide()
                .chain(Some(0))
                .collect::<Vec<_>>()
                .as_ptr(),
        ) as usize;
        match w {
            0 => Err(FWError::NotFoundOrFault),
            p => Ok(p),
        }
    }
}

unsafe extern "system" fn wl_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let vec = lparam as *mut Vec<HwndName>;
    const CHAR_LIM: i32 = 128;

    if IsWindowVisible(hwnd) == FALSE {
        return TRUE;
    }

    // as GetWindowTextW return UTF-16 string, which can contain 4 byte per char
    // allocate buffer ((char_count+1) * 4) bytes, to avoid potentially buf overflow
    let name_buf: Vec<u16> = vec![0; ((CHAR_LIM + 1) * 2) as usize];

    let gwt = GetWindowTextW(hwnd, name_buf.as_ptr() as LPWSTR, CHAR_LIM);
    if gwt == 0 {
        return TRUE;
    }

    let name = String::from_utf16_lossy(&name_buf)
        .trim_matches(char::from(0))
        .to_string();

    (*vec).push(HwndName {
        hwnd: hwnd as usize,
        window_name: name,
    });

    TRUE
}

#[derive(Debug)]
pub enum WLError {
    EnumWindowsError,
}

pub fn window_list() -> Result<Vec<HwndName>, WLError> {
    let mut hwnd_name = Vec::new();
    unsafe {
        let ew = EnumWindows(
            Some(wl_callback),
            &mut hwnd_name as *mut Vec<HwndName> as LPARAM,
        );
        if ew == 0 {
            return Err(WLError::EnumWindowsError);
        }
    }
    Ok(hwnd_name)
}
