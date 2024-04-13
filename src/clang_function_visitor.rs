extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ptr;

pub struct FunctionNode {
    pub name: String,
    pub signature: String,
    pub return_type: String,
    pub arguments_count: i32,
    pub class_name: String,
    pub is_method: bool,
    pub is_virtual: bool,
    pub is_pure_virtual: bool,
    pub has_template: bool,
}

pub fn select_clang_functions(path: &str) -> Vec<FunctionNode> {
    let mut functions: Vec<FunctionNode> = Vec::new();
    let data = &mut functions as *mut Vec<FunctionNode> as *mut c_void;

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
    parent: CXCursor,
    data: *mut c_void,
) -> CXChildVisitResult {
    unsafe {
        if clang_Location_isFromMainFile(clang_getCursorLocation(cursor)) == 0 {
            return CXChildVisit_Continue;
        }

        let cursor_kind = clang_getCursorKind(cursor);
        if cursor_kind == CXCursor_FunctionDecl
            || cursor_kind == CXCursor_CXXMethod
            || cursor_kind == CXCursor_FunctionTemplate
        {
            let functions = &mut *(data as *mut Vec<FunctionNode>);

            let cursor_name = clang_getCursorSpelling(cursor);
            let name = CStr::from_ptr(clang_getCString(cursor_name)).to_string_lossy();

            let function_type = clang_getCursorType(cursor);

            let function_signature = clang_getTypeSpelling(function_type);
            let signature = CStr::from_ptr(clang_getCString(function_signature)).to_string_lossy();

            let result_type = clang_getResultType(function_type);
            let result_type_spelling = clang_getTypeSpelling(result_type);
            let return_type =
                CStr::from_ptr(clang_getCString(result_type_spelling)).to_string_lossy();

            let arguments_count = clang_getNumArgTypes(function_type);

            let is_method = cursor_kind == CXCursor_CXXMethod;
            let has_template = cursor_kind == CXCursor_FunctionTemplate;

            let class_name = if is_method {
                let parent_spelling = clang_getCursorSpelling(parent);
                let parent_name =
                    CStr::from_ptr(clang_getCString(parent_spelling)).to_string_lossy();
                parent_name.to_string()
            } else {
                "None".to_string()
            };

            let is_virtual = if is_method {
                clang_CXXMethod_isVirtual(cursor) != 0
            } else {
                false
            };

            let is_pure_virtual = if is_method {
                clang_CXXMethod_isPureVirtual(cursor) != 0
            } else {
                false
            };

            functions.push(FunctionNode {
                name: name.to_string(),
                signature: signature.to_string(),
                return_type: return_type.to_string(),
                arguments_count,
                class_name,
                is_method,
                is_virtual,
                is_pure_virtual,
                has_template,
            });

            clang_disposeString(cursor_name);
            clang_disposeString(function_signature);
            clang_disposeString(result_type_spelling);
        }
    }
    CXChildVisit_Recurse
}
