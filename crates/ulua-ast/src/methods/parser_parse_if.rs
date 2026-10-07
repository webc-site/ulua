//! Source: `Ast/src/Parser.cpp:559`
//!
//! Faithful port of `Parser::parseIf` — `if exp then block {elseif/else} end`.
//! Chained `elseif` recurses through `parse_if` (guarded by the recursion
//! counter); a trailing `else` is parsed as a block whose begin is snapped to
//! the `else` keyword. `hasEnd` is propagated onto whichever block terminates
//! the chain so the pretty-printer / CST consumers see the real end span.

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_stat::AstStat,
    ast_stat_block::AstStatBlock,
    ast_stat_if::AstStatIf,
    location::Location,
    match_lexeme::MatchLexeme,
    node_handle::{Node, OptNode},
    parser::Parser,
  },
  rtti::ast_node_try_as_mut,
};

impl Parser {
  pub fn parse_if(&mut self) -> *mut AstStat {
    let start = self.lexer.current().location;

    self.next_lexeme(); // if / elseif

    let cond = self.parse_expr(0);

    let match_then = *self.lexer.current();
    let mut then_location: Option<Location> = None;
    if self.expect_and_consume_type(Type::RESERVED_THEN, "if statement") {
      then_location = Some(match_then.location);
    }

    // thenbody/elsebody 收进 arena 句柄：parse_block/parse_if 产出恒非空
    //（alloc 恒非空，失败中止），has_end 与基类 location 的写穿经 DerefMut
    // 安全完成（独占性由 parser 对 arena 的独占与借用检查器共同保证）。
    let mut thenbody = Node::from_raw(self.parse_block());

    let mut elsebody = OptNode::<AstStat>::default();
    let end;
    let mut else_location: Option<Location> = None;

    if self.lexer.current().r#type == Type::RESERVED_ELSEIF {
      thenbody.has_end = true;
      let old_recursion_count = self.recursion_counter;
      self.increment_recursion_counter("elseif");
      else_location = Some(self.lexer.current().location);
      elsebody = OptNode::from_ptr(self.parse_if());
      end = elsebody
        .get()
        .expect("parse_if 产出 arena 存活非空 AstStat（alloc_stat 恒非空，失败中止）")
        .base
        .location;
      self.recursion_counter = old_recursion_count;
    } else {
      let mut match_then_else = match_then;

      if self.lexer.current().r#type == Type::RESERVED_ELSE {
        thenbody.has_end = true;
        else_location = Some(self.lexer.current().location);
        match_then_else = *self.lexer.current();
        self.next_lexeme();

        let mut else_block = Node::from_raw(self.parse_block());
        else_block.base.base.location.begin = match_then_else.location.end;
        elsebody = OptNode::from_ptr(else_block.cast::<AstStat>().as_ptr());
      }

      end = self.lexer.current().location;

      let has_end =
        self.expect_match_end_and_consume(Type::RESERVED_END, &MatchLexeme::new(&match_then_else));

      if let Some(mut else_stat) = elsebody.to_option() {
        // cpp `elseBody->as<AstStatBlock>()`：判型与下转走安全门面
        // ast_node_try_as_mut（依 class_index 判定，未命中返回 None，命中即
        // repr(C) 基址重合下转），可变性沿句柄 get_mut 的独占借用继承。
        if let Some(else_block) = ast_node_try_as_mut::<AstStatBlock>(else_stat.get_mut()) {
          else_block.has_end = has_end;
        }
      } else {
        thenbody.has_end = has_end;
      }
    }

    self.alloc_stat(AstStatIf::new(
      Location::new(start.begin, end.end),
      // cond 出自 parse_expr 的 alloc(恒非空)门面，建槽交接不再落回裸指针判空。
      Node::from_raw(cond),
      thenbody,
      elsebody,
      then_location,
      else_location,
    ))
  }
}
