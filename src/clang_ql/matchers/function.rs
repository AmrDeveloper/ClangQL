use clang_sys::clang_CXXConstructor_isConvertingConstructor;
use clang_sys::clang_CXXConstructor_isCopyConstructor;
use clang_sys::clang_CXXConstructor_isDefaultConstructor;
use clang_sys::clang_CXXConstructor_isMoveConstructor;
use clang_sys::clang_CXXMethod_isConst;
use clang_sys::clang_CXXMethod_isPureVirtual;
use clang_sys::clang_CXXMethod_isStatic;
use clang_sys::clang_CXXMethod_isVirtual;
use clang_sys::clang_Cursor_isFunctionInlined;
use clang_sys::clang_getCXXAccessSpecifier;
use clang_sys::clang_getCursorKind;
use clang_sys::clang_isCursorDefinition;
use clang_sys::clang_isDeclaration;
use clang_sys::CXCursor_CXXMethod;
use clang_sys::CXCursor_Constructor;
use clang_sys::CXCursor_ConversionFunction;
use clang_sys::CXCursor_Destructor;
use clang_sys::CXCursor_FunctionTemplate;
use clang_sys::CX_CXXAccessSpecifier;
use clang_sys::CX_CXXPrivate;
use clang_sys::CX_CXXProtected;
use clang_sys::CX_CXXPublic;

use crate::clang_ql::values::FunctionNode;

use super::Matcher;

#[derive(Clone)]
pub struct IsFunctionDeclaration;

impl Matcher<FunctionNode> for IsFunctionDeclaration {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_isDeclaration(function.cursor.kind) != 0 }
    }
}

#[derive(Clone)]
pub struct IsFunctionDefination;

impl Matcher<FunctionNode> for IsFunctionDefination {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_isCursorDefinition(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsInlineFunction;

impl Matcher<FunctionNode> for IsInlineFunction {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_Cursor_isFunctionInlined(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsTemplateFunction;

impl Matcher<FunctionNode> for IsTemplateFunction {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCursorKind(function.cursor) == CXCursor_FunctionTemplate }
    }
}

#[derive(Clone)]
pub struct IsConversionFunction;

impl Matcher<FunctionNode> for IsConversionFunction {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCursorKind(function.cursor) == CXCursor_ConversionFunction }
    }
}

#[derive(Clone)]
pub struct IsVirtualMatcher;

impl Matcher<FunctionNode> for IsVirtualMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isVirtual(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsPureVirtualMatcher;

impl Matcher<FunctionNode> for IsPureVirtualMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isPureVirtual(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsStaticMethodMatcher;

impl Matcher<FunctionNode> for IsStaticMethodMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isStatic(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsConstMethodMatcher;

impl Matcher<FunctionNode> for IsConstMethodMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isConst(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsDeletedMethodMatcher;

impl Matcher<FunctionNode> for IsDeletedMethodMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXMethod_isConst(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsMethodMatcher;

impl Matcher<FunctionNode> for IsMethodMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCursorKind(function.cursor) == CXCursor_CXXMethod }
    }
}

#[derive(Clone)]
pub struct IsConstructorMatcher;

impl Matcher<FunctionNode> for IsConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCursorKind(function.cursor) == CXCursor_Constructor }
    }
}

#[derive(Clone)]
pub struct IsDefaultConstructorMatcher;

impl Matcher<FunctionNode> for IsDefaultConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXConstructor_isDefaultConstructor(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsCopyConstructorMatcher;

impl Matcher<FunctionNode> for IsCopyConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXConstructor_isCopyConstructor(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsMoveConstructorMatcher;

impl Matcher<FunctionNode> for IsMoveConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXConstructor_isMoveConstructor(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsConvertingConstructorMatcher;

impl Matcher<FunctionNode> for IsConvertingConstructorMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_CXXConstructor_isConvertingConstructor(function.cursor) != 0 }
    }
}

#[derive(Clone)]
pub struct IsDestructorMatcher;

impl Matcher<FunctionNode> for IsDestructorMatcher {
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

impl Matcher<FunctionNode> for AccessSpecifierMatcher {
    fn is_match(&self, function: &FunctionNode) -> bool {
        unsafe { clang_getCXXAccessSpecifier(function.cursor) == self.access }
    }
}
