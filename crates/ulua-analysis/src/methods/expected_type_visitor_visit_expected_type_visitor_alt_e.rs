//! @interface-stub
use ulua_ast::records::ast_expr_index_expr::AstExprIndexExpr;

use crate::records::{
  expected_type_visitor::ExpectedTypeVisitor, generic_type_visitor::GenericTypeVisitorTrait,
  index_collector::IndexCollector, union_type::UnionType,
};

impl ExpectedTypeVisitor {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_index_expr(&mut self, expr: *mut AstExprIndexExpr) -> bool {
    unsafe {
      let expr_ref = &*expr;

      if let Some(&ty) = (*self.ast_types).find(&(expr_ref.expr as *const _)) {
        let mut ic = IndexCollector::new(self.arena);
        ic.traverse_type_id(ty);

        if ic.indexes.size() > 1 {
          let union = (*self.arena).add_type(UnionType {
            options: ic.indexes.take(),
          });
          self.apply_expected_type(union, expr_ref.index as *const _);
        } else if ic.indexes.size() == 1 {
          let first = ic.indexes.order[0];
          self.apply_expected_type(first, expr_ref.index as *const _);
        }
      }
    }

    true
  }
}
