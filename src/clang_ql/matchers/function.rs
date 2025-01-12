use clang_sys::clang_CXXConstructor_isConvertingConstructor;
use clang_sys::clang_CXXConstructor_isCopyConstructor;
use clang_sys::clang_CXXConstructor_isDefaultConstructor;
use clang_sys::clang_CXXConstructor_isMoveConstructor;
use clang_sys::clang_CXXMethod_isConst;
use clang_sys::clang_CXXMethod_isPureVirtual;
use clang_sys::clang_CXXMethod_isStatic;
use clang_sys::clang_CXXMethod_isVirtual;
use clang_sys::clang_getCXXAccessSpecifier;
use clang_sys::clang_getCursorKind;
use clang_sys::CXCursor_CXXMethod;
use clang_sys::CXCursor_Constructor;
use clang_sys::CXCursor_Destructor;
use clang_sys::CX_CXXAccessSpecifier;
use clang_sys::CX_CXXPrivate;
use clang_sys::CX_CXXProtected;
use clang_sys::CX_CXXPublic;

use crate::clang_ql::values::FunctionNode;

use super::FunctionMatcher;

#[derive(Clone)]
pub struct IsVirtualMatcher;

impl FunctionMatcher for IsVirtualMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isVirtual(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsPureVirtualMatcher;

impl FunctionMatcher for IsPureVirtualMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isPureVirtual(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsStaticMethodMatcher;

impl FunctionMatcher for IsStaticMethodMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isStatic(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsConstMethodMatcher;

impl FunctionMatcher for IsConstMethodMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isConst(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsDeletedMethodMatcher;

impl FunctionMatcher for IsDeletedMethodMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isConst(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsMethodMatcher;

impl FunctionMatcher for IsMethodMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCursorKind(function.cursor) == CXCursor_CXXMethod }
    }
}

#[derive(Clone)]
pub struct IsConstructorMatcher;

impl FunctionMatcher for IsConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCursorKind(function.cursor) == CXCursor_Constructor }
    }
}

#[derive(Clone)]
pub struct IsDefaultConstructorMatcher;

impl FunctionMatcher for IsDefaultConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXConstructor_isDefaultConstructor(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsCopyConstructorMatcher;

impl FunctionMatcher for IsCopyConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXConstructor_isCopyConstructor(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsMoveConstructorMatcher;

impl FunctionMatcher for IsMoveConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXConstructor_isMoveConstructor(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsConvertingConstructorMatcher;

impl FunctionMatcher for IsConvertingConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXConstructor_isConvertingConstructor(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsDestructorMatcher;

impl FunctionMatcher for IsDestructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCursorKind(function.cursor) == CXCursor_Destructor }
    }
}

#[derive(Clone)]
pub struct AccessSpecifierMatcher {
    access: CX_CXXAccessSpecifier,
}

impl AccessSpecifierMatcher {
    pub fn match_public() -> Self {
        AccessSpecifierMatcher {
            access: CX_CXXPublic,
        }
    }

    pub fn match_protected() -> Self {
        AccessSpecifierMatcher {
            access: CX_CXXProtected,
        }
    }

    pub fn match_private() -> Self {
        AccessSpecifierMatcher {
            access: CX_CXXPrivate,
        }
    }
}

impl FunctionMatcher for AccessSpecifierMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCXXAccessSpecifier(function.cursor) == self.access }
    }
}
