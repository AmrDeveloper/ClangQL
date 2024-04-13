use gitql_ast::environment::Environment;
use gitql_ast::expression::Expression;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::object::GitQLObject;
use gitql_ast::object::Group;
use gitql_ast::object::Row;
use gitql_ast::value::Value;
use gitql_engine::data_provider::select_values;
use gitql_engine::data_provider::DataProvider;
use gitql_engine::engine_evaluator::evaluate_expression;

use crate::clang_function_visitor;
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
    ) -> GitQLObject {
        let mut groups: Vec<Group> = vec![];

        for path in &self.paths {
            let selected_group = select_clang_ast_objects(
                env,
                path,
                table.to_string(),
                fields_names,
                titles,
                fields_values,
            );

            if let Ok(mut group) = selected_group {
                if groups.is_empty() {
                    groups.push(group);
                } else {
                    groups[0].rows.append(&mut group.rows);
                }
            }
        }

        GitQLObject {
            titles: titles.to_vec(),
            groups,
        }
    }
}

fn select_clang_ast_objects(
    env: &mut Environment,
    path: &str,
    table: String,
    fields_names: &[String],
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    match table.as_str() {
        "functions" => select_functions(env, path, fields_names, titles, fields_values),
        _ => select_values(env, titles, fields_values),
    }
}

fn select_functions(
    env: &mut Environment,
    path: &str,
    fields_names: &[String],
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let mut rows: Vec<Row> = vec![];

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    let ast_functions = clang_function_visitor::select_clang_functions(path);
    for function in ast_functions.iter() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

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

            if field_name == "has_template" {
                values.push(Value::Boolean(function.has_template));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(Group { rows })
}
