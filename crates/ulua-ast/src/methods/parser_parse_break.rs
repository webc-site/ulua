use crate::records::{
  ast_array::AstArray, ast_stat::AstStat, ast_stat_break::AstStatBreak, parser::Parser,
};

impl Parser {
  pub fn parser_parse_break(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    self.next_lexeme(); // break

    if self
      .function_stack
      .last()
      .expect("Parser::new 构造期压入的顶层 chunk 底帧永不出栈，解析期间栈必非空")
      .loop_depth
      == 0
    {
      // cpp:790 report_stat_error 的 statements 携带新分配的 break 节点
      let node = self.alloc_stat(AstStatBreak::new(start));
      let statements = self.copy_initializer_list_t(&[node]);
      return self.report_stat_error(
        start,
        AstArray::EMPTY,
        statements,
        format_args!("break statement must be inside a loop"),
      );
    }

    self.alloc_stat(AstStatBreak::new(start))
  }
}
