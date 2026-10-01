use crate::{
  records::{ast_stat_local::AstStatLocal, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit, ast_type_visit},
};

impl_visitable!(AstStatLocal, StatLocal, |this, visitor| {
  for &var_ptr in this.vars.iter() {
    // Safety: 元素指针按 arena 契约为 null 或指向存活 AstLocal；
    // as_ref 折叠判空，null 静默跳过（不 panic）。
    if let Some(var) = unsafe { var_ptr.as_ref() } {
      // Safety: annotation 为 null 或存活 AstType 节点；ast_type_visit 对 null 内部短路
      // （等价旧守卫，不 panic）。
      unsafe { ast_type_visit(var.annotation, visitor) };
    }
  }

  for &expr_ptr in this.values.iter() {
    // Safety: ast_expr_visit 接收 arena 中存活节点的裸指针（null 内部短路）。
    unsafe { ast_expr_visit(expr_ptr, visitor) };
  }
});
