use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_group::AstExprGroup, ast_node::AstNode,
    ast_visitor::AstVisitor,
  },
  rtti::ast_node_is,
  visit::ast_expr_visit_ref,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::is_literal::is_literal,
  records::{
    arena_handle::{alias, alias_ref},
    blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor,
  },
  type_aliases::type_id::TypeId,
};
impl AstVisitor for BlockedTypeInLiteralVisitor<'_> {
  fn visit_node(&mut self, _node: &mut AstNode) -> bool {
    self.visit_ast_node()
  }

  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    self.visit_ast_expr(node)
  }
}

/// 形参全为受检引用（`expr` 为约束构造期携带的 AST 调用点的独占借用、
/// `ast_types` 为模块级映射），契约由类型承载；函数体内 args 元素的
/// parse-arena 裸指针判型与出借统一经 `alias_ref`/`alias` 句柄门面收口，
/// 不再有 `unsafe` 块。
pub fn find_blocked_arg_types_in(
  expr: &mut AstExprCall,
  ast_types: &mut DenseHashMap<*const AstExpr, TypeId>,
) -> Vec<TypeId> {
  let mut to_block: Vec<TypeId> = Vec::new();
  let mut v = BlockedTypeInLiteralVisitor {
    ast_types,
    to_block: &mut to_block,
  };
  // cpp: tryDispatch(FunctionCheckConstraint) 直接解引用 `c.callSite`
  // （ConstraintSolver.cpp:1915）——args 元素为构造期保证非空存活的
  // AstExpr 槽位；is_literal/ast_node_is 仅做 class index 只读判别，
  // alias 出借的 `&mut` 喂引用门面全链路 safe；遍历期间访客是 to_block
  // 的唯一写者，单线程无别名冲突。
  for &arg in expr.args.iter() {
    if is_literal(arg) || ast_node_is::<AstExprGroup>(&alias_ref(arg).base) {
      ast_expr_visit_ref(alias(arg), &mut v);
    }
  }
  to_block
}
