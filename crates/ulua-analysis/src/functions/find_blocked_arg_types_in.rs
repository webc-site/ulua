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
impl AstVisitor for BlockedTypeInLiteralVisitor<'_> {
  fn visit_node(&mut self, _node: &mut AstNode) -> bool {
    self.visit_ast_node()
  }

  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    self.visit_ast_expr(node)
  }
}

/// 形参全为受检引用（`expr` 为约束构造期携带的 AST 调用点、`ast_types` 为模块级
/// 映射的独占借用），契约由类型承载；函数体内 `unsafe` 仅为 args 元素的
/// parse-arena 裸指针判型，见对应 `// SAFETY` 注释。
pub fn find_blocked_arg_types_in(
  expr: &AstExprCall,
  ast_types: &mut DenseHashMap<*const AstExpr, TypeId>,
) -> Vec<TypeId> {
  let mut to_block: Vec<TypeId> = Vec::new();
  let mut v = BlockedTypeInLiteralVisitor {
    ast_types,
    to_block: &mut to_block,
  };
  // Safety: expr 是约束（FunctionCheckConstraint.callSite）构造期携带的 AstExprCall
  // 引用（cpp ConstraintSolver.cpp:1915 同款直解引用），指向 parser arena 存活节点，
  // args 元素同为 arena 存活 AstExpr 指针（地址不移动）；is_literal/ast_node_is 仅做
  // class index 只读判别；遍历期间访客是 to_block 的唯一写者，单线程无别名冲突。
  unsafe {
    for &arg in expr.args.iter() {
      if is_literal(arg) || ast_node_is::<AstExprGroup>(&(*arg).base) {
        ast_expr_visit(arg, &mut v);
      }
    }
  }
  to_block
}
