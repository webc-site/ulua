use crate::{
  records::{ast_expr::AstExpr, ast_stat_function::AstStatFunction, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatFunction, StatFunction, |this, visitor| {
  // name/func 已句柄化（node_handle::Node）：可变借用沿 `&mut self` 逐级传递，
  // dispatch 走 safe 引用形态，全链路无裸指针；func 的 `cast::<AstExpr>` 仅改
  // 节点类型位（repr(C) 基址重合）。cpp visit 端即二子节点顺序下钻
  // （Ast.cpp:912/913）。
  ast_expr_visit_ref(this.name.get_mut(), visitor);
  ast_expr_visit_ref(this.func.cast::<AstExpr>().get_mut(), visitor);
});
