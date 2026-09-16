use crate::records::{
  ast_array::AstArray, ast_stat::AstStat, ast_stat_break::AstStatBreak, parser::Parser,
};

impl Parser {
  pub fn parser_parse_break(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    self.next_lexeme(); // break

    if self.function_stack.last().unwrap().loop_depth == 0 {
      return self.report_stat_error(
        start,
        AstArray::default(),
        AstArray::default(),
        format_args!("break statement must be inside a loop"),
      ) as *mut AstStat;
    }

    unsafe { (*self.allocator).alloc(AstStatBreak::new(start)) as *mut AstStat }
  }
}
