use crate::{records::anyification::Anyification, type_aliases::type_pack_id::TypePackId};

impl Anyification {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn ignore_children_type_pack_id(&mut self, ty: TypePackId) -> bool {
    unsafe { (*ty).persistent }
  }
}
