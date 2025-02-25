use std::any::Any;

use gitql_ast::types::DataType;

#[derive(Clone)]
pub struct SourceLocType;

impl DataType for SourceLocType {
    fn literal(&self) -> String {
        "SourceLoc".to_string()
    }

    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let self_type: Box<dyn DataType> = Box::new(SourceLocType);
        other.is_any()
            || other.is_variant_contains(&self_type)
            || other.as_any().downcast_ref::<SourceLocType>().is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
