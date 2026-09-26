use core::ptr::from_mut;

use ulua_ast::{
  functions::to_string_ast::to_str_binary as to_str,
  records::{
    ast_attr::AstAttr,
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_node::AstNode,
    ast_visitor::AstVisitor,
  },
  rtti::{AstNodePtr, ast_node_try_as_ptr},
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintComparisonPrecedence<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintComparisonPrecedence<'ctx> {
  fn visit_expr_binary(&mut self, node: &mut AstExprBinary) -> bool {
    self.visit_ast_expr_binary(from_mut(node))
  }

  // visit_node 沿用 trait 默认实现（返回 true）
  fn visit_attr(&mut self, _node: &mut AstAttr) -> bool {
    false
  }
}

// —— 原 methods/lint_comparison_precedence_is_comparison.rs ——
impl<'ctx> LintComparisonPrecedence<'ctx> {
  pub fn is_comparison(&self, op: AstExprBinaryOp) -> bool {
    matches!(
      op,
      AstExprBinaryOp::CompareNe
        | AstExprBinaryOp::CompareEq
        | AstExprBinaryOp::CompareLt
        | AstExprBinaryOp::CompareLe
        | AstExprBinaryOp::CompareGt
        | AstExprBinaryOp::CompareGe
    )
  }
}

// —— 原 methods/lint_comparison_precedence_is_equality.rs ——
impl<'ctx> LintComparisonPrecedence<'ctx> {
  pub fn is_equality(&self, op: AstExprBinaryOp) -> bool {
    matches!(op, AstExprBinaryOp::CompareNe | AstExprBinaryOp::CompareEq)
  }
}

// —— 原 methods/lint_comparison_precedence_is_not.rs ——
impl<'ctx> LintComparisonPrecedence<'ctx> {
  pub fn is_not(&self, node: *mut AstExpr) -> bool {
    let node: *mut AstNode = node.as_ast_node();
    // Safety: node 为 lint 遍历传入的 AST arena 存活 AstExpr 节点地址或 null
    // （arena bump 分配不移动，活过整个 lint 阶段；上方 as_ast_node 仅指针
    // 地址转换，不产生解引用）。ast_node_try_as_ptr 把判空与 class-index 判型
    // 折叠为一次操作：null/未命中返回 None，命中即动态类型确为 AstExprUnary，
    // repr(C) 单继承首字段基址重合保证引用重建合法；返回引用只读 op 字段，
    // arena 在其存活期内无并存可变借用。
    let unary = unsafe { ast_node_try_as_ptr::<AstExprUnary>(node) };
    unary.is_some_and(|expr| expr.op == AstExprUnaryOp::Not)
  }
}

// —— 原 methods/lint_comparison_precedence_process.rs ——
impl<'ctx> LintComparisonPrecedence<'ctx> {
  lint_stat_process!(LintComparisonPrecedence);
}

// —— 原 methods/lint_comparison_precedence_visit.rs ——
impl<'ctx> LintComparisonPrecedence<'ctx> {
  /// # Safety
  /// `node` 必须非空并指向 parser arena 中存活的 `AstExprBinary`（crate 内唯一
  /// 入口是 `AstVisitor::visit_expr_binary`，实参经 `from_mut` 取自分发给出的
  /// `&mut` 借用；其 `left`/`right` 由 parser 构造，恒为非空存活节点指针）。
  pub(crate) fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
    // Safety: node 存活（函数级契约），`op` 是纯值读的枚举字段，等价 cpp
    // `node->op`。
    let op = unsafe { (*node).op };
    if !self.is_comparison(op) {
      return true;
    }
    // Safety: 拷出左操作数的句柄值仅要求 node 此刻存活；操作数已句柄化恒非空
    // 且指向 arena 存活节点；is_not 为既有裸指针 API，经 as_ptr 桥接。
    let left = unsafe { (*node).left }.as_ptr();
    // Safety: 同 left——node 存活期内按值读出右操作数的句柄。
    let right = unsafe { (*node).right }.as_ptr();
    let left_is_not = self.is_not(left);
    let right_is_not = self.is_not(right);
    if left_is_not && !right_is_not {
      let op_str = to_str(op);
      if self.is_equality(op) {
        let opposite = if op == AstExprBinaryOp::CompareEq {
          "~="
        } else {
          "=="
        };
        emit_warning(
          self.context.get(),
          Code::ComparisonPrecedence,
          // Safety: node 存活，location 为按值读出的 (Position, Position) 纯
          // 数据。
          unsafe { (*node).base.base.location },
          format_args!(
            "not X {} Y is equivalent to (not X) {} Y; consider using X {} Y, or add parentheses to silence",
            op_str, op_str, opposite
          ),
        );
      } else {
        emit_warning(
          self.context.get(),
          Code::ComparisonPrecedence,
          // Safety: 纯值读存活节点的 location 字段（cpp `node->location`）。
          unsafe { (*node).base.base.location },
          format_args!(
            "not X {} Y is equivalent to (not X) {} Y; add parentheses to silence",
            op_str, op_str
          ),
        );
      }
    } else {
      // Safety: left 是 node 内 parser 构造的存活 `AstExpr` 指针；
      // `ast_node_try_as_ptr` 判空 + class index 后按 repr(C) 基址重合下转为
      // `&'static`，借用的是左操作数子树，与本帧持有的 `node`（父二元节点）
      // 及 `self` 状态位置不重叠，遍历期该子树无其他写路径（cpp
      // `left->as<AstExprBinary>()` 的 const 下转同款语义）。
      if let Some(left_binary) = unsafe { ast_node_try_as_ptr::<AstExprBinary>(left) } {
        let left_op = left_binary.op;
        if self.is_comparison(left_op) {
          let lop_str = to_str(left_op);
          let rop_str = to_str(op);
          if self.is_equality(left_op) || self.is_equality(op) {
            emit_warning(
              self.context.get(),
              Code::ComparisonPrecedence,
              // Safety: 存活节点 location 的按值读取。
              unsafe { (*node).base.base.location },
              format_args!(
                "X {} Y {} Z is equivalent to (X {} Y) {} Z; add parentheses to silence",
                lop_str, rop_str, lop_str, rop_str
              ),
            );
          } else {
            emit_warning(
              self.context.get(),
              Code::ComparisonPrecedence,
              // Safety: 存活节点 location 的按值读取（emit_warning 只读参数）。
              unsafe { (*node).base.base.location },
              format_args!(
                "X {} Y {} Z is equivalent to (X {} Y) {} Z; did you mean X {} Y and Y {} Z?",
                lop_str, rop_str, lop_str, rop_str, lop_str, rop_str
              ),
            );
          }
        }
      }
    }
    true
  }
}
