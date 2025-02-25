use std::collections::HashMap;

use clang_sys::clang_CXXMethod_isConst;
use clang_sys::clang_CXXMethod_isPureVirtual;
use clang_sys::clang_CXXMethod_isStatic;
use clang_sys::clang_CXXMethod_isVirtual;
use clang_sys::clang_getCursorKind;
use clang_sys::CXCursor_CXXMethod;
use gitql_ast::types::boolean::BoolType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::Value;

use crate::clang_ql::types::FunctionType;
use crate::clang_ql::values::FunctionValue;

#[inline(always)]
pub fn register_ast_function_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("is_virtual", function_is_virtual);
    map.insert("is_pure_virtual", function_is_pure_virtual);
    map.insert("is_method", function_is_method);
    map.insert("is_static", function_is_static);
    map.insert("is_const", function_is_const);
    map.insert("is_deleted", function_is_deleted);
}

#[inline(always)]
pub fn register_ast_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "is_virtual",
        Signature::with_return(Box::new(BoolType)).add_parameter(Box::new(FunctionType)),
    );

    map.insert(
        "is_pure_virtual",
        Signature::with_return(Box::new(BoolType)).add_parameter(Box::new(FunctionType)),
    );

    map.insert(
        "is_method",
        Signature::with_return(Box::new(BoolType)).add_parameter(Box::new(FunctionType)),
    );

    map.insert(
        "is_static",
        Signature::with_return(Box::new(BoolType)).add_parameter(Box::new(FunctionType)),
    );

    map.insert(
        "is_const",
        Signature::with_return(Box::new(BoolType)).add_parameter(Box::new(FunctionType)),
    );

    map.insert(
        "is_deleted",
        Signature::with_return(Box::new(BoolType)).add_parameter(Box::new(FunctionType)),
    );
}

fn function_is_virtual(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let ast_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let is_virtual = unsafe { clang_CXXMethod_isVirtual(ast_node.node.cursor) != 0 };
    Box::new(BoolValue::new(is_virtual))
}

fn function_is_pure_virtual(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let ast_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let is_virtual = unsafe { clang_CXXMethod_isPureVirtual(ast_node.node.cursor) != 0 };
    Box::new(BoolValue::new(is_virtual))
}

fn function_is_method(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let ast_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let is_virtual = unsafe {
        let cursor_kind = clang_getCursorKind(ast_node.node.cursor);
        cursor_kind == CXCursor_CXXMethod
    };
    Box::new(BoolValue::new(is_virtual))
}

fn function_is_static(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let ast_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let is_virtual = unsafe { clang_CXXMethod_isStatic(ast_node.node.cursor) != 0 };
    Box::new(BoolValue::new(is_virtual))
}

fn function_is_const(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let ast_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let is_virtual = unsafe { clang_CXXMethod_isConst(ast_node.node.cursor) != 0 };
    Box::new(BoolValue::new(is_virtual))
}

fn function_is_deleted(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let ast_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let is_virtual = unsafe { clang_CXXMethod_isConst(ast_node.node.cursor) != 0 };
    Box::new(BoolValue::new(is_virtual))
}
