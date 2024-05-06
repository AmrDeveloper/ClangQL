extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ptr;

use crate::visitor::location;

pub struct EnumNode {
    pub name: String,
    pub type_literal: String,
    pub attributes: EnumAttributes,
    pub location: location::SourceLocation,
}

#[derive(Default)]
pub struct EnumAttributes {
    pub constants_count: u32,
}

pub fn select_clang_enums(path: &str) -> Vec<EnumNode> {
    let mut enums: Vec<EnumNode> = Vec::new();
    let data = &mut enums as *mut Vec<EnumNode> as *mut c_void;

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
        clang_visitChildren(cursor, visit_enum_declaration, data);

        // Dispose the translation unit
        clang_disposeTranslationUnit(translation_unit);

        // Dispose the index
        clang_disposeIndex(index);
    }

    enums
}

extern "C" fn visit_enum_declaration(
    cursor: CXCursor,
    _parent: CXCursor,
    data: *mut c_void,
) -> CXChildVisitResult {
    unsafe {
        if clang_Location_isFromMainFile(clang_getCursorLocation(cursor)) == 0 {
            return CXChildVisit_Continue;
        }

        let cursor_kind = clang_getCursorKind(cursor);
        if cursor_kind == CXCursor_EnumDecl {
            let cursor_name = clang_getCursorSpelling(cursor);
            let enum_name = CStr::from_ptr(clang_getCString(cursor_name)).to_string_lossy();

            let location = location::visit_source_location(cursor);

            let enum_type = clang_getEnumDeclIntegerType(cursor);
            let enum_type_spelling = clang_getTypeSpelling(enum_type);
            let type_literal =
                CStr::from_ptr(clang_getCString(enum_type_spelling)).to_string_lossy();

            let mut attributes = EnumAttributes::default();
            let attributes_pointer = &mut attributes as *mut EnumAttributes as *mut c_void;
            clang_visitChildren(cursor, visit_enum_attributes, attributes_pointer);

            let enums: &mut Vec<EnumNode> = &mut *(data as *mut Vec<EnumNode>);
            enums.push(EnumNode {
                name: enum_name.to_string(),
                type_literal: type_literal.to_string(),
                attributes,
                location,
            });

            clang_disposeString(cursor_name);
            clang_disposeString(enum_type_spelling);
            return CXChildVisit_Continue;
        }
    }
    CXChildVisit_Recurse
}

extern "C" fn visit_enum_attributes(
    cursor: CXCursor,
    _parent: CXCursor,
    data: *mut c_void,
) -> CXChildVisitResult {
    unsafe {
        if clang_Location_isFromMainFile(clang_getCursorLocation(cursor)) == 0 {
            return CXChildVisit_Continue;
        }

        let cursor_kind = clang_getCursorKind(cursor);

        if cursor_kind == CXCursor_EnumConstantDecl {
            let attributes = &mut *(data as *mut EnumAttributes);
            attributes.constants_count += 1;
            return CXChildVisit_Continue;
        }
    }
    CXChildVisit_Recurse
}
