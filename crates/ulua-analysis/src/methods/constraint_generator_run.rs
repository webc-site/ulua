use core::mem::take;

use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{constraint_generator::ConstraintGenerator, constraint_set::ConstraintSet},
};
impl ConstraintGenerator {
  pub(crate) fn run(&mut self, block: *mut AstStatBlock) -> ConstraintSet {
    unsafe { self.visit_module_root(block) };

    ConstraintSet {
      // `visit_module_root` 已置位 root_scope（C++ `NotNull` 语义），向下沉为
      // 写句柄交给 ConstraintSet/求解器（其字段 Option 化属后续波段）。
      root_scope: arc_as_mut(self.root()),
      constraints: take(&mut self.constraints),
      free_types: take(&mut self.free_types),
      // take 与原 `replace(.., DenseHashMap::default())` 逐位等价：
      // default 门面哨兵即 null 键。
      scope_to_function: take(&mut self.scope_to_function),
      errors: take(&mut self.errors),
    }
  }
}
