use ulua_ast::{
  records::ast_stat_for_in::AstStatForIn,
  visit::{ast_expr_visit, ast_stat_block_visit},
};

use crate::records::lint_global_local::LintGlobalLocal;
impl LintGlobalLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_for_in(&mut self, node: *mut AstStatForIn) -> bool {
    let reset_to_false = self.set_conditional_execution();

    let values = unsafe { (*node).values };
    for &value in values.as_slice() {
      unsafe {
        ast_expr_visit(value, self);
      }
    }

    unsafe {
      ast_stat_block_visit(&*(*node).body, self);
    }

    if reset_to_false {
      self
        .function_stack
        .last_mut()
        .unwrap()
        .conditional_execution = false;
    }

    false
  }
}
