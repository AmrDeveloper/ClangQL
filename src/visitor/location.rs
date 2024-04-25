extern crate clang_sys;

use clang_sys::*;
use core::ffi::CStr;

pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

pub fn visit_source_location(cursor: CXCursor) -> SourceLocation {
    unsafe {
        let cursor_location = clang_getCursorLocation(cursor);

        let mut file: CXFile = std::ptr::null_mut();
        let mut line: u32 = 0;
        let mut column: u32 = 0;
        let mut offset: u32 = 0;

        clang_getFileLocation(
            cursor_location,
            &mut file,
            &mut line,
            &mut column,
            &mut offset,
        );

        let file_name = clang_getFileName(file);
        let file_name_str = CStr::from_ptr(clang_getCString(file_name)).to_string_lossy();

        clang_disposeString(file_name);

        SourceLocation {
            file: file_name_str.to_string(),
            line,
            column,
            offset,
        }
    }
}
