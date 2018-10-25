
use ash::vk::uint32_t;

use config::error::ConfigError;

use std::ffi::{ CStr, CString };
use std::os::raw::c_char;

pub fn vk_to_version(raw_version: &String) -> Result<uint32_t, ConfigError> {

    let versions = raw_version.split('.').collect::<Vec<_>>();

    if versions.len() == 3 {

        let major = versions[0].parse::<uint32_t>().map_err(|_| ConfigError::ParseError)?;
        let minor = versions[1].parse::<uint32_t>().map_err(|_| ConfigError::ParseError)?;
        let patch = versions[2].parse::<uint32_t>().map_err(|_| ConfigError::ParseError)?;

        let verion = vk_make_version!(major, minor, patch);

        Ok(verion)
    } else {
        return Err(ConfigError::ParseError)
    }
}

/// Helper function to convert [c_char; SIZE] to string
pub fn vk_to_string(raw_string_array: &[c_char]) -> String {

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
pub fn vk_to_cstring(raw_string_array: &[c_char]) -> CString {
    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string.to_owned()
}

pub fn to_array_ptr(raw_string_array: &[CString]) -> Vec<*const c_char> {
    raw_string_array.iter().map(|l| l.as_ptr()).collect()
}
