//! Source: `cpp/Ast/src/Parser.cpp` 的 `Parser::parseExpr`（行号随上游漂移，按函数名
//! 检索；当前约 3631）
//!
//! The precedence-climbing expression parser (`Parser::parseExpr(unsigned limit)`).
//! Operands come from `parse_assertion_expr` (and recursively from here for unary
//! operators); binary operators expand while their left priority exceeds `limit`,
//! right-associative ops (`^`, `..`) recurse at one-lower priority. All operand
//! and operator helpers (`parse_unary_op`/`parse_binary_op`/`check_*_confusables`)
//! are already ported; only this driver was a stub.

use crate::{
  functions::optional_node::slot_ref,
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_unary::AstExprUnary,
    binary_op_priority::BinaryOpPriority,
    cst_expr_op::CstExprOp,
    location::Location,
    node_handle::Node,
    parser::Parser,
  },
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
  /// 双目运算符探测收口：先试常规运算符，未命中再探测 `&&`/`||`/`!=`
  /// 易混淆序列。C++ 在 parseExpr 中将这两步连写两遍，此处合并为一处。
  fn probe_binary_op(&mut self, limit: u32) -> Option<AstExprBinaryOp> {
    let curr = *self.lexer.current();
    self
      .parse_binary_op(&curr)
      .or_else(|| self.check_binary_confusables(&BINARY_PRIORITY, limit))
  }

  pub fn parse_expr(&mut self, limit: u32) -> *mut AstExpr {
    let start = self.lexer.current().location;

    // cpp Parser.cpp:3631 在 parseExpr 入口递增/回溯递归计数，深度嵌套据此
    // 触发 LuauRecursionLimit 报错。
    let old_recursion_count = self.recursion_counter;
    // this handles recursive calls to parse_sub_expr/parse_expr
    self.increment_recursion_counter("expression");

    let curr = *self.lexer.current();
    let uop = self
      .parse_unary_op(&curr)
      .or_else(|| self.check_unary_confusables());

    let mut expr: *mut AstExpr;
    if let Some(uop) = uop {
      let op_position = self.lexer.current().location.begin;
      self.next_lexeme();
      let subexpr = self.parse_expr(UNARY_PRIORITY);
      // `subexpr` 是 parse_expr 返回、parser 保证非空的 arena 存活节点；仅读基类 location。
      let end = slot_ref(subexpr).base.location.end;
      // subexpr 出自 parse_expr 的 arena 分配（cpp 语义错误路径亦返回错误节点，恒非空）。
      let node = self.alloc_expr(AstExprUnary::new(
        Location::new(start.begin, end),
        uop,
        Node::from_raw(subexpr),
      ));
      self.attach_cst(node, |alloc| alloc.alloc(CstExprOp::new(op_position)));
      expr = node;
    } else {
      expr = self.parse_assertion_expr();
    }

    // Expand while operators have priority higher than `limit`.
    let mut op = self.probe_binary_op(limit);
    while let Some(o) = op {
      if BINARY_PRIORITY[o as usize].left as u32 <= limit {
        break;
      }
      let op_position = self.lexer.current().location.begin;
      self.next_lexeme();
      // read sub-expression with higher priority
      let next = self.parse_expr(BINARY_PRIORITY[o as usize].right as u32);
      // `next` 是 parse_expr 返回、parser 保证非空的 arena 存活节点；仅读基类 location。
      let end = slot_ref(next).base.location.end;
      // expr/next 均出自 parse_expr 线的 arena 分配（cpp 语义错误路径亦返回节点，恒非空）。
      let node = self.alloc_expr(AstExprBinary::new(
        Location::new(start.begin, end),
        o,
        Node::from_raw(expr),
        Node::from_raw(next),
      ));
      self.attach_cst(node, |alloc| alloc.alloc(CstExprOp::new(op_position)));
      expr = node;

      op = self.probe_binary_op(limit);

      // note: while the parser isn't recursive here, we're generating
      // recursive structures of unbounded depth
      self.increment_recursion_counter("expression");
    }

    self.recursion_counter = old_recursion_count;

    expr
  }
}
