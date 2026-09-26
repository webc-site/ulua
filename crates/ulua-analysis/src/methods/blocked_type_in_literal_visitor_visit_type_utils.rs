use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_group::AstExprGroup},
  rtti::ast_node_is,
};

use crate::{
  functions::{follow_type, get_type, is_literal::is_literal},
  records::{
    blocked_type::BlockedType, blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor,
  },
};

impl BlockedTypeInLiteralVisitor {
  pub fn visit_ast_node(&mut self) -> bool {
    false
  }

  /// # Safety
  /// `e` 须指向本次遍历存活的 parse-arena `AstExpr`（非空、对齐、地址在 arena 释放前不移动）；
  /// `self.ast_types`/`self.to_block` 由驱动方注入，指向比本遍历长寿的存活容器，本函数按契约对二者
  /// 做只读查询与独占 push。单线程、无并发写。对应 C++ `bool BlockedTypeInLiteralVisitor::visit(AstExpr* e)`
  /// (`cpp/Analysis/src/TypeUtils.cpp:490`)。
  pub unsafe fn visit_ast_expr(&mut self, e: *mut AstExpr) -> bool {
    // SAFETY: ast_types/to_block 由驱动方注入，指向存活的容器。
    unsafe {
      if let Some(&ty) = (*self.ast_types).find(&(e as *const AstExpr)) {
        let followed = follow_type::follow(ty);
        if get_type::get::<BlockedType>(followed).is_some() {
          (*self.to_block).push(ty);
        }
      }
    }
    // SAFETY: e 指向 AST arena 内存活的 repr(C) 节点，判空与判型一并完成。
    is_literal(e as *const AstExpr)
      || unsafe {
        e.as_ref()
          .is_some_and(|er| ast_node_is::<AstExprGroup>(&er.base))
      }
  }
}
