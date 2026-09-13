use core::ptr;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    clone_public_interface::ClonePublicInterface, function_type::FunctionType,
    table_type::TableType, type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

impl ClonePublicInterface {
  /// `bool ClonePublicInterface::isDirty(TypeId ty)`.
  /// Reference: `Module.cpp:134-144`.
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    // SAFETY: module 在 ClonePublicInterface 存活期内有效。
    let module = unsafe { &*self.module };

    if ptr::eq(type_owning_arena(ty), &module.internal_types) {
      return true;
    }

    if let Some(ftv) = get_type_id::<FunctionType>(ty) {
      return ftv.level.level != 0;
    }

    if let Some(ttv) = get_type_id::<TableType>(ty) {
      return ttv.level.level != 0;
    }

    false
  }
}

/// C++ `ty->owningArena`（Module.cpp:135）：TypeId 句柄解引用收口在私有
/// helper，公共方法保持 safe。
fn type_owning_arena(ty: TypeId) -> *mut TypeArena {
  // SAFETY: ty 有效性由调用方按 C++ 同契约保证（arena 存活分配），此处仅解引用一次。
  unsafe { (*ty).owning_arena }
}
