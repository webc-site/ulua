use ulua_analysis::{records::union_type::UnionType, type_aliases::type_id::TypeId};

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn join(&self, a: TypeId, b: TypeId) -> TypeId {
    // Safety: `self.arena` 为 `arena_` Box 的稳定非空堆地址（随 fixture 存活），
    // add_type 独占顺序追加（cpp `fixture.arena->addType` 同形，见
    // [`OverloadResolverFixture::arena_view`] 契约）。
    unsafe { self.arena_view() }.add_type(UnionType {
      options: vec![a, b],
    })
  }
}
