use std::collections::HashMap;

use clang_sys::clang_CXXMethod_isPureVirtual;
use clang_sys::clang_CXXMethod_isVirtual;
use gitql_ast::types::boolean::BoolType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;

use crate::clang_ql::types::FunctionType;
use crate::clang_ql::values::FunctionValue;

#[inline(always)]
pub fn register_ast_function_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("is_virtual", function_is_virtual);
    map.insert("is_pure_virtual", is_pure_virtual);
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
}

fn function_is_virtual(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let ast_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let is_virtual = unsafe { clang_CXXMethod_isVirtual(ast_node.node.cursor) != 0 };
    Box::new(BoolValue::new(is_virtual))
}

fn is_pure_virtual(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let ast_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let is_virtual = unsafe { clang_CXXMethod_isPureVirtual(ast_node.node.cursor) != 0 };
    Box::new(BoolValue::new(is_virtual))
}
