use dyn_clone::DynClone;

use super::values::FunctionNode;

mod function;
pub use function::IsConstMethodMatcher;
pub use function::IsConstructorMatcher;
pub use function::IsConvertingConstructorMatcher;
pub use function::IsCopyConstructorMatcher;
pub use function::IsDefaultConstructorMatcher;
pub use function::IsDeletedMethodMatcher;
pub use function::IsDestructorMatcher;
pub use function::IsMethodMatcher;
pub use function::IsMoveConstructorMatcher;
pub use function::IsPureVirtualMatcher;
pub use function::IsStaticMethodMatcher;
pub use function::IsVirtualMatcher;

dyn_clone::clone_trait_object!(FunctionMatcher);

pub trait FunctionMatcher: DynClone {
    fn is_match(&self, node: &FunctionNode) -> bool;
}
