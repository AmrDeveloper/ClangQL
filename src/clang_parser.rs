use std::ffi::CString;
use std::ptr;

use clang_sys::clang_createIndex;
use clang_sys::clang_parseTranslationUnit;
use clang_sys::CXIndex;
use clang_sys::CXTranslationUnit;

pub struct CompilationUnit {
    #[allow(dead_code)]
    pub path: String,

    pub index: CXIndex,
    pub translation_unit: CXTranslationUnit,
}

pub fn parse_files(files: &[String]) -> Vec<CompilationUnit> {
    let mut translation_units: Vec<CompilationUnit> = vec![];

    for file in files {
        unsafe {
            let fname: CString = CString::new(file.as_str()).unwrap();
            let index: CXIndex = clang_createIndex(0, 0);
            let translation_unit: CXTranslationUnit = clang_parseTranslationUnit(
                index,
                fname.as_ptr(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                0,
                0,
            );

            if translation_unit.is_null() {
                continue;
            }

            let compilation_unit = CompilationUnit {
                path: file.to_string(),
                index,
                translation_unit,
            };

            translation_units.push(compilation_unit);
        }
    }

    translation_units
}
