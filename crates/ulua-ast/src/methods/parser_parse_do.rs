use crate::records::{
  ast_node::AstNode, ast_stat::AstStat, cst_node::CstNode, cst_stat_do::CstStatDo, lexeme::Type,
  match_lexeme::MatchLexeme, parser::Parser, position::Position,
};

impl Parser {
  pub fn parse_do(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    let match_do = *self.lexer.current();
    self.next_lexeme(); // do

    let stats_start = self.lexer.current().location.begin;

    let body = self.parse_block();

    unsafe {
      (*body).base.base.location.begin = start.begin;
    }

    let end_location = self.lexer.current().location;
    let has_end =
      self.expect_match_end_and_consume(Type::RESERVED_END, &MatchLexeme::new(&match_do));
    unsafe {
      (*body).has_end = has_end;
    }
    if has_end {
      unsafe {
        (*body).base.base.location.end = end_location.end;
      }
    }

    if self.options.store_cst_data {
      let end_position = if has_end {
        end_location.begin
      } else {
        Position::missing()
      };
      let cst_node = unsafe { (*self.allocator).alloc(CstStatDo::new(stats_start, end_position)) };
      self
        .cst_node_map
        .try_insert(body as *mut AstNode, cst_node as *mut CstNode);
    }

    body as *mut AstStat
  }
}
