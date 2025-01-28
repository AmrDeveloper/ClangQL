### AST matchers functions

|         Function         | Parameters |     Return      |                  Description                   |
| :----------------------: | :--------: | :-------------: | :--------------------------------------------: |
|   m_template_function    |     ()     | FunctionMatcher |   Create Matcher to match template function    |
|        m_virtual         |     ()     | FunctionMatcher |    Create Matcher to match virtual function    |
|      m_pure_virtual      |     ()     | FunctionMatcher | Create Matcher to match pure virtual function  |
|         m_method         |     ()     | FunctionMatcher |         Create Matcher to match method         |
|         m_static         |     ()     | FunctionMatcher |    Create Matcher to match static function     |
|         m_const          |     ()     | FunctionMatcher |     Create Matcher to match const function     |
|        m_deleted         |     ()     | FunctionMatcher |    Create Matcher to match deleted function    |
|      m_constructor       |     ()     | FunctionMatcher |      Create Matcher to match constructor       |
|  m_default_constructor   |     ()     | FunctionMatcher |  Create Matcher to match default constructor   |
|    m_copy_constructor    |     ()     | FunctionMatcher |    Create Matcher to match copy constructor    |
|    m_move_constructor    |     ()     | FunctionMatcher |    Create Matcher to match move constructor    |
| m_converting_constructor |     ()     | FunctionMatcher | Create Matcher to match converting constructor |
|       m_destructor       |     ()     | FunctionMatcher | Create Matcher to match function is destructor |
|         m_public         |     ()     | FunctionMatcher |    Create Matcher to match public function     |
|       m_protected        |     ()     | FunctionMatcher |   Create Matcher to match protected function   |
|        m_private         |     ()     | FunctionMatcher |    Create Matcher to match private function    |

### Combine matchers functions

| Function |        Parameters        |     Return      |                            Description                             |
| :------: | :----------------------: | :-------------: | :----------------------------------------------------------------: |
| m_oneof  | (n : ...FunctionMatcher) | FunctionMatcher |   Create a matcher that returns true if any sub matcher is true    |
| m_allof  | (n : ...FunctionMatcher) | FunctionMatcher | Create a matcher that returns true if all of sub matcher are true  |
| m_noneof | (n : ...FunctionMatcher) | FunctionMatcher | Create a matcher that returns true if none of sub matcher are true |
