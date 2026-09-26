use ulua_analysis::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn pack_initializer_list_type_id(&self, tys: &[TypeId]) -> TypePackId {
    // Safety: `self.arena` 为 `arena_` Box 的稳定非空堆地址（随 fixture 存活），
    // add_type_pack 独占顺序追加（cpp 同形，见
    // [`OverloadResolverFixture::arena_view`] 契约）。
    unsafe { self.arena_view() }.add_type_pack_initializer_list_type_id(tys)
  }
}
