use std::vec;

use gitql_core::object::Row;
use gitql_core::value::Value;
use gitql_engine::data_provider::DataProvider;

use crate::visitor::class;
use crate::visitor::enumeration;
use crate::visitor::function;
use crate::visitor::global;
use crate::visitor::unions;

pub struct ClangAstDataProvider {
    pub paths: Vec<String>,
}

impl ClangAstDataProvider {
    pub fn new(paths: Vec<String>) -> Self {
        Self { paths }
    }
}

impl DataProvider for ClangAstDataProvider {
    fn provide(&self, table: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
        let mut rows: Vec<Row> = vec![];

        for path in &self.paths {
            let mut selected_rows = select_clang_ast_objects(path, table, selected_columns)?;
            rows.append(&mut selected_rows);
        }

        Ok(rows)
    }
}

fn select_clang_ast_objects(
    path: &str,
    table: &str,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let rows = match table {
        "classes" => select_classes(path, selected_columns)?,
        "enums" => select_enumss(path, selected_columns)?,
        "unions" => select_unions(path, selected_columns)?,
        "functions" => select_functions(path, selected_columns)?,
        "globals" => select_variables(path, selected_columns)?,
        _ => vec![],
    };
    Ok(rows)
}

fn select_classes(path: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_classes = class::select_clang_classes(path);
    for class in ast_classes.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(selected_columns.len());

        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Value::Text(class.name.to_owned()));
                continue;
            }

            if field_name == "bases_count" {
                values.push(Value::Integer(class.attributes.bases_count.into()));
                continue;
            }

            if field_name == "methods_count" {
                values.push(Value::Integer(class.attributes.methods_count.into()));
                continue;
            }

            if field_name == "fields_count" {
                values.push(Value::Integer(class.attributes.fields_count.into()));
                continue;
            }

            if field_name == "is_struct" {
                values.push(Value::Boolean(class.is_struct));
                continue;
            }

            if field_name == "size" {
                values.push(Value::Integer(class.size));
                continue;
            }

            if field_name == "align" {
                values.push(Value::Integer(class.align));
                continue;
            }

            if field_name == "file" {
                values.push(Value::Text(class.location.file.to_string()));
                continue;
            }

            if field_name == "line" {
                values.push(Value::Integer(class.location.line.into()));
                continue;
            }

            if field_name == "column" {
                values.push(Value::Integer(class.location.column.into()));
                continue;
            }

            if field_name == "offset" {
                values.push(Value::Integer(class.location.offset.into()));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_enumss(path: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_enums = enumeration::select_clang_enums(path);
    for enumeration in ast_enums.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(selected_columns.len());

        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Value::Text(enumeration.name.to_owned()));
                continue;
            }

            if field_name == "constants_count" {
                values.push(Value::Integer(
                    enumeration.attributes.constants_count.into(),
                ));
                continue;
            }

            if field_name == "type_literal" {
                values.push(Value::Text(enumeration.type_literal.to_string()));
                continue;
            }

            if field_name == "file" {
                values.push(Value::Text(enumeration.location.file.to_string()));
                continue;
            }

            if field_name == "line" {
                values.push(Value::Integer(enumeration.location.line.into()));
                continue;
            }

            if field_name == "column" {
                values.push(Value::Integer(enumeration.location.column.into()));
                continue;
            }

            if field_name == "offset" {
                values.push(Value::Integer(enumeration.location.offset.into()));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_unions(path: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_unions = unions::select_clang_unions(path);
    for union_node in ast_unions.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(selected_columns.len());

        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Value::Text(union_node.name.to_owned()));
                continue;
            }

            if field_name == "fields_count" {
                values.push(Value::Integer(union_node.attributes.fields_count.into()));
                continue;
            }

            if field_name == "size" {
                values.push(Value::Integer(union_node.size));
                continue;
            }

            if field_name == "file" {
                values.push(Value::Text(union_node.location.file.to_string()));
                continue;
            }

            if field_name == "line" {
                values.push(Value::Integer(union_node.location.line.into()));
                continue;
            }

            if field_name == "column" {
                values.push(Value::Integer(union_node.location.column.into()));
                continue;
            }

            if field_name == "offset" {
                values.push(Value::Integer(union_node.location.offset.into()));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_functions(path: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_functions = function::select_clang_functions(path);
    for function in ast_functions.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(selected_columns.len());

        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Value::Text(function.name.to_owned()));
                continue;
            }

            if field_name == "signature" {
                values.push(Value::Text(function.signature.to_owned()));
                continue;
            }

            if field_name == "args_count" {
                values.push(Value::Integer(function.arguments_count as i64));
                continue;
            }

            if field_name == "class_name" {
                values.push(Value::Text(function.class_name.to_owned()));
                continue;
            }

            if field_name == "return_type" {
                values.push(Value::Text(function.return_type.to_owned()));
                continue;
            }

            if field_name == "is_method" {
                values.push(Value::Boolean(function.is_method));
                continue;
            }

            if field_name == "is_virtual" {
                values.push(Value::Boolean(function.is_virtual));
                continue;
            }

            if field_name == "is_pure_virtual" {
                values.push(Value::Boolean(function.is_pure_virtual));
                continue;
            }

            if field_name == "is_static" {
                values.push(Value::Boolean(function.is_static));
                continue;
            }

            if field_name == "is_const" {
                values.push(Value::Boolean(function.is_const));
                continue;
            }

            if field_name == "has_template" {
                values.push(Value::Boolean(function.has_template));
                continue;
            }

            if field_name == "access_modifier" {
                values.push(Value::Integer(function.access_modifier as i64));
                continue;
            }

            if field_name == "is_variadic" {
                values.push(Value::Boolean(function.is_variadic));
                continue;
            }

            if field_name == "file" {
                values.push(Value::Text(function.location.file.to_string()));
                continue;
            }

            if field_name == "line" {
                values.push(Value::Integer(function.location.line as i64));
                continue;
            }

            if field_name == "column" {
                values.push(Value::Integer(function.location.column as i64));
                continue;
            }

            if field_name == "offset" {
                values.push(Value::Integer(function.location.offset as i64));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_variables(path: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_variables = global::select_clang_variables(path);
    for variable in ast_variables.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(selected_columns.len());
        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Value::Text(variable.name.to_owned()));
                continue;
            }

            if field_name == "type" {
                values.push(Value::Text(variable.type_literal.to_owned()));
                continue;
            }

            if field_name == "is_volatile" {
                values.push(Value::Boolean(variable.is_volatile));
                continue;
            }

            if field_name == "file" {
                values.push(Value::Text(variable.location.file.to_string()));
                continue;
            }

            if field_name == "line" {
                values.push(Value::Integer(variable.location.line as i64));
                continue;
            }

            if field_name == "column" {
                values.push(Value::Integer(variable.location.column as i64));
                continue;
            }

            if field_name == "offset" {
                values.push(Value::Integer(variable.location.offset as i64));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}
