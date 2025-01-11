use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::base::DataType;
use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::text::TextType;
use gitql_core::environment::Environment;
use gitql_core::schema::Schema;
use gitql_std::aggregation::aggregation_function_signatures;
use gitql_std::aggregation::aggregation_functions;
use gitql_std::window::window_function_signatures;
use gitql_std::window::window_functions;

use super::functions::clang_ql_functions;
use super::functions::clang_ql_functions_signatures;
use super::types::FunctionType;
use super::types::SourceLocType;

fn tables_fields_types() -> HashMap<&'static str, Box<dyn DataType>> {
    let mut map: HashMap<&'static str, Box<dyn DataType>> = HashMap::new();
    map.insert("name", Box::new(TextType));
    map.insert("type", Box::new(TextType));
    map.insert("signature", Box::new(TextType));
    map.insert("ast_function", Box::new(FunctionType));

    map.insert("is_volatile", Box::new(BoolType));
    map.insert("is_struct", Box::new(BoolType));

    map.insert("return_type", Box::new(TextType));
    map.insert("type_literal", Box::new(TextType));

    map.insert("args_count", Box::new(IntType));
    map.insert("bases_count", Box::new(IntType));
    map.insert("methods_count", Box::new(IntType));
    map.insert("fields_count", Box::new(IntType));
    map.insert("constants_count", Box::new(IntType));

    map.insert("size", Box::new(IntType));
    map.insert("align", Box::new(IntType));

    map.insert("source_loc", Box::new(SourceLocType));
    map
}

fn tables_fields_names() -> &'static HashMap<&'static str, Vec<&'static str>> {
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
                "source_loc",
            ],
        );
        map.insert(
            "enums",
            vec!["name", "constants_count", "type_literal", "source_loc"],
        );
        map.insert("unions", vec!["name", "fields_count", "size", "source_loc"]);
        map.insert(
            "functions",
            vec![
                "name",
                "signature",
                "return_type",
                "ast_function",
                "source_loc",
            ],
        );
        map.insert("globals", vec!["name", "type", "is_volatile", "source_loc"]);
        map
    })
}

pub fn create_clang_ql_environment() -> Environment {
    let schema = Schema {
        tables_fields_names: tables_fields_names().to_owned(),
        tables_fields_types: tables_fields_types().to_owned(),
    };

    let std_signatures = clang_ql_functions_signatures();
    let std_functions = clang_ql_functions();

    let aggregation_signatures = aggregation_function_signatures();
    let aggregation_functions = aggregation_functions();

    let window_signatures = window_function_signatures();
    let window_function = window_functions();

    let mut env = Environment::new(schema);
    env.with_standard_functions(&std_signatures, std_functions);
    env.with_aggregation_functions(&aggregation_signatures, aggregation_functions);
    env.with_window_functions(&window_signatures, window_function);
    env
}
