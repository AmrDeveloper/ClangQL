## Tables structures

### Classes table

| Name          | Type      | Description                     |
| ------------- | --------- | ------------------------------- |
| name          | Text      | Class variable name             |
| is_struct     | Boolean   | True if it a struct declaration |
| bases_count   | Integer   | Number of bases for this class  |
| methods_count | Integer   | Number of methods declarations  |
| fields_count  | Integer   | Number of fields declarations   |
| source_loc    | SourceLoc | Source location of AST node     |

---

### Enums table

| Name            | Type      | Description                      |
| --------------- | --------- | -------------------------------- |
| name            | Text      | Enumeration name                 |
| constants_count | Integer   | Number of constants in this enum |
| type_literal    | Text      | Type literal for enum constants  |
| source_loc      | SourceLoc | Source location of AST node      |

---

### Unions table

| Name         | Type      | Description                   |
| ------------ | --------- | ----------------------------- |
| name         | Text      | Union name                    |
| size         | Integer   | The size of union in bits     |
| fields_count | Integer   | Number of fields declarations |
| source_loc   | SourceLoc | Source location of AST node   |

---

### Functions table

| Name         | Type         | Description                        |
| ------------ | ------------ | ---------------------------------- |
| name         | Text         | Function or Method name            |
| signature    | Text         | Parameters and return type literal |
| return_type  | Text         | Return type                        |
| ast_function | FunctionNode | AST node of the function           |
| source_loc   | SourceLoc    | Source location of AST node        |

---

### Globals table<

| Name        | Type      | Description                       |
| ----------- | --------- | --------------------------------- |
| name        | Text      | Global variable name              |
| type        | Text      | Global variable type literal      |
| is_volatile | Boolean   | True if variable type is volatile |
| source_loc  | SourceLoc | Source location of AST node       |
