### AST matchers functions

|         Function         | Parameters |     Return      |                            Description                            |
| :----------------------: | :--------: | :-------------: | :---------------------------------------------------------------: |
|        m_virtual         |     ()     | FunctionMatcher |        Create Matcher to check if the function is virtual         |
|      m_pure_virtual      |     ()     | FunctionMatcher |      Create Matcher to check if the function is pure virtual      |
|         m_method         |     ()     | FunctionMatcher |        Create Matcher to check if the function is a method        |
|         m_static         |     ()     | FunctionMatcher |         Create Matcher to check if the function is static         |
|         m_const          |     ()     | FunctionMatcher |         Create Matcher to check if the function is const          |
|        m_deleted         |     ()     | FunctionMatcher |        Create Matcher to check if the function is deleted         |
|      m_constructor       |     ()     | FunctionMatcher |      Create Matcher to check if the function is constructor       |
|  m_default_constructor   |     ()     | FunctionMatcher |  Create Matcher to check if the function is default constructor   |
|    m_copy_constructor    |     ()     | FunctionMatcher |    Create Matcher to check if the function is copy constructor    |
|    m_move_constructor    |     ()     | FunctionMatcher |    Create Matcher to check if the function is move constructor    |
| m_converting_constructor |     ()     | FunctionMatcher | Create Matcher to check if the function is converting constructor |
|       m_destructor       |     ()     | FunctionMatcher |       Create Matcher to check if the function is destructor       |