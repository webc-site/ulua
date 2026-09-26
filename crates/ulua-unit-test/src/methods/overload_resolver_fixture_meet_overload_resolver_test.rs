use ulua_analysis::{records::intersection_type::IntersectionType, type_aliases::type_id::TypeId};

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn meet_initializer_list_type_id(&self, parts: &[TypeId]) -> TypeId {
    // Safety: `self.arena` 为 `arena_` Box 的稳定非空堆地址（随 fixture 存活），
    // add_type 独占顺序追加（cpp 同形，见 [`OverloadResolverFixture::arena_view`]
    // 契约）。
    unsafe { self.arena_view() }.add_type(IntersectionType {
      parts: parts.to_vec(),
    })
  }
}
