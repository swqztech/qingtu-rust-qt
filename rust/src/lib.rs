mod image_jobs;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;

/// 释放由本库返回的 C 字符串
#[no_mangle]
pub extern "C" fn qingtu_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(s));
    }
}

fn to_c_string(s: String) -> *mut c_char {
    CString::new(s).unwrap_or_default().into_raw()
}

fn paths_from_c(paths_ptr: *const *const c_char, count: usize) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if paths_ptr.is_null() || count == 0 {
        return out;
    }
    unsafe {
        let slice = std::slice::from_raw_parts(paths_ptr, count);
        for p in slice {
            if p.is_null() {
                continue;
            }
            if let Ok(s) = CStr::from_ptr(*p).to_str() {
                out.push(PathBuf::from(s));
            }
        }
    }
    out
}

/// format: 0 保持 / 1 JPEG / 2 PNG
#[no_mangle]
pub extern "C" fn qingtu_compress_files(
    paths_ptr: *const *const c_char,
    count: usize,
    quality: i32,
    max_side: i32,
    format: i32,
    custom_out: bool,
    out_dir: *const c_char,
) -> *mut c_char {
    let paths = paths_from_c(paths_ptr, count);
    let out = if custom_out && !out_dir.is_null() {
        unsafe {
            CStr::from_ptr(out_dir)
                .to_str()
                .map(PathBuf::from)
                .unwrap_or_default()
        }
    } else {
        PathBuf::new()
    };
    let log = image_jobs::run_compress(&paths, quality, max_side, format, custom_out, &out);
    to_c_string(log)
}

#[no_mangle]
pub extern "C" fn qingtu_stitch_files(
    paths_ptr: *const *const c_char,
    count: usize,
    custom_out: bool,
    out_dir: *const c_char,
) -> *mut c_char {
    let paths = paths_from_c(paths_ptr, count);
    let out = if custom_out && !out_dir.is_null() {
        unsafe {
            CStr::from_ptr(out_dir)
                .to_str()
                .map(PathBuf::from)
                .unwrap_or_default()
        }
    } else {
        PathBuf::new()
    };
    let log = image_jobs::run_stitch(&paths, custom_out, &out);
    to_c_string(log)
}
