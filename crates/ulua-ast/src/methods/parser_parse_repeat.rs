use crate::records::{
  ast_node::AstNode, ast_stat::AstStat, ast_stat_repeat::AstStatRepeat, cst_node::CstNode,
  cst_stat_repeat::CstStatRepeat, lexeme::Type, location::Location, match_lexeme::MatchLexeme,
  parser::Parser, position::Position,
};

impl Parser {
  pub fn parse_repeat(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    let match_repeat = *self.lexer.current();
    self.next_lexeme(); // repeat

    let locals_begin = self.save_locals();

    self.function_stack.last_mut().unwrap().loop_depth += 1;

    let body = self.parse_block_no_scope();

    self.function_stack.last_mut().unwrap().loop_depth -= 1;

    let has_until =
      self.expect_match_end_and_consume(Type::RESERVED_UNTIL, &MatchLexeme::new(&match_repeat));
    unsafe {
      (*body).has_end = has_until;
    }
    let until_position = if has_until {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let cond = self.parse_expr_i32(0);

    self.restore_locals(locals_begin);

    let node = unsafe {
      (*self.allocator).alloc(AstStatRepeat::new(
        Location::new(start.begin, (*cond).base.location.end),
        cond,
        body,
        has_until,
      ))
    };

    if self.options.store_cst_data {
      let cst_node = unsafe { (*self.allocator).alloc(CstStatRepeat::new(until_position)) };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node as *mut AstStat
  }
}
