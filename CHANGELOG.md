Change Log
==========

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