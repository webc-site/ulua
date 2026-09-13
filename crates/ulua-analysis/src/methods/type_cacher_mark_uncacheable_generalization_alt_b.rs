use crate::{
  functions::follow_type_pack::follow_type_pack_id, records::type_cacher::TypeCacher,
  type_aliases::type_pack_id::TypePackId,
};

impl TypeCacher {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn mark_uncacheable_type_pack_id(&mut self, tp: TypePackId) {
    let followed = unsafe { follow_type_pack_id(tp) };
    self.uncacheable_packs.insert(followed);
  }
}
