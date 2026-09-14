use crate::{records::substitution::Substitution, type_aliases::type_pack_id::TypePackId};

impl Substitution {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn replace_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

    if let Some(prev_tp) = self.new_packs.find(&tp) {
      *prev_tp
    } else {
      tp
    }
  }
}
