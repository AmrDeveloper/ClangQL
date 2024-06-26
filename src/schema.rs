use gitql_core::types::DataType;
use std::collections::HashMap;
use std::sync::OnceLock;

pub fn tables_fields_types() -> &'static HashMap<&'static str, DataType> {
    static HASHMAP: OnceLock<HashMap<&'static str, DataType>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("name", DataType::Text);
        map.insert("type", DataType::Text);
        map.insert("signature", DataType::Text);
        map.insert("class_name", DataType::Text);

        map.insert("access_modifier", DataType::Integer);

        map.insert("is_method", DataType::Boolean);
        map.insert("is_virtual", DataType::Boolean);
        map.insert("is_pure_virtual", DataType::Boolean);
        map.insert("is_static", DataType::Boolean);
        map.insert("is_const", DataType::Boolean);
        map.insert("is_variadic", DataType::Boolean);
        map.insert("is_volatile", DataType::Boolean);
        map.insert("is_struct", DataType::Boolean);
        map.insert("has_template", DataType::Boolean);

        map.insert("return_type", DataType::Text);
        map.insert("type_literal", DataType::Text);

        map.insert("args_count", DataType::Integer);
        map.insert("bases_count", DataType::Integer);
        map.insert("methods_count", DataType::Integer);
        map.insert("fields_count", DataType::Integer);
        map.insert("constants_count", DataType::Integer);

        map.insert("size", DataType::Integer);
        map.insert("align", DataType::Integer);

        // Source code location columns
        map.insert("file", DataType::Text);
        map.insert("line", DataType::Integer);
        map.insert("column", DataType::Integer);
        map.insert("offset", DataType::Integer);
        map
    })
}

pub fn tables_fields_names() -> &'static HashMap<&'static str, Vec<&'static str>> {
    static HASHMAP: OnceLock<HashMap<&'static str, Vec<&'static str>>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(
            "classes",
            vec![
                "name",
                "is_struct",
                "bases_count",
                "methods_count",
                "fields_count",
                "size",
                "align",
                "line",
                "column",
                "offset",
            ],
        );
        map.insert(
            "enums",
            vec![
                "name",
                "constants_count",
                "type_literal",
                "file",
                "line",
                "column",
                "offset",
            ],
        );
        map.insert(
            "unions",
            vec![
                "name",
                "fields_count",
                "size",
                "file",
                "line",
                "column",
                "offset",
            ],
        );
        map.insert(
            "functions",
            vec![
                "name",
                "signature",
                "args_count",
                "return_type",
                "class_name",
                "is_method",
                "is_virtual",
                "is_pure_virtual",
                "is_static",
                "is_const",
                "has_template",
                "access_modifier",
                "is_variadic",
                "file",
                "line",
                "column",
                "offset",
            ],
        );
        map.insert(
            "globals",
            vec![
                "name",
                "type",
                "is_volatile",
                "file",
                "line",
                "column",
                "offset",
            ],
        );
        map
    })
}
