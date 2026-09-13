use core::{
  mem::{replace, take},
  ptr::null_mut,
};

use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{constraint_generator::ConstraintGenerator, constraint_set::ConstraintSet};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn run(&mut self, block: *mut AstStatBlock) -> ConstraintSet {
    unsafe { self.visit_module_root(block) };

    ConstraintSet {
      root_scope: self.root_scope,
      constraints: take(&mut self.constraints),
      free_types: take(&mut self.free_types),
      scope_to_function: replace(&mut self.scope_to_function, DenseHashMap::new(null_mut())),
      errors: take(&mut self.errors),
    }
  }
}
