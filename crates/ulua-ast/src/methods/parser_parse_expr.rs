//! Source: `Ast/src/Parser.cpp:3530`
//!
//! The precedence-climbing expression parser (`Parser::parseExpr(unsigned limit)`).
//! Operands come from `parse_assertion_expr` (and recursively from here for unary
//! operators); binary operators expand while their left priority exceeds `limit`,
//! right-associative ops (`^`, `..`) recurse at one-lower priority. All operand
//! and operator helpers (`parse_unary_op`/`parse_binary_op`/`check_*_confusables`)
//! are already ported; only this driver was a stub.

use crate::records::{
  ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_unary::AstExprUnary,
  ast_node::AstNode, binary_op_priority::BinaryOpPriority, cst_expr_op::CstExprOp,
  cst_node::CstNode, location::Location, parser::Parser,
};

const fn prio(left: u8, right: u8) -> BinaryOpPriority {
  BinaryOpPriority { left, right }
}

/// Per-operator {left, right} binding priorities, indexed by
/// `AstExprBinaryOp as usize` (same order as the C++ table).
///
/// 模块级 `const`：编译期求值进 .rodata，取代原先函数体内每次
/// `parse_expr` 调用都在栈上重建的 16 项表。
const BINARY_PRIORITY: [BinaryOpPriority; 16] = [
  prio(6, 6),  // +
  prio(6, 6),  // -
  prio(7, 7),  // *
  prio(7, 7),  // /
  prio(7, 7),  // //
  prio(7, 7),  // %
  prio(10, 9), // ^ (right associative)
  prio(5, 4),  // .. (right associative)
  prio(3, 3),  // ~=
  prio(3, 3),  // ==
  prio(3, 3),  // <
  prio(3, 3),  // <=
  prio(3, 3),  // >
  prio(3, 3),  // >=
  prio(2, 2),  // and
  prio(1, 1),  // or
];
const UNARY_PRIORITY: u32 = 8;

impl Parser {
  pub fn parse_expr(&mut self, limit: u32) -> *mut AstExpr {
    let start = self.lexer.current().location;

    // C++ parseExpr enforces the recursion limit here (the port dropped it
    // entirely, so deeply-nested expressions never hit LuauRecursionLimit).
    let old_recursion_count = self.recursion_counter;
    // this handles recursive calls to parse_sub_expr/parse_expr
    self.increment_recursion_counter("expression");

    let curr = *self.lexer.current();
    let mut uop = self.parse_unary_op(&curr);
    if uop.is_none() {
      uop = self.check_unary_confusables();
    }

    let mut expr: *mut AstExpr;
    if let Some(uop) = uop {
      let op_position = self.lexer.current().location.begin;
      self.next_lexeme();
      let subexpr = self.parse_expr(UNARY_PRIORITY);
      let end = unsafe { (*subexpr).base.location.end };
      let node = unsafe {
        (*self.allocator).alloc(AstExprUnary::new(
          Location::new(start.begin, end),
          uop,
          subexpr,
        ))
      };
      self.attach_cst(node, |alloc| alloc.alloc(CstExprOp::new(op_position)));
      expr = node as *mut AstExpr;
    } else {
      expr = self.parse_assertion_expr();
    }

    // Expand while operators have priority higher than `limit`.
    let curr = *self.lexer.current();
    let mut op = self.parse_binary_op(&curr);
    if op.is_none() {
      op = self.check_binary_confusables(&BINARY_PRIORITY, limit);
    }

    while let Some(o) = op {
      if BINARY_PRIORITY[o as usize].left as u32 <= limit {
        break;
      }
      let op_position = self.lexer.current().location.begin;
      self.next_lexeme();
      let next = self.parse_expr(BINARY_PRIORITY[o as usize].right as u32);
      let end = unsafe { (*next).base.location.end };
      let node = unsafe {
        (*self.allocator).alloc(AstExprBinary::new(
          Location::new(start.begin, end),
          o,
          expr,
          next,
        ))
      };
      self.attach_cst(node, |alloc| alloc.alloc(CstExprOp::new(op_position)));
      expr = node as *mut AstExpr;
      let curr = *self.lexer.current();
      op = self.parse_binary_op(&curr);
      if op.is_none() {
        op = self.check_binary_confusables(&BINARY_PRIORITY, limit);
      }

      // note: while the parser isn't recursive here, we're generating
      // recursive structures of unbounded depth
      self.increment_recursion_counter("expression");
    }

    self.recursion_counter = old_recursion_count;

    expr
  }
}
