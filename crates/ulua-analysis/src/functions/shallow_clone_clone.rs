use alloc::vec::Vec;

use crate::{
  functions::clone_clone::{type_is_persistent, with_clone_maps},
  records::{
    arena_handle::Handle, clone_state::CloneState, type_arena::TypeArena, type_cloner::TypeCloner,
  },
  type_aliases::type_id::TypeId,
};
/// 形参全为受检类型/引用（`TypeId` 为 arena 句柄别名、`dest`/`clone_state` 为
/// 独占可变借用），clone 表经 `with_clone_maps` 以 `&mut` 接线给 `TypeCloner`，
/// 契约由类型承载。
pub fn shallow_clone(
  type_id: TypeId,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
  clone_persistent_types: bool,
) -> TypeId {
  // `type_is_persistent` 即 crate 对 `ty->persistent` 的只读收口探针。
  if type_is_persistent(type_id) && !clone_persistent_types {
    return type_id;
  }

  let builtin_types = clone_state.builtin_types;
  let force_ty = clone_persistent_types.then_some(type_id);
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = TypeCloner {
        arena: Handle::from_mut(dest),
        builtin_types,
        queue: Vec::new(),
        types: tys,
        packs: tps,
        force_ty,
        force_tp: None,
        steps: 0,
        replacement_for_null_scope: None,
        skip_lazy_type_clone: false,
      };
      cloner.shallow_clone_type_id(type_id)
    },
  )
}
