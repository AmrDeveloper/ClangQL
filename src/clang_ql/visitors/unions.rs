extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_void;
use std::ffi::CStr;

use crate::clang_ql::clang_parser::CompilationUnit;
use crate::clang_ql::values::FileLocation;
use crate::clang_ql::visitors::location;

pub struct UnionNode {
    pub name: String,
    pub attributes: UnionAttributes,
    pub size: i64,
    pub location: FileLocation,
}

#[derive(Default)]
pub struct UnionAttributes {
    pub fields_count: u32,
}

pub fn select_clang_unions(compilation_unit: &CompilationUnit) -> Vec<UnionNode> {
    let mut unions: Vec<UnionNode> = Vec::new();
    let data = &mut unions as *mut Vec<UnionNode> as *mut c_void;

    unsafe {
        clang_visitChildren(compilation_unit.cursor, visit_union_declaration, data);
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

            let mut attributes = UnionAttributes::default();
            let attributes_pointer = &mut attributes as *mut UnionAttributes as *mut c_void;
            clang_visitChildren(cursor, visit_union_attributes, attributes_pointer);

            let unions: &mut Vec<UnionNode> = &mut *(data as *mut Vec<UnionNode>);
            unions.push(UnionNode {
                name: union_name.to_string(),
                attributes,
                size,
                location,
            });

            clang_disposeString(cursor_name);
            return CXChildVisit_Continue;
        }
    }
    CXChildVisit_Recurse
}

extern "C" fn visit_union_attributes(
    cursor: CXCursor,
    _parent: CXCursor,
    data: *mut c_void,
) -> CXChildVisitResult {
    unsafe {
        if clang_Location_isFromMainFile(clang_getCursorLocation(cursor)) == 0 {
            return CXChildVisit_Continue;
        }

        let cursor_kind = clang_getCursorKind(cursor);
        if cursor_kind == CXCursor_FieldDecl {
            let attributes = &mut *(data as *mut UnionAttributes);
            attributes.fields_count += 1;
            return CXChildVisit_Continue;
        }
    }

    CXChildVisit_Continue
}
