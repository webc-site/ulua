use std::ptr::eq;

use crate::{
  records::clone_public_interface::ClonePublicInterface, type_aliases::type_pack_id::TypePackId,
};
impl ClonePublicInterface {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  /// `bool ClonePublicInterface::isDirty(TypePackId tp)`.
  /// Reference: `Module.cpp:146-149`.
  pub unsafe fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let module = unsafe { &*self.module };
    let owning_arena = unsafe { (*tp).owning_arena };
    eq(owning_arena, &module.internal_types)
  }
}
