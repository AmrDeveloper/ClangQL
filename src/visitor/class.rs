extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ptr;

use crate::visitor::location;

pub struct ClassNode {
    pub name: String,
    pub is_struct: bool,
    pub location: location::SourceLocation,
}

pub fn select_clang_classes(path: &str) -> Vec<ClassNode> {
    let mut classes: Vec<ClassNode> = Vec::new();
    let data = &mut classes as *mut Vec<ClassNode> as *mut c_void;

    unsafe {
        let index = clang_createIndex(0, 0);
        let translation_unit: CXTranslationUnit = clang_parseTranslationUnit(
            index,
            path.as_ptr() as *const c_char,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            0,
            0,
        );

        let cursor = clang_getTranslationUnitCursor(translation_unit);
        clang_visitChildren(cursor, visit_children, data);

        // Dispose the translation unit
        clang_disposeTranslationUnit(translation_unit);

        // Dispose the index
        clang_disposeIndex(index);
    }

    classes
}

extern "C" fn visit_children(
    cursor: CXCursor,
    _parent: CXCursor,
    data: *mut c_void,
) -> CXChildVisitResult {
    unsafe {
        if clang_Location_isFromMainFile(clang_getCursorLocation(cursor)) == 0 {
            return CXChildVisit_Continue;
        }

        let cursor_kind = clang_getCursorKind(cursor);
        if cursor_kind == CXCursor_ClassDecl || cursor_kind == CXCursor_StructDecl {
            let cursor_name = clang_getCursorSpelling(cursor);
            let class_name = CStr::from_ptr(clang_getCString(cursor_name)).to_string_lossy();

            let location = location::visit_source_location(cursor);

            let classes = &mut *(data as *mut Vec<ClassNode>);
            let is_struct = cursor_kind == CXCursor_StructDecl;

            classes.push(ClassNode {
                name: class_name.to_string(),
                is_struct,
                location,
            });

            clang_disposeString(cursor_name);
            return CXChildVisit_Continue;
        }
    }
    CXChildVisit_Recurse
}
