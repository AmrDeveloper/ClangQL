use dyn_clone::DynClone;

use super::values::FunctionNode;

mod function;
pub use function::IsConstMethodMatcher;
pub use function::IsDeletedMethodMatcher;
pub use function::IsMethodMatcher;
pub use function::IsPureVirtualMatcher;
pub use function::IsStaticMethodMatcher;
pub use function::IsVirtualMatcher;

dyn_clone::clone_trait_object!(FunctionMatcher);

pub trait FunctionMatcher: DynClone {
    fn is_match(&self, node: &FunctionNode) -> bool;
}
