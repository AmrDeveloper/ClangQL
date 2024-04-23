extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ptr;

pub struct GlobalVariableNode {
    pub name: String,
    pub type_literal: String,
}

pub fn select_clang_variables(path: &str) -> Vec<GlobalVariableNode> {
    let mut functions: Vec<GlobalVariableNode> = Vec::new();
    let data = &mut functions as *mut Vec<GlobalVariableNode> as *mut c_void;

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

    functions
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

        let kind = clang_getCursorKind(cursor);
        if kind == CXCursor_VarDecl {
            let variables = &mut *(data as *mut Vec<GlobalVariableNode>);

            let field_name = clang_getCursorSpelling(cursor);
            let field_name_str = CStr::from_ptr(clang_getCString(field_name)).to_string_lossy();

            let field_type = clang_getCursorType(cursor);
            let field_type_str =
                CStr::from_ptr(clang_getCString(clang_getTypeSpelling(field_type)))
                    .to_string_lossy();

            variables.push(GlobalVariableNode {
                name: field_name_str.to_string(),
                type_literal: field_type_str.to_string(),
            });

            clang_disposeString(field_name);
        }
    }
    CXChildVisit_Continue
}
