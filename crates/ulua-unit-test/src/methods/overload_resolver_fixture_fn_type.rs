use ulua_analysis::type_aliases::type_id::TypeId;

use crate::{
  methods::overload_resolver_fixture_new::add_function_type,
  records::overload_resolver_fixture::OverloadResolverFixture,
};

impl OverloadResolverFixture {
  /// 对应 C++ `OverloadResolverFixture::fn`（tests/OverloadResolver.test.cpp:63-66）：
  /// 在 arena 上造一个单态函数类型 `(args) -> rets`。
  pub fn fn_type(&self, args: &[TypeId], rets: &[TypeId]) -> TypeId {
    // Safety: `self.arena` 为 `arena_` Box 的稳定堆地址，独占借用仅此一处、
    // 顺序完成三笔追加（[`OverloadResolverFixture::arena_view`] 契约）。
    add_function_type(unsafe { self.arena_view() }, args, rets)
  }
}
