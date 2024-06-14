use gitql_ast::expression::Expression;
use gitql_ast::expression::SymbolExpression;
use gitql_core::environment::Environment;
use gitql_core::object::GitQLObject;
use gitql_core::object::Group;
use gitql_core::object::Row;
use gitql_core::value::Value;
use gitql_engine::data_provider::select_values;
use gitql_engine::data_provider::DataProvider;
use gitql_engine::engine_evaluator::evaluate_expression;

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
    fn provide(
        &self,
        env: &mut Environment,
        table: &str,
        fields_names: &[String],
        titles: &[String],
        fields_values: &[Box<dyn Expression>],
        hidden_selection_count: i64,
    ) -> Result<GitQLObject, String> {
        let mut groups: Vec<Group> = vec![];

        for path in &self.paths {
            let mut selected_group = select_clang_ast_objects(
                env,
                path,
                table.to_string(),
                fields_names,
                titles,
                fields_values,
                hidden_selection_count,
            )?;

            if groups.is_empty() {
                groups.push(selected_group);
            } else {
                groups[0].rows.append(&mut selected_group.rows);
            }
        }

        Ok(GitQLObject {
            titles: titles.to_vec(),
            groups,
        })
    }
}

fn select_clang_ast_objects(
    env: &mut Environment,
    path: &str,
    table: String,
    fields_names: &[String],
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
    hidden_selection_count: i64,
) -> Result<Group, String> {
    match table.as_str() {
        "classes" => select_classes(
            env,
            path,
            fields_names,
            titles,
            fields_values,
            hidden_selection_count,
        ),
        "enums" => select_enumss(
            env,
            path,
            fields_names,
            titles,
            fields_values,
            hidden_selection_count,
        ),
        "unions" => select_unions(
            env,
            path,
            fields_names,
            titles,
            fields_values,
            hidden_selection_count,
        ),
        "functions" => select_functions(
            env,
            path,
            fields_names,
            titles,
            fields_values,
            hidden_selection_count,
        ),
        "globals" => select_variables(
            env,
            path,
            fields_names,
            titles,
            fields_values,
            hidden_selection_count,
        ),
        _ => select_values(env, titles, fields_values),
    }
}

fn select_classes(
    env: &mut Environment,
    path: &str,
    fields_names: &[String],
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
    hidden_selection_count: i64,
) -> Result<Group, String> {
    let mut rows: Vec<Row> = vec![];

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    let ast_classes = class::select_clang_classes(path);
    for class in ast_classes.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            if index >= hidden_selection_count && (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            let field_name = &fields_names[index as usize];

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

    Ok(Group { rows })
}

fn select_enumss(
    env: &mut Environment,
    path: &str,
    fields_names: &[String],
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
    hidden_selection_count: i64,
) -> Result<Group, String> {
    let mut rows: Vec<Row> = vec![];

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    let ast_enums = enumeration::select_clang_enums(path);
    for enumeration in ast_enums.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            if index >= hidden_selection_count && (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            let field_name = &fields_names[index as usize];

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

    Ok(Group { rows })
}

fn select_unions(
    env: &mut Environment,
    path: &str,
    fields_names: &[String],
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
    hidden_selection_count: i64,
) -> Result<Group, String> {
    let mut rows: Vec<Row> = vec![];

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    let ast_unions = unions::select_clang_unions(path);
    for union_node in ast_unions.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            if index >= hidden_selection_count && (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            let field_name = &fields_names[index as usize];

            if field_name == "name" {
                values.push(Value::Text(union_node.name.to_owned()));
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

    Ok(Group { rows })
}

fn select_functions(
    env: &mut Environment,
    path: &str,
    fields_names: &[String],
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
    hidden_selection_count: i64,
) -> Result<Group, String> {
    let mut rows: Vec<Row> = vec![];

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    let ast_functions = function::select_clang_functions(path);
    for function in ast_functions.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            if index >= hidden_selection_count && (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            let field_name = &fields_names[index as usize];

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

    Ok(Group { rows })
}

fn select_variables(
    env: &mut Environment,
    path: &str,
    fields_names: &[String],
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
    hidden_selection_count: i64,
) -> Result<Group, String> {
    let mut rows: Vec<Row> = vec![];

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    let ast_variables = global::select_clang_variables(path);
    for variable in ast_variables.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            if index >= hidden_selection_count && (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            let field_name = &fields_names[index as usize];

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

    Ok(Group { rows })
}
