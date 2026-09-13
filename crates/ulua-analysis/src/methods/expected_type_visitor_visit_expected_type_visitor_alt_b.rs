use core::cmp::min;

use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::records::expected_type_visitor::ExpectedTypeVisitor;
impl ExpectedTypeVisitor {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local(&mut self, stat: *mut AstStatLocal) -> bool {
    unsafe {
      let stat_ref = &*stat;
      let max_idx = min(stat_ref.vars.size, stat_ref.values.size);

      for idx in 0..max_idx {
        let var = *stat_ref.vars.data.add(idx);
        let value = *stat_ref.values.data.add(idx);

        if let Some(&annot) = (*self.ast_resolved_types).find(&((*var).annotation as *const _)) {
          self.apply_expected_type(annot, value as *const _);
        }
      }
    }

    true
  }
}
