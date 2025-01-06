use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::base::DataType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;

pub fn tables_fields_types() -> HashMap<&'static str, Box<dyn DataType>> {
    let mut map: HashMap<&'static str, Box<dyn DataType>> = HashMap::new();
    map.insert("name", Box::new(TextType));
    map.insert("type", Box::new(TextType));
    map.insert("signature", Box::new(TextType));
    map.insert("class_name", Box::new(TextType));

    map.insert("access_modifier", Box::new(IntType));

    map.insert("is_method", Box::new(BoolType));
    map.insert("is_virtual", Box::new(BoolType));
    map.insert("is_pure_virtual", Box::new(BoolType));
    map.insert("is_static", Box::new(BoolType));
    map.insert("is_const", Box::new(BoolType));
    map.insert("is_variadic", Box::new(BoolType));
    map.insert("is_volatile", Box::new(BoolType));
    map.insert("is_struct", Box::new(BoolType));
    map.insert("has_template", Box::new(BoolType));

    map.insert("return_type", Box::new(TextType));
    map.insert("type_literal", Box::new(TextType));

    map.insert("args_count", Box::new(IntType));
    map.insert("bases_count", Box::new(IntType));
    map.insert("methods_count", Box::new(IntType));
    map.insert("fields_count", Box::new(IntType));
    map.insert("constants_count", Box::new(IntType));

    map.insert("size", Box::new(IntType));
    map.insert("align", Box::new(IntType));

    // Source code location columns
    map.insert("file", Box::new(TextType));
    map.insert("line", Box::new(IntType));
    map.insert("column", Box::new(IntType));
    map.insert("offset", Box::new(IntType));
    map
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
