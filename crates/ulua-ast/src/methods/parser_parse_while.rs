//! Source: `Ast/src/Parser.cpp:620`
//!
//! Faithful port of `Parser::parseWhile` — `while exp do block end`. Tracks
//! loop depth on the current function frame around the body parse and records
//! the `do` keyword location for CST-free reconstruction.

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_stat::AstStat, ast_stat_while::AstStatWhile, location::Location, match_lexeme::MatchLexeme,
    node_handle::Node, parser::Parser,
  },
};

impl Parser {
  pub fn parse_while(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    self.next_lexeme(); // while

    let cond = self.parse_expr(0);

    let match_do = *self.lexer.current();
    let has_do = self.expect_and_consume_type(Type::RESERVED_DO, "while loop");

    // loop_depth 升降与 end 收尾均为循环骨架，见 parser_loop_body
    let body = self.with_loop_depth(|p| p.parse_block());
    // Safety: `body` 由 parse_block 返回 arena 中存活的非空 `*mut AstStatBlock`。
    let end = unsafe { self.consume_loop_end(&MatchLexeme::new(&match_do), body) };

    self.alloc_stat(AstStatWhile::new(
      Location::new(start.begin, end.end),
      // 槽位收进 arena 句柄：cond/body 皆出自 alloc(恒非空)门面。
      Node::from_raw(cond),
      Node::from_raw(body),
      has_do,
      match_do.location,
    ))
  }
}
