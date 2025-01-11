use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_std::standard::standard_function_signatures;
use gitql_std::standard::standard_functions;

mod ast;

#[inline(always)]
pub fn clang_ql_functions() -> &'static HashMap<&'static str, StandardFunction> {
    static HASHMAP: OnceLock<HashMap<&'static str, StandardFunction>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = standard_functions().to_owned();
        ast::register_ast_functions(&mut map);
        map
    })
}

#[inline(always)]
pub fn clang_ql_functions_signatures() -> HashMap<&'static str, Signature> {
    let mut map = standard_function_signatures().to_owned();
    ast::register_ast_signatures(&mut map);
    map
}
