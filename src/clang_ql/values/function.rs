use clang_sys::CXCursor;
use gitql_core::values::Value;

use crate::clang_ql::types::FunctionType;

use super::FileLocation;

#[derive(Clone)]
pub struct FunctionNode {
    pub name: String,
    pub cursor: CXCursor,

    #[allow(dead_code)]
    pub parent: CXCursor,

    pub signature: String,
    pub return_type: String,
    pub location: FileLocation,
}

#[derive(Clone)]
pub struct FunctionValue {
    pub node: FunctionNode,
}

impl FunctionValue {
    pub fn new(node: FunctionNode) -> Self {
        FunctionValue { node }
    }
}

impl Value for FunctionValue {
    fn literal(&self) -> String {
        self.node.signature.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_fun) = other.as_any().downcast_ref::<FunctionValue>() {
            return self.node.name.eq(&other_fun.node.name)
                && self.node.signature.eq(&other_fun.node.signature)
                && self.node.return_type.eq(&other_fun.node.return_type)
                && self.node.location.eq(&other_fun.node.location);
        }
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<std::cmp::Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn gitql_ast::types::DataType> {
        Box::new(FunctionType)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
