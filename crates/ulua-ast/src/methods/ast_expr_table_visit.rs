use crate::{
  records::{ast_expr_table::AstExprTable, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstExprTable, ExprTable, |this, visitor| {
  for item in this.items.iter() {
    // Safety: item.key 为 arena 分配的键表达式节点或 null（Record 才有键，其它 kind 为
    // null）；ast_expr_visit 对 null 内部短路（等价旧守卫），与 self 同 arena 存活、
    // 地址不移动，遍历期单线程独占写穿。
    unsafe {
      ast_expr_visit(item.key, visitor);
    }

    // Safety: item.value 是表构造必填槽位，parser 总以 arena 节点填充（null 也被 dispatch 短路）；存活与独占前提同上。
    unsafe {
      ast_expr_visit(item.value, visitor);
    }
  }
});
