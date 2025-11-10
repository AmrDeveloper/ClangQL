Change Log
==========

Version 0.11.0 *(2025-11-10)*

- Migrate to GitQL SDK 0.41.0

Version 0.10.0 *(2025-03-26)*
-----------------------------

- Implement `m_template_function` function matcher.
- Implement `m_conversion_function` function matcher.
- Implement `m_function_decl` function matcher.
- Implement `m_function_def` function matcher.

Version 0.9.0 *(2025-01-12)*
-----------------------------

- Restructure the parser and data provider for speedup and fix memory issues.
- Implement `is_virtual`, `is_pure_virtual` and `is_method` functions.
- Implement `is_static`, `is_const` and `is_deleted` functions.
- Implement `m_function`, `m_virtual` and `m_pure_virtual` matchers functions.
- Implement `m_static`, `m_const`, `m_method` and `m_deleted` matchers functions.
- Implement `m_constructor` and `m_destructor` matchers functions.
- Implement `m_default_constructor`, `m_copy_destructor`, `m_move_destructor` matchers functions.
- Implement `m_converting_constructor` matchers functions.
- Implement `m_public`, `m_protected` and `m_private` matchers functions.
- Implement `m_oneof`, `m_allof` and `m_noneof` combine matchers functions.
- Implement Combine and, or and xor for Function Matcher.
- Create new GitQL Custom type to allow advanced analysis.
- Support script file mode.

Version 0.8.0 *(2025-01-06)*
-----------------------------

* Migrate GitQL SDK to 0.35.0

Version 0.7.0 *(2024-08-14)*
-----------------------------

* Migrate GitQL SDK to 0.25.0

Version 0.6.0 *(2024-06-12)*
-----------------------------

* Migrate GitQL SDK to 0.23.0

Version 0.5.0 *(2024-06-12)*
-----------------------------

* Migrate GitQL SDK to 0.22.0
* Support query `name` and `location` of union.

Version 0.4.0 *(2024-06-08)*
-----------------------------

* Migrate GitQL SDK to 0.20.0
* Remove lazy_static crate

Version 0.3.0 *(2024-05-10)*
-----------------------------

* Support query `name`, `is_struct`, `location` for classes.
* Support query `methods_count`, `fields_count` for class.
* Support query `bases_count` for class
* Speedup parsing functions.
* Support query struct and class info.
* Support query enum `name`, `constants_count` and `type_literal`.
* Support query `size` and `align` of class.

Version 0.2.0 *(2024-04-26)*
-----------------------------

* Support query `name`, `class_name`, `return_type` for methods in structs or classes.
* Support query `is_template` option in Function model.
* Support query `is_static`, `is_const` option in Function model.
* Support query `is_method`, `is_variadic` option in Function model.
* Support query `is_virtual`, `is_pure_virtual` option in Function model.
* Support query `access_modifier` option in Function model.
* Support query global variables `name`, `type` and `is_volatile`.

Version 0.1.0 *(2024-04-05)*
-----------------------------

* First release of ClangQL.
