extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_void;
use std::ffi::CStr;

use crate::clang_ql::clang_parser::CompilationUnit;
use crate::clang_ql::values::FunctionNode;
use crate::clang_ql::visitors::location;

pub fn select_clang_functions(compilation_unit: &CompilationUnit) -> Vec<FunctionNode> {
    let mut functions: Vec<FunctionNode> = Vec::new();
    let data = &mut functions as *mut Vec<FunctionNode> as *mut c_void;

    unsafe {
        let cursor = clang_getTranslationUnitCursor(compilation_unit.translation_unit);
        clang_visitChildren(cursor, visit_children, data);
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
            || cursor_kind == CXCursor_Constructor
            || cursor_kind == CXCursor_Destructor
            || cursor_kind == CXCursor_ConversionFunction
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

            let location = location::visit_source_location(cursor);

            functions.push(FunctionNode {
                name: name.to_string(),
                cursor,
                parent,
                signature: signature.to_string(),
                return_type: return_type.to_string(),
                location,
            });

            clang_disposeString(cursor_name);
            clang_disposeString(function_signature);
            clang_disposeString(result_type_spelling);
            return CXChildVisit_Continue;
        }
    }
    CXChildVisit_Recurse
}
