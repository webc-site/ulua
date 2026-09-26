use crate::{
  functions::get_type,
  records::{
    arena_id::ArenaId, clone_public_interface::ClonePublicInterface, function_type::FunctionType,
    table_type::TableType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ClonePublicInterface {
  /// `bool ClonePublicInterface::isDirty(TypeId ty)`.
  /// Reference: `Module.cpp:134-144`.
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    // SAFETY: module 在 ClonePublicInterface 存活期内有效。
    let module = unsafe { &*self.module };

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
  // SAFETY: ty 有效性由调用方按 C++ 同契约保证（arena 存活分配），此处仅解引用一次。
  unsafe { (*ty).owning_arena }
}

impl ClonePublicInterface {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  /// `bool ClonePublicInterface::isDirty(TypePackId tp)`.
  /// Reference: `Module.cpp:146-149`.
  pub unsafe fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    // Safety: `self.module` 为构造期接线的非空存活 `*mut Module`，取共享引用只读其
    // `internal_types` arena 字段，本对象存活期内 module 一直有效（同 `is_dirty_type_id`）。
    let module = unsafe { &*self.module };
    // Safety: `tp` 是传入的存活 `TypePackId`（借用自类型 arena 的身份句柄），此处只读
    // 其 `owning_arena` 身份值用于归属比较。
    let owning_arena = unsafe { (*tp).owning_arena };
    owning_arena == module.internal_types.arena_id
  }
}
