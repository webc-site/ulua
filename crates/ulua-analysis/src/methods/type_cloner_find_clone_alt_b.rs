use crate::{
  functions::follow_type_pack::follow_type_pack_id, records::type_cloner::TypeCloner,
  type_aliases::type_pack_id::TypePackId,
};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn find_type_pack_id(&self, tp: TypePackId) -> Option<TypePackId> {
    let tp = unsafe { follow_type_pack_id(tp) };

    if let Some(it) = unsafe { (*self.packs).get(&tp) } {
      return Some(*it);
    } else if unsafe { (*tp).persistent } && tp != self.force_tp {
      return Some(tp);
    }

    None
  }
}
