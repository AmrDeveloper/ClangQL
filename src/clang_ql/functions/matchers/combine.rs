use std::collections::HashMap;

use gitql_ast::types::dynamic::DynamicType;
use gitql_ast::types::varargs::VarargsType;
use gitql_ast::types::variant::VariantType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use gitql_std::meta_types::first_element_type;

use crate::clang_ql::matchers::CombineMatcher;
use crate::clang_ql::matchers::Matcher;
use crate::clang_ql::types::FunctionMatcherType;
use crate::clang_ql::values::FunctionMatcherValue;
use crate::clang_ql::values::FunctionNode;

#[inline(always)]
pub(crate) fn register_combine_matchers_functions(
    map: &mut HashMap<&'static str, StandardFunction>,
) {
    map.insert("m_oneof", matcher_combine_oneof);
    map.insert("m_allof", matcher_combine_allof);
    map.insert("m_noneof", matcher_combine_noneof);
}

#[inline(always)]
pub(crate) fn register_combine_matchers_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_oneof",
        Signature::with_return(Box::new(DynamicType::new(first_element_type)))
            .add_parameter(Box::new(VariantType::new(vec![Box::new(
                FunctionMatcherType,
            )])))
            .add_parameter(Box::new(VarargsType::new(Box::new(DynamicType::new(
                first_element_type,
            ))))),
    );

    map.insert(
        "m_allof",
        Signature::with_return(Box::new(DynamicType::new(first_element_type)))
            .add_parameter(Box::new(VariantType::new(vec![Box::new(
                FunctionMatcherType,
            )])))
            .add_parameter(Box::new(VarargsType::new(Box::new(DynamicType::new(
                first_element_type,
            ))))),
    );

    map.insert(
        "m_noneof",
        Signature::with_return(Box::new(DynamicType::new(first_element_type)))
            .add_parameter(Box::new(VariantType::new(vec![Box::new(
                FunctionMatcherType,
            )])))
            .add_parameter(Box::new(VarargsType::new(Box::new(DynamicType::new(
                first_element_type,
            ))))),
    );
}

fn matcher_combine_oneof(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut matchers: Vec<Box<dyn Matcher<FunctionNode>>> = vec![];
    for value in values.iter() {
        if let Some(matcher_value) = value.as_any().downcast_ref::<FunctionMatcherValue>() {
            matchers.push(matcher_value.matcher.to_owned());
        }
    }

    let combine_matcher = Box::new(CombineMatcher::create_one_of(matchers));
    Box::new(FunctionMatcherValue::new(combine_matcher))
}

fn matcher_combine_allof(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut matchers: Vec<Box<dyn Matcher<FunctionNode>>> = vec![];
    for value in values.iter() {
        if let Some(matcher_value) = value.as_any().downcast_ref::<FunctionMatcherValue>() {
            matchers.push(matcher_value.matcher.to_owned());
        }
    }

    let combine_matcher = Box::new(CombineMatcher::create_all_of(matchers));
    Box::new(FunctionMatcherValue::new(combine_matcher))
}

fn matcher_combine_noneof(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut matchers: Vec<Box<dyn Matcher<FunctionNode>>> = vec![];
    for value in values.iter() {
        if let Some(matcher_value) = value.as_any().downcast_ref::<FunctionMatcherValue>() {
            matchers.push(matcher_value.matcher.to_owned());
        }
    }

    let combine_matcher = Box::new(CombineMatcher::create_none_of(matchers));
    Box::new(FunctionMatcherValue::new(combine_matcher))
}
