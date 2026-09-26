use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_group::AstExprGroup, ast_node::AstNode,
    ast_visitor::AstVisitor,
  },
  rtti::ast_node_is,
  visit::ast_expr_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::is_literal::is_literal,
  records::blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor,
  type_aliases::type_id::TypeId,
};
impl AstVisitor for BlockedTypeInLiteralVisitor {
  fn visit_node(&mut self, _node: &mut AstNode) -> bool {
    self.visit_ast_node()
  }

  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    // SAFETY: dispatch 传入的 node 必指向存活 AstExpr，满足 inherent visit_ast_expr 的指针契约。
    unsafe { self.visit_ast_expr(from_mut(node)) }
  }
}

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_blocked_arg_types_in(
  expr: *mut AstExprCall,
  ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
) -> Vec<TypeId> {
  let mut to_block: Vec<TypeId> = Vec::new();
  let mut v = BlockedTypeInLiteralVisitor {
    ast_types,
    to_block: &mut to_block as *mut Vec<TypeId>,
  };
  // Safety: expr 是约束（FunctionCheckConstraint.callSite）构造期携带的非空 AstExprCall
  // 指针（cpp ConstraintSolver.cpp:1915 同款裸解引用），指向 parser arena 存活节点，
  // args 元素同为 arena 存活 AstExpr*（地址不移动）；is_literal/ast_node_is 仅做 class
  // index 只读判别；v 内 ast_types 为调用方传入的模块级映射裸指针、to_block 转指针后
  // 其 &mut 访问即结束，遍历期间访客是唯一写该 Vec 者，单线程无别名冲突。
  unsafe {
    for &arg in (*expr).args.iter() {
      if is_literal(arg as *const AstExpr) || ast_node_is::<AstExprGroup>(&(*arg).base) {
        ast_expr_visit(arg, &mut v);
      }
    }
  }
  to_block
}
