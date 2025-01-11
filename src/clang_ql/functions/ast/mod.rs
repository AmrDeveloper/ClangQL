mod functions;
use functions::register_ast_function_functions;
use functions::register_ast_function_signatures;

use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;

#[inline(always)]
pub(crate) fn register_ast_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    register_ast_function_functions(map);
}

#[inline(always)]
pub(crate) fn register_ast_signatures(map: &mut HashMap<&'static str, Signature>) {
    register_ast_function_signatures(map);
}
