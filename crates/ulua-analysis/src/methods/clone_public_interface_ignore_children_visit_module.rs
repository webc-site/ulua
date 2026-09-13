use std::ptr::eq;

use crate::{records::clone_public_interface::ClonePublicInterface, type_aliases::type_id::TypeId};
impl ClonePublicInterface {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  /// `bool ClonePublicInterface::ignoreChildrenVisit(TypeId ty)`.
  /// Reference: `Module.cpp:151-157`.
  pub unsafe fn ignore_children_visit_type_id(&mut self, ty: TypeId) -> bool {
    let module = unsafe { &*self.module };

    let owning_arena = unsafe { (*ty).owning_arena };
    !eq(owning_arena, &module.internal_types)
  }
}
