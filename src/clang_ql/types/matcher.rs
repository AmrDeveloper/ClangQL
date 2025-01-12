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

    fn can_perform_logical_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FunctionMatcherType)]
    }

    fn logical_and_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(FunctionMatcherType)
    }

    fn can_perform_logical_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FunctionMatcherType)]
    }

    fn logical_or_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(FunctionMatcherType)
    }

    fn can_perform_logical_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(FunctionMatcherType)]
    }

    fn logical_xor_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(FunctionMatcherType)
    }

    fn can_perform_bang_op(&self) -> bool {
        true
    }

    fn bang_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(FunctionMatcherType)
    }
}
