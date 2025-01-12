use gitql_core::values::base::Value;

use crate::clang_ql::matchers::{Matcher, UnaryCombineMatcher};
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

    fn data_type(&self) -> Box<dyn gitql_ast::types::base::DataType> {
        Box::new(FunctionMatcherType)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn bang_op(&self) -> Result<Box<dyn Value>, String> {
        let combine_matcher = Box::new(UnaryCombineMatcher::not(self.matcher.clone()));
        Ok(Box::new(FunctionMatcherValue::new(combine_matcher)))
    }
}
