use std::any::Any;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use gitql_core::values::base::Value;

use crate::clang_ql::types::SourceLocType;

#[derive(PartialEq, Clone)]
pub struct FileLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

impl Display for FileLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&format!("{}:{}:{}", self.file, self.line, self.column))
    }
}

#[derive(Clone)]
pub struct SourceLocValue {
    source_loc: FileLocation,
}

impl SourceLocValue {
    pub fn new(source_loc: FileLocation) -> Self {
        SourceLocValue { source_loc }
    }
}

impl Value for SourceLocValue {
    fn literal(&self) -> String {
        self.source_loc.to_string()
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_loc) = other.as_any().downcast_ref::<SourceLocValue>() {
            return self.source_loc.eq(&other_loc.source_loc);
        }
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<std::cmp::Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn gitql_ast::types::base::DataType> {
        Box::new(SourceLocType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
