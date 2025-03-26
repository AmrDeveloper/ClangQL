use dyn_clone::DynClone;

mod function;
pub use function::AccessSpecifierMatcher;
pub use function::IsConstMethodMatcher;
pub use function::IsConstructorMatcher;
pub use function::IsConversionFunction;
pub use function::IsConvertingConstructorMatcher;
pub use function::IsCopyConstructorMatcher;
pub use function::IsDefaultConstructorMatcher;
pub use function::IsDeletedMethodMatcher;
pub use function::IsDestructorMatcher;
pub use function::IsFunctionDeclaration;
pub use function::IsFunctionDefination;
pub use function::IsMethodMatcher;
pub use function::IsMoveConstructorMatcher;
pub use function::IsPureVirtualMatcher;
pub use function::IsStaticMethodMatcher;
pub use function::IsTemplateFunction;
pub use function::IsVirtualMatcher;

mod combine;
pub use combine::CombineBinaryMatcher;
pub use combine::CombineMatcher;
pub use combine::UnaryCombineMatcher;

dyn_clone::clone_trait_object!(<T> Matcher<T>);

pub trait Matcher<T: Clone>: DynClone {
    fn is_match(&self, node: &T) -> bool;
}
