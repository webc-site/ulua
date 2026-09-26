use ulua_ast::methods::ast_stat_block_visit::ast_stat_block_visit;

use crate::{
  functions::{arc_as_mut::arc_as_mut, freeze::freeze, unfreeze::unfreeze},
  records::{
    arena_handle::Handle, expected_type_visitor::ExpectedTypeVisitor, frontend::Frontend,
    module::Module, source_module::SourceModule,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl Frontend {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn populate_expected_types(
    &self,
    source_module: &SourceModule,
    module: *mut Module,
    root_scope: &ScopePtr,
  ) {
    unsafe {
      let was_frozen = (*module).internal_types.types.is_frozen()
        || (*module).internal_types.type_packs.is_frozen();
      if was_frozen {
        unfreeze(&mut (*module).internal_types);
      }

      let mut visitor = ExpectedTypeVisitor::new(
        &mut (*module).ast_types,
        &mut (*module).ast_expected_types,
        &mut (*module).ast_resolved_types,
        &mut (*module).ast_overload_resolved_types,
        Handle::from_mut(&mut (*module).internal_types),
        self.builtin_types_handle(),
        arc_as_mut(root_scope),
      );

      ast_stat_block_visit(&mut *source_module.root, &mut visitor);

      if was_frozen {
        freeze(&mut (*module).internal_types);
      }
    }
  }
}
