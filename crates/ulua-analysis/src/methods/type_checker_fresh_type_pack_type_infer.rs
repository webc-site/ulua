use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::{arc_as_mut::arc_as_mut, fresh_index::fresh_index},
  records::{
    free_type_pack::FreeTypePack, scope::Scope, type_checker::TypeChecker, type_level::TypeLevel,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl TypeChecker {
  /// C++ `freshType(sharedState, scope, ...)`：只读取 `scope->level`，
  /// 以 `&Scope` 入参避免调用方为传值克隆 `ScopePtr`（Arc 引用计数）。
  pub fn fresh_type_pack_scope_ptr(&mut self, scope: &Scope) -> TypePackId {
    self.fresh_type_pack_type_level(scope.level)
  }

  pub fn fresh_type_pack_type_level(&mut self, level: TypeLevel) -> TypePackId {
    unsafe {
      let module = arc_as_mut(self.expect_current_module());
      (*module).internal_types.add_type_pack_t(FreeTypePack {
        index: fresh_index(),
        level,
        scope: null_mut(),
        polarity: Polarity::None,
      })
    }
  }
}
