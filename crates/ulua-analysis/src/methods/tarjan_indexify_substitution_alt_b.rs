use core::ptr::null_mut;

use crate::{
  records::{tarjan::Tarjan, tarjan_node::TarjanNode},
  type_aliases::type_pack_id::TypePackId,
};
impl Tarjan {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn indexify_type_pack_id(&mut self, mut tp: TypePackId) -> (i32, bool) {
    tp = unsafe { (*self.log).follow_type_pack_id(tp) };

    if let Some(&index) = self.pack_to_index.find(&tp) {
      (index, false)
    } else {
      let index = self.nodes.len() as i32;

      self.pack_to_index.try_insert(tp, index);

      self.nodes.push(TarjanNode {
        ty: null_mut(),
        tp,
        on_stack: false,
        dirty: false,
        lowlink: index,
      });

      (index, true)
    }
  }
}
