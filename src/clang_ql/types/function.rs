use std::any::Any;

use gitql_ast::types::base::DataType;

#[derive(Clone)]
pub struct FunctionType;

impl DataType for FunctionType {
    fn literal(&self) -> String {
        "Function".to_string()
    }

    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let self_type: Box<dyn DataType> = Box::new(FunctionType);
        other.is_any()
            || other.is_variant_contains(&self_type)
            || other.as_any().downcast_ref::<FunctionType>().is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
