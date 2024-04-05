use gitql_ast::types::DataType;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLES_FIELDS_TYPES: HashMap<&'static str, DataType> = {
        let mut map = HashMap::new();
        map.insert("name", DataType::Text);
        map.insert("signature", DataType::Text);
        map.insert("args_count", DataType::Integer);
        map.insert("return_type", DataType::Text);
        map.insert("is_method", DataType::Boolean);
        map
    };
}

lazy_static! {
    pub static ref TABLES_FIELDS_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert(
            "functions",
            vec![
                "name",
                "signature",
                "args_count",
                "return_type",
                "is_method",
            ],
        );
        map
    };
}
