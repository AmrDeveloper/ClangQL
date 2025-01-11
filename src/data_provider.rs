use std::vec;

use clang_sys::clang_disposeIndex;
use clang_sys::clang_disposeTranslationUnit;
use gitql_core::object::Row;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;
use gitql_engine::data_provider::DataProvider;

use crate::clang_parser::CompilationUnit;
use crate::visitor::class;
use crate::visitor::enumeration;
use crate::visitor::function;
use crate::visitor::global;
use crate::visitor::unions;

pub struct ClangDataProvider {
    pub compilation_units: Vec<CompilationUnit>,
}

impl ClangDataProvider {
    pub fn new(compilation_units: Vec<CompilationUnit>) -> Self {
        Self { compilation_units }
    }
}

impl DataProvider for ClangDataProvider {
    fn provide(&self, table: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
        let mut rows: Vec<Row> = vec![];
        for compilation_unit in &self.compilation_units {
            let mut selected_rows =
                select_clang_ast_objects(compilation_unit, table, selected_columns)?;
            rows.append(&mut selected_rows);
        }
        Ok(rows)
    }
}

impl Drop for ClangDataProvider {
    fn drop(&mut self) {
        for compilation_unit in self.compilation_units.iter() {
            unsafe {
                // Dispose the translation unit
                clang_disposeTranslationUnit(compilation_unit.translation_unit);

                // Dispose the index
                clang_disposeIndex(compilation_unit.index);
            }
        }
    }
}

fn select_clang_ast_objects(
    compilation_unit: &CompilationUnit,
    table: &str,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let rows = match table {
        "classes" => select_classes(compilation_unit, selected_columns)?,
        "enums" => select_enums(compilation_unit, selected_columns)?,
        "unions" => select_unions(compilation_unit, selected_columns)?,
        "functions" => select_functions(compilation_unit, selected_columns)?,
        "globals" => select_variables(compilation_unit, selected_columns)?,
        _ => vec![Row { values: vec![] }],
    };
    Ok(rows)
}

