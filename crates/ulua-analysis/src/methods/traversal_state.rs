//! `traversal_state` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::{dfint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{follow_type, follow_type_pack},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, traversal_state::TraversalState,
    type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

impl TraversalState {
  pub fn check_invariants(&mut self) -> bool {
    self.too_long()
  }
}

impl TraversalState {
  pub fn too_long(&mut self) -> bool {
    self.steps += 1;
    self.steps > dfint::LuauTypePathMaximumTraverseSteps.get()
  }
}

impl TraversalState {
  pub fn traversal_state_type_id_not_null_builtin_types_type_arena(
    root: TypeId,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Self {
    TraversalState {
      current: TypeOrPack::V0(follow_type::follow(root)),
      builtin_types,
      arena: Handle::from_ptr(arena),
      steps: 0,
      encountered_error_suppression: false,
    }
  }

  pub(crate) fn traversal_state_type_pack_id_not_null_builtin_types_type_arena(
    root: TypePackId,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Self {
    TraversalState {
      current: TypeOrPack::V1(follow_type_pack::follow(root)),
      builtin_types,
      arena: Handle::from_ptr(arena),
      steps: 0,
      encountered_error_suppression: false,
    }
  }
}

impl TraversalState {
  pub fn update_current_type_id(&mut self, ty: TypeId) {
    LUAU_ASSERT!(!ty.is_null());
    self.current = TypeOrPack::V0(follow_type::follow(ty));
  }

  pub(crate) fn update_current_type_pack_id(&mut self, tp: TypePackId) {
    LUAU_ASSERT!(!tp.is_null());
    self.current = TypeOrPack::V1(follow_type_pack::follow(tp));
  }
}
