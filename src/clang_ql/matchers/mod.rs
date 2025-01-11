use dyn_clone::DynClone;

use super::values::FunctionNode;

dyn_clone::clone_trait_object!(FunctionMatcher);

pub trait FunctionMatcher: DynClone {
    fn is_match(&self, function: FunctionNode) -> bool;
}
