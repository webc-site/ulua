use ulua_common::FInt;

use crate::{
  enums::tarjan_result::TarjanResult,
  records::{tarjan::Tarjan, tarjan_worklist_vertex::TarjanWorklistVertex},
  type_aliases::type_id::TypeId,
};
impl Tarjan {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_root_type_id(&mut self, ty: TypeId) -> TarjanResult {
    self.child_count = 0;
    if self.child_limit == 0 {
      self.child_limit = FInt::LuauTarjanChildLimit.get();
    }

    let ty = unsafe { (*self.log).follow_type_id(ty) };

    let (index, _fresh) = self.indexify_type_id(ty);
    self.worklist.push(TarjanWorklistVertex {
      index,
      curr_edge: -1,
      last_edge: -1,
    });

    self.loop_item()
  }
}
