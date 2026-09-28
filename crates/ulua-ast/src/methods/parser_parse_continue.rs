use crate::records::{
  ast_array::AstArray, ast_stat::AstStat, ast_stat_continue::AstStatContinue, location::Location,
  parser::Parser,
};

impl Parser {
  pub fn parser_parse_continue(&mut self, start: &Location) -> *mut AstStat {
    if self
      .function_stack
      .last()
      .expect("Parser::new 构造期压入的顶层 chunk 底帧永不出栈，解析期间栈必非空")
      .loop_depth
      == 0
    {
      // cpp:799 错误路径 statements 携带新分配的 continue 节点
      let node = self.alloc_stat(AstStatContinue::new(*start));
      let statements = self.copy_initializer_list_t(&[node]);
      return self.report_stat_error(
        *start,
        AstArray::EMPTY,
        statements,
        format_args!("continue statement must be inside a loop"),
      );
    }

    // note: the token is already parsed for us!
    let continue_stat = AstStatContinue::new(*start);
    self.alloc_stat(continue_stat)
  }
}
