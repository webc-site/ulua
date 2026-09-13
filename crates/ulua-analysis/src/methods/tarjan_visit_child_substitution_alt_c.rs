use core::ptr::null_mut;

use crate::{records::tarjan::Tarjan, type_aliases::type_pack_id::TypePackId};
impl Tarjan {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_child_type_pack_id(&mut self, tp: TypePackId) {
    let tp = unsafe { (*self.log).follow_type_pack_id(tp) };

    self.edges_ty.push(null_mut());
    self.edges_tp.push(tp);
  }
}
