use clang_sys::clang_CXXMethod_isConst;
use clang_sys::clang_CXXMethod_isPureVirtual;
use clang_sys::clang_CXXMethod_isStatic;
use clang_sys::clang_CXXMethod_isVirtual;
use clang_sys::clang_getCursorKind;
use clang_sys::CXCursor_CXXMethod;

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
        unsafe {
            let cursor_kind = clang_getCursorKind(function.cursor);
            cursor_kind == CXCursor_CXXMethod
        }
    }
}
