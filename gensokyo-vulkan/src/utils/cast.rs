
use crate::types::vkchar;

use std::ffi::{ CStr, CString };

/// Helper function to convert [c_char; SIZE] to string
pub fn chars2string(raw_string_array: &[vkchar]) -> String {

    // Implementation 1
//    let end = '\0' as u8;
//    let mut content: Vec<u8> = vec![];
//
//    for ch in raw_string_array.iter() {
//        let ch = (*ch) as u8;
//
//        if ch != end {
//            content.push(ch);
//        } else {
//            break
//        }
//    }
//
//    String::from_utf8(content)
//        .expect("Failed to convert vulkan raw string to Rust String.")

    // Implementation 2
    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string.to_str()
        .expect("Failed to convert vulkan raw string to Rust String.")
        .to_owned()
}

pub fn chars2cstring(raw_string_array: &[vkchar]) -> CString {

    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string.to_owned()
}

pub fn string2cstring(content: Option<&String>) -> Option<CString> {
    content.and_then({
        |c| CString::new(c.clone()).ok()
    })
}

pub fn cstrings2ptrs(raw_string_array: &[CString]) -> Vec<*const vkchar> {

    raw_string_array.iter()
        .map(|l| l.as_ptr()).collect()
}
