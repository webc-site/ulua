use crate::{
  records::{
    ast_expr::AstExpr, ast_stat_local_function::AstStatLocalFunction, ast_visitor::AstVisitor,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatLocalFunction, StatLocalFunction, |this, visitor| {
  // func 已句柄化（node_handle::Node）：`cast` 仅改节点类型位（repr(C) 基址重合），
  // 可变借用沿 `&mut self` 传递，dispatch 走 safe 引用形态，全链路无裸指针；
  // cpp visit 端只下钻 func（Ast.cpp:929），name 是 AstLocal 而非子节点。
  ast_expr_visit_ref(this.func.cast::<AstExpr>().get_mut(), visitor);
});
