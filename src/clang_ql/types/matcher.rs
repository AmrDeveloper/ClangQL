use std::any::Any;

use gitql_ast::types::base::DataType;

#[derive(Clone)]
pub struct FunctionMatcherType;

impl DataType for FunctionMatcherType {
    fn literal(&self) -> String {
        "FunctionMatcherType".to_string()
    }

    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let self_type: Box<dyn DataType> = Box::new(FunctionMatcherType);
        other.is_any()
            || other.is_variant_contains(&self_type)
            || other
                .as_any()
                .downcast_ref::<FunctionMatcherType>()
                .is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_bang_op(&self) -> bool {
        true
    }

    fn bang_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(FunctionMatcherType)
    }
}