fn select_classes(
    compilation_unit: &CompilationUnit,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_classes = class::select_clang_classes(compilation_unit);
    for class in ast_classes.iter() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Box::new(TextValue::new(class.name.to_owned())));
                continue;
            }

            if field_name == "bases_count" {
                values.push(Box::new(IntValue::new(class.attributes.bases_count.into())));
                continue;
            }

            if field_name == "methods_count" {
                values.push(Box::new(IntValue::new(
                    class.attributes.methods_count.into(),
                )));
                continue;
            }

            if field_name == "fields_count" {
                values.push(Box::new(IntValue::new(
                    class.attributes.fields_count.into(),
                )));
                continue;
            }

            if field_name == "is_struct" {
                values.push(Box::new(BoolValue::new(class.is_struct)));
                continue;
            }

            if field_name == "size" {
                values.push(Box::new(IntValue::new(class.size)));
                continue;
            }

            if field_name == "align" {
                values.push(Box::new(IntValue::new(class.align)));
                continue;
            }

            if field_name == "file" {
                values.push(Box::new(TextValue::new(class.location.file.to_string())));
                continue;
            }

            if field_name == "line" {
                values.push(Box::new(IntValue::new(class.location.line.into())));
                continue;
            }

            if field_name == "column" {
                values.push(Box::new(IntValue::new(class.location.column.into())));
                continue;
            }

            if field_name == "offset" {
                values.push(Box::new(IntValue::new(class.location.offset.into())));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_enums(
    compilation_unit: &CompilationUnit,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_enums = enumeration::select_clang_enums(compilation_unit);
    for enumeration in ast_enums.iter() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Box::new(TextValue::new(enumeration.name.to_owned())));
                continue;
            }

            if field_name == "constants_count" {
                let value = enumeration.attributes.constants_count.into();
                values.push(Box::new(IntValue::new(value)));
                continue;
            }

            if field_name == "type_literal" {
                let value = Box::new(TextValue::new(enumeration.type_literal.to_owned()));
                values.push(value);
                continue;
            }

            if field_name == "file" {
                let value = Box::new(TextValue::new(enumeration.location.file.to_owned()));
                values.push(value);
                continue;
            }

            if field_name == "line" {
                values.push(Box::new(IntValue::new(enumeration.location.line.into())));
                continue;
            }

            if field_name == "column" {
                values.push(Box::new(IntValue::new(enumeration.location.column.into())));
                continue;
            }

            if field_name == "offset" {
                values.push(Box::new(IntValue::new(enumeration.location.offset.into())));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_unions(
    compilation_unit: &CompilationUnit,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_unions = unions::select_clang_unions(compilation_unit);
    for union_node in ast_unions.iter() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Box::new(TextValue::new(union_node.name.to_owned())));
                continue;
            }

            if field_name == "fields_count" {
                let value = union_node.attributes.fields_count.into();
                values.push(Box::new(IntValue::new(value)));
                continue;
            }

            if field_name == "size" {
                values.push(Box::new(IntValue::new(union_node.size)));
                continue;
            }

            if field_name == "file" {
                let value = Box::new(TextValue::new(union_node.location.file.to_owned()));
                values.push(value);
                continue;
            }

            if field_name == "line" {
                values.push(Box::new(IntValue::new(union_node.location.line.into())));
                continue;
            }

            if field_name == "column" {
                values.push(Box::new(IntValue::new(union_node.location.column.into())));
                continue;
            }

            if field_name == "offset" {
                values.push(Box::new(IntValue::new(union_node.location.offset.into())));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_functions(
    compilation_unit: &CompilationUnit,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_functions = function::select_clang_functions(compilation_unit);
    for function in ast_functions.iter() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Box::new(TextValue::new(function.name.to_owned())));
                continue;
            }

            if field_name == "signature" {
                values.push(Box::new(TextValue::new(function.signature.to_owned())));
                continue;
            }

            if field_name == "args_count" {
                values.push(Box::new(IntValue::new(function.arguments_count as i64)));
                continue;
            }

            if field_name == "class_name" {
                values.push(Box::new(TextValue::new(function.class_name.to_owned())));
                continue;
            }

            if field_name == "return_type" {
                values.push(Box::new(TextValue::new(function.return_type.to_owned())));
                continue;
            }

            if field_name == "is_method" {
                values.push(Box::new(BoolValue::new(function.is_method)));
                continue;
            }

            if field_name == "is_virtual" {
                values.push(Box::new(BoolValue::new(function.is_virtual)));
                continue;
            }

            if field_name == "is_pure_virtual" {
                values.push(Box::new(BoolValue::new(function.is_pure_virtual)));
                continue;
            }

            if field_name == "is_static" {
                values.push(Box::new(BoolValue::new(function.is_static)));
                continue;
            }

            if field_name == "is_const" {
                values.push(Box::new(BoolValue::new(function.is_const)));
                continue;
            }

            if field_name == "has_template" {
                values.push(Box::new(BoolValue::new(function.has_template)));
                continue;
            }

            if field_name == "access_modifier" {
                values.push(Box::new(IntValue::new(function.access_modifier as i64)));
                continue;
            }

            if field_name == "is_variadic" {
                values.push(Box::new(BoolValue::new(function.is_variadic)));
                continue;
            }

            if field_name == "file" {
                values.push(Box::new(TextValue::new(function.location.file.to_owned())));
                continue;
            }

            if field_name == "line" {
                values.push(Box::new(IntValue::new(function.location.line.into())));
                continue;
            }

            if field_name == "column" {
                values.push(Box::new(IntValue::new(function.location.column.into())));
                continue;
            }

            if field_name == "offset" {
                values.push(Box::new(IntValue::new(function.location.offset.into())));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_variables(
    compilation_unit: &CompilationUnit,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let ast_variables = global::select_clang_variables(compilation_unit);
    for variable in ast_variables.iter() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());
        for field_name in selected_columns {
            if field_name == "name" {
                values.push(Box::new(TextValue::new(variable.name.to_owned())));
                continue;
            }

            if field_name == "type" {
                values.push(Box::new(TextValue::new(variable.type_literal.to_owned())));
                continue;
            }

            if field_name == "is_volatile" {
                values.push(Box::new(BoolValue::new(variable.is_volatile)));
                continue;
            }

            if field_name == "file" {
                values.push(Box::new(TextValue::new(variable.location.file.to_string())));
                continue;
            }

            if field_name == "line" {
                values.push(Box::new(IntValue::new(variable.location.line as i64)));
                continue;
            }

            if field_name == "column" {
                values.push(Box::new(IntValue::new(variable.location.column as i64)));
                continue;
            }

            if field_name == "offset" {
                values.push(Box::new(IntValue::new(variable.location.offset as i64)));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}
