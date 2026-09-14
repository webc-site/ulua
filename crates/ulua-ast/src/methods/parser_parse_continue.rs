use crate::records::{
  ast_array::AstArray, ast_stat::AstStat, ast_stat_continue::AstStatContinue, location::Location,
  parser::Parser,
};

impl Parser {
  pub fn parser_parse_continue(&mut self, start: &Location) -> *mut AstStat {
    if self.function_stack.last().unwrap().loop_depth == 0 {
      return self.report_stat_error(
        *start,
        AstArray::default(),
        AstArray::default(),
        format_args!("continue statement must be inside a loop"),
      ) as *mut AstStat;
    }

    // note: the token is already parsed for us!
    let continue_stat = AstStatContinue::new(*start);
    unsafe { (*self.allocator).alloc(continue_stat) as *mut AstStat }
  }
}
