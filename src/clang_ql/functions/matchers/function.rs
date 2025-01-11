use std::collections::HashMap;

use gitql_ast::types::boolean::BoolType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;

use crate::clang_ql::matchers::IsConstMethodMatcher;
use crate::clang_ql::matchers::IsDeletedMethodMatcher;
use crate::clang_ql::matchers::IsMethodMatcher;
use crate::clang_ql::matchers::IsPureVirtualMatcher;
use crate::clang_ql::matchers::IsStaticMethodMatcher;
use crate::clang_ql::matchers::IsVirtualMatcher;
use crate::clang_ql::types::FunctionMatcherType;
use crate::clang_ql::types::FunctionType;
use crate::clang_ql::values::FunctionMatcherValue;
use crate::clang_ql::values::FunctionValue;

#[inline(always)]
pub(crate) fn register_function_matchers_functions(
    map: &mut HashMap<&'static str, StandardFunction>,
) {
    map.insert("m_function", match_function);

    map.insert("m_virtual", match_virtual_function);
    map.insert("m_pure_virtual", match_pure_virtual_function);
    map.insert("m_static", match_static_function);
    map.insert("m_const", match_const_function);
    map.insert("m_deleted", match_deleted_function);
    map.insert("m_method", match_method_function);
}

#[inline(always)]
pub(crate) fn register_function_matchers_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_function",
        Signature::with_return(Box::new(BoolType))
            .add_parameter(Box::new(FunctionType))
            .add_parameter(Box::new(FunctionMatcherType)),
    );

    map.insert(
        "m_virtual",
        Signature::with_return(Box::new(FunctionMatcherType)),
    );

    map.insert(
        "m_pure_virtual",
        Signature::with_return(Box::new(FunctionMatcherType)),
    );

    map.insert(
        "m_const",
        Signature::with_return(Box::new(FunctionMatcherType)),
    );

    map.insert(
        "m_deleted",
        Signature::with_return(Box::new(FunctionMatcherType)),
    );

    map.insert(
        "m_method",
        Signature::with_return(Box::new(FunctionMatcherType)),
    );
}

fn match_function(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let function_node = values[0].as_any().downcast_ref::<FunctionValue>().unwrap();
    let function_matcher = values[1]
        .as_any()
        .downcast_ref::<FunctionMatcherValue>()
        .unwrap();
    let is_matches = function_matcher.matcher.is_match(&function_node.node);
    Box::new(BoolValue::new(is_matches))
}

fn match_virtual_function(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(IsVirtualMatcher);
    Box::new(FunctionMatcherValue::new(matcher))
}

fn match_pure_virtual_function(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(IsPureVirtualMatcher);
    Box::new(FunctionMatcherValue::new(matcher))
}

fn match_static_function(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(IsStaticMethodMatcher);
    Box::new(FunctionMatcherValue::new(matcher))
}

fn match_const_function(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(IsConstMethodMatcher);
    Box::new(FunctionMatcherValue::new(matcher))
}

fn match_deleted_function(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(IsDeletedMethodMatcher);
    Box::new(FunctionMatcherValue::new(matcher))
}

fn match_method_function(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(IsMethodMatcher);
    Box::new(FunctionMatcherValue::new(matcher))
}
