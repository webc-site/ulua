//! Source: `Ast/src/Parser.cpp:559`
//!
//! Faithful port of `Parser::parseIf` — `if exp then block {elseif/else} end`.
//! Chained `elseif` recurses through `parse_if` (guarded by the recursion
//! counter); a trailing `else` is parsed as a block whose begin is snapped to
//! the `else` keyword. `hasEnd` is propagated onto whichever block terminates
//! the chain so the pretty-printer / CST consumers see the real end span.

use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::{node_opt, slot_ref},
  records::{
    ast_stat::AstStat,
    ast_stat_block::AstStatBlock,
    ast_stat_if::AstStatIf,
    location::Location,
    match_lexeme::MatchLexeme,
    node_handle::{Node, OptNode},
    parser::Parser,
  },
  rtti::ast_node_try_as_ptr_mut,
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

    let thenbody = self.parse_block();

    let mut elsebody: Option<NonNull<AstStat>> = None;
    let end;
    let mut else_location: Option<Location> = None;

    if self.lexer.current().r#type == Type::RESERVED_ELSEIF {
      // Safety: `thenbody` 由 parse_block 返回、指向 arena 存活的 `AstStatBlock`（非空、bump 地址稳定、
      // 此刻仅本指针可达），写入其 `has_end` 字段不与其它借用重叠（parser 独占该 arena 节点）。
      unsafe {
        (*thenbody).has_end = true;
      }
      let old_recursion_count = self.recursion_counter;
      self.increment_recursion_counter("elseif");
      else_location = Some(self.lexer.current().location);
      elsebody = node_opt(self.parse_if());
      // Safety: `elsebody` 由递归 parse_if 产出、node_opt 折叠后即非空 arena 存活 `AstStat`（expect 兜底
      // alloc 恒非空不变式）；as_ptr 后仅读基类 location，无 `&mut`。
      let else_ptr =
        elsebody.expect("parse_if 产出 arena 存活非空 AstStat（alloc_stat 恒非空，失败中止）");
      end = slot_ref(else_ptr.as_ptr()).base.location;
      self.recursion_counter = old_recursion_count;
    } else {
      let mut match_then_else = match_then;

      if self.lexer.current().r#type == Type::RESERVED_ELSE {
        // Safety: 同上，`thenbody` 为 parser 独占存活的 arena `AstStatBlock`，写 `has_end` 合法。
        unsafe {
          (*thenbody).has_end = true;
        }
        else_location = Some(self.lexer.current().location);
        match_then_else = *self.lexer.current();
        self.next_lexeme();

        let else_block = self.parse_block();
        // Safety: `else_block` 由 parse_block 返回、指向 arena 存活的 `AstStatBlock`（非空、地址稳定），
        // 写入其基类 location.begin 不与其它借用重叠（parser 独占该 arena 节点）。
        unsafe {
          (*else_block).base.base.location.begin = match_then_else.location.end;
        }
        elsebody = node_opt(else_block.cast::<AstStat>());
      }

      end = self.lexer.current().location;

      let has_end =
        self.expect_match_end_and_consume(Type::RESERVED_END, &MatchLexeme::new(&match_then_else));

      if let Some(else_ptr) = elsebody {
        // Safety: `else_ptr` 经 node_opt 折叠必非空且为 arena 存活 `AstStat`；try_as_ptr_mut 依 class_index
        // 判定，未命中返回 None，命中即 repr(C) 基址重合下转为 `AstStatBlock`，此刻 parser 独占该 arena。
        if let Some(else_block) =
          unsafe { ast_node_try_as_ptr_mut::<AstStatBlock>(else_ptr.as_ptr()) }
        {
          else_block.has_end = has_end;
        }
      } else {
        // Safety: 同上，`thenbody` 为 parser 独占存活的 arena `AstStatBlock`，写 `has_end` 合法。
        unsafe {
          (*thenbody).has_end = has_end;
        }
      }
    }

    self.alloc_stat(AstStatIf::new(
      Location::new(start.begin, end.end),
      // 槽位收进 arena 句柄：cond/thenbody 出自 alloc(恒非空)门面，elsebody 的
      // `Option<NonNull>` 即非空性证明,交接不再落回裸指针。
      Node::from_raw(cond),
      Node::from_raw(thenbody),
      OptNode::from_non_null(elsebody),
      then_location,
      else_location,
    ))
  }
}
