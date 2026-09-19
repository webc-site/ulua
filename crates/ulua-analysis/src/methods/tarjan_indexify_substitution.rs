use std::ptr::null_mut;

use crate::{
  records::{tarjan::Tarjan, tarjan_node::TarjanNode},
  type_aliases::type_id::TypeId,
};
impl Tarjan {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn indexify_type_id(&mut self, ty: TypeId) -> (i32, bool) {
    let ty = unsafe { (*self.log).follow_type_id(ty) };

    if let Some(&index) = self.type_to_index.find(&ty) {
      (index, false)
    } else {
      let index = self.nodes.len() as i32;
      self.type_to_index.try_insert(ty, index);
      self.nodes.push(TarjanNode {
        ty,
        tp: null_mut(),
        on_stack: false,
        dirty: false,
        lowlink: index,
      });
      (index, true)
    }
  }
}
