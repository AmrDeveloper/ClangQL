extern crate clang_sys;

use clang_sys::*;
use std::ffi::c_void;
use std::ffi::CStr;

use crate::clang_ql::clang_parser::CompilationUnit;
use crate::clang_ql::values::FileLocation;
use crate::clang_ql::visitors::location;

pub struct ClassNode {
    pub name: String,
    pub attributes: ClassAttributes,
    pub is_struct: bool,
    pub size: i64,
    pub align: i64,
    pub location: FileLocation,
}

#[derive(Default)]
pub struct ClassAttributes {
    pub bases_count: u32,
    pub methods_count: u32,
    pub fields_count: u32,
}

pub fn select_clang_classes(compilation_unit: &CompilationUnit) -> Vec<ClassNode> {
    let mut classes: Vec<ClassNode> = Vec::new();
    let data = &mut classes as *mut Vec<ClassNode> as *mut c_void;

    unsafe {
        clang_visitChildren(
            compilation_unit.cursor,
            visit_class_or_struct_declaration,
            data,
        );
    }

    classes
}

extern "C" fn visit_class_or_struct_declaration(
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

            let class_type = clang_getCursorType(cursor);
            let size = clang_Type_getSizeOf(class_type);
            let align = clang_Type_getAlignOf(class_type);

            let mut attributes = ClassAttributes::default();
            let attributes_pointer = &mut attributes as *mut ClassAttributes as *mut c_void;
            clang_visitChildren(cursor, visit_class_attributes, attributes_pointer);

            classes.push(ClassNode {
                name: class_name.to_string(),
                attributes,
                is_struct,
                size,
                align,
                location,
            });

            clang_disposeString(cursor_name);
            return CXChildVisit_Continue;
        }
    }
    CXChildVisit_Recurse
}

extern "C" fn visit_class_attributes(
    cursor: CXCursor,
    _parent: CXCursor,
    data: *mut c_void,
) -> CXChildVisitResult {
    unsafe {
        if clang_Location_isFromMainFile(clang_getCursorLocation(cursor)) == 0 {
            return CXChildVisit_Continue;
        }

        let cursor_kind = clang_getCursorKind(cursor);

        if cursor_kind == CXCursor_CXXBaseSpecifier {
            let attributes = &mut *(data as *mut ClassAttributes);
            attributes.bases_count += 1;
            return CXChildVisit_Continue;
        }

        if cursor_kind == CXCursor_CXXMethod || cursor_kind == CXCursor_FunctionTemplate {
            let attributes = &mut *(data as *mut ClassAttributes);
            attributes.methods_count += 1;
            return CXChildVisit_Continue;
        }

        if cursor_kind == CXCursor_FieldDecl {
            let attributes = &mut *(data as *mut ClassAttributes);
            attributes.fields_count += 1;
            return CXChildVisit_Continue;
        }
    }
    CXChildVisit_Recurse
}
