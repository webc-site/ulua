use crate::{
  functions::follow_type_pack::follow_type_pack_id, records::type_cacher::TypeCacher,
  type_aliases::type_pack_id::TypePackId,
};

impl TypeCacher {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn is_uncacheable_type_pack_id(&self, tp: TypePackId) -> bool {
    unsafe { self.uncacheable_packs.contains(&follow_type_pack_id(tp)) }
  }
}
