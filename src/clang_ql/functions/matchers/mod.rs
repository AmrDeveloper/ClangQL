use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;

mod function;

#[inline(always)]
pub(crate) fn register_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    function::register_function_matchers_functions(map);
}

#[inline(always)]
pub(crate) fn register_matchers_signatures(map: &mut HashMap<&'static str, Signature>) {
    function::register_function_matchers_signatures(map);
}
