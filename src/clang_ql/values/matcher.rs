use gitql_core::values::base::Value;

use crate::clang_ql::matchers::FunctionMatcher;
use crate::clang_ql::types::FunctionMatcherType;

#[derive(Clone)]
pub struct FunctionMatcherValue {
    pub matcher: Box<dyn FunctionMatcher>,
}

impl FunctionMatcherValue {
    pub fn new(matcher: Box<dyn FunctionMatcher>) -> Self {
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
}
