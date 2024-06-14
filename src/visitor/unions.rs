extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ptr;

use crate::visitor::location;
pub struct UnionNode {
    pub name: String,
    pub size: i64,
    pub location: location::SourceLocation,
}

pub fn select_clang_unions(path: &str) -> Vec<UnionNode> {
    let mut unions: Vec<UnionNode> = Vec::new();
    let data = &mut unions as *mut Vec<UnionNode> as *mut c_void;

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
        clang_visitChildren(cursor, visit_union_declaration, data);

        // Dispose the translation unit
        clang_disposeTranslationUnit(translation_unit);

        // Dispose the index
        clang_disposeIndex(index);
    }

    unions
}

extern "C" fn visit_union_declaration(
    cursor: CXCursor,
    _parent: CXCursor,
    data: *mut c_void,
) -> CXChildVisitResult {
    unsafe {
        if clang_Location_isFromMainFile(clang_getCursorLocation(cursor)) == 0 {
            return CXChildVisit_Continue;
        }

        let cursor_kind = clang_getCursorKind(cursor);
        if cursor_kind == CXCursor_UnionDecl {
            let cursor_name = clang_getCursorSpelling(cursor);
            let union_name = CStr::from_ptr(clang_getCString(cursor_name)).to_string_lossy();
            let location = location::visit_source_location(cursor);

            let union_type = clang_getCursorType(cursor);
            let size = clang_Type_getSizeOf(union_type);

            let unions: &mut Vec<UnionNode> = &mut *(data as *mut Vec<UnionNode>);
            unions.push(UnionNode {
                name: union_name.to_string(),
                size,
                location,
            });

            clang_disposeString(cursor_name);
            return CXChildVisit_Continue;
        }
    }
    CXChildVisit_Recurse
}
