### AST function node functions

|    Function     |     Parameters     | Return |             Description              |
| :-------------: | :----------------: | :----: | :----------------------------------: |
|   is_virtual    | (n : FunctionType) |  Bool  |   True if the function is virtual    |
| is_pure_virtual | (n : FunctionType) |  Bool  | True if the function is pure virtual |
|    is_method    | (n : FunctionType) |  Bool  |   True if the function is a method   |
|    is_static    | (n : FunctionType) |  Bool  |    True if the function is static    |
|    is_const     | (n : FunctionType) |  Bool  |    True if the function is const     |
|   is_deleted    | (n : FunctionType) |  Bool  |   True if the function is deleted    |