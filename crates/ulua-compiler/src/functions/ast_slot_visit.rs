//! arena 接线的 AST 槽位**写穿遍历**门面：把 cpp 的 `node->visit(visitor)` 两步
//! 样板（判空 + 裸指针借用）收口一处。
//!
//! 语义与 ulua-ast 的 `ast_expr_visit` / `ast_stat_visit` / `ast_type_visit` 逐位
//! 一致：null 槽早退（cpp 亦由 `AstX::visit` 的判空入口保证），非空槽按 class-index
//! 分发到类型化 hook。区别只在借用兑现方式——这里经 [`Node`] 地址句柄，
//! 「节点存活 + 本次遍历独占写权限」的 arena 契约已由 `node.rs` 两处 `unsafe`
//! 统一承担，故业务调用点不再出现 `unsafe`（review.md C 档收口）。
//!
//! 调用方前提与 cpp 完全相同：visitor 串行遍历、同层子树无重叠写权限。

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, ast_visitor::AstVisitor},
  visit::dispatch_node,
};

use crate::records::node::Node;

/// `expr->visit(visitor)`：null 早退，非空交 `dispatch_node` 做 class-index 分发。
#[inline]
pub(crate) fn ast_slot_visit_expr<V: AstVisitor + ?Sized>(slot: *mut AstExpr, visitor: &mut V) {
  ast_slot_visit_node(slot.cast(), visitor);
}

/// `stat->visit(visitor)` 的槽位形态。
#[inline]
pub(crate) fn ast_slot_visit_stat<V: AstVisitor + ?Sized>(slot: *mut AstStat, visitor: &mut V) {
  ast_slot_visit_node(slot.cast(), visitor);
}

/// 两者共用底座：句柄借用 `&mut AstNode` 后走中央分发器。
fn ast_slot_visit_node<V: AstVisitor + ?Sized>(slot: *mut AstNode, visitor: &mut V) {
  if slot.is_null() {
    return;
  }

  dispatch_node(Node::<AstNode>::from(slot).borrow_mut(), visitor);
}
