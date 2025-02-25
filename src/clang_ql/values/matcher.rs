use gitql_core::values::Value;

use crate::clang_ql::matchers::{CombineBinaryMatcher, Matcher, UnaryCombineMatcher};
use crate::clang_ql::types::FunctionMatcherType;

use super::FunctionNode;

#[derive(Clone)]
pub struct FunctionMatcherValue {
    pub matcher: Box<dyn Matcher<FunctionNode>>,
}

impl FunctionMatcherValue {
    pub fn new(matcher: Box<dyn Matcher<FunctionNode>>) -> Self {
        FunctionMatcherValue { matcher }
    }
}

impl Value for FunctionMatcherValue {
    fn literal(&self) -> String {
        "FunctionMatcherValue".to_string()
    }

    fn equals(&self, _other: &Box<dyn Value>) -> bool {
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<std::cmp::Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn gitql_ast::types::DataType> {
        Box::new(FunctionMatcherType)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn logical_and_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let lhs = self.matcher.clone();
        let rhs = other
            .as_any()
            .downcast_ref::<FunctionMatcherValue>()
            .unwrap()
            .matcher
            .clone();
        let combine_matcher = Box::new(CombineBinaryMatcher::and(lhs, rhs));
        Ok(Box::new(FunctionMatcherValue::new(combine_matcher)))
    }

    fn logical_or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let lhs = self.matcher.clone();
        let rhs = other
            .as_any()
            .downcast_ref::<FunctionMatcherValue>()
            .unwrap()
            .matcher
            .clone();
        let combine_matcher = Box::new(CombineBinaryMatcher::or(lhs, rhs));
        Ok(Box::new(FunctionMatcherValue::new(combine_matcher)))
    }

    fn logical_xor_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let lhs = self.matcher.clone();
        let rhs = other
            .as_any()
            .downcast_ref::<FunctionMatcherValue>()
            .unwrap()
            .matcher
            .clone();
        let combine_matcher = Box::new(CombineBinaryMatcher::xor(lhs, rhs));
        Ok(Box::new(FunctionMatcherValue::new(combine_matcher)))
    }

    fn bang_op(&self) -> Result<Box<dyn Value>, String> {
        let combine_matcher = Box::new(UnaryCombineMatcher::not(self.matcher.clone()));
        Ok(Box::new(FunctionMatcherValue::new(combine_matcher)))
    }
}
