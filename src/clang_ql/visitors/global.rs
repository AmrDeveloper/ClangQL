extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_void;
use std::ffi::CStr;

use crate::clang_ql::clang_parser::CompilationUnit;
use crate::clang_ql::values::FileLocation;

use super::location;

pub struct GlobalVariableNode {
    pub name: String,
    pub type_literal: String,
    pub is_volatile: bool,
    pub location: FileLocation,
}

pub fn select_clang_variables(compilation_unit: &CompilationUnit) -> Vec<GlobalVariableNode> {
    let mut functions: Vec<GlobalVariableNode> = Vec::new();
    let data = &mut functions as *mut Vec<GlobalVariableNode> as *mut c_void;

    unsafe {
        clang_visitChildren(compilation_unit.cursor, visit_children, data);
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

            let is_volatile = clang_isVolatileQualifiedType(field_type) != 0;
            let location = location::visit_source_location(cursor);
            variables.push(GlobalVariableNode {
                name: field_name_str.to_string(),
                type_literal: field_type_str.to_string(),
                is_volatile,
                location,
            });

            clang_disposeString(field_name);
        }
    }
    CXChildVisit_Continue
}
