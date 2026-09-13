use core::ptr::null_mut;

use crate::{records::tarjan::Tarjan, type_aliases::type_id::TypeId};
impl Tarjan {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_child_type_id(&mut self, ty: TypeId) {
    let ty = unsafe { (*self.log).follow_type_id(ty) };

    self.edges_ty.push(ty);
    self.edges_tp.push(null_mut());
  }
}
