use crate::{
  functions::get_type,
  records::{
    arena_handle::alias_ref, arena_id::ArenaId, clone_public_interface::ClonePublicInterface,
    function_type::FunctionType, table_type::TableType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ClonePublicInterface {
  /// `bool ClonePublicInterface::isDirty(TypeId ty)`.
  /// Reference: `Module.cpp:134-144`.
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    let module = alias_ref(self.module);

    if type_owning_arena(ty) == module.internal_types.arena_id {
      return true;
    }

    if let Some(ftv) = get_type::get::<FunctionType>(ty) {
      return ftv.level.level != 0;
    }

    if let Some(ttv) = get_type::get::<TableType>(ty) {
      return ttv.level.level != 0;
    }

    false
  }
}

/// C++ `ty->owningArena`（Module.cpp:135）：TypeId 句柄解引用收口在私有
/// helper，公共方法保持 safe。读出的是 [`ArenaId`] 值，比较不含指针解引用。
fn type_owning_arena(ty: TypeId) -> ArenaId {
  alias_ref(ty).owning_arena
}

impl ClonePublicInterface {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub(crate) fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let module = alias_ref(self.module);
    let owning_arena = alias_ref(tp).owning_arena;
    owning_arena == module.internal_types.arena_id
  }
}
