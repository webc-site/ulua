use ulua_analysis::type_aliases::type_id::TypeId;

use crate::{
  methods::overload_resolver_fixture_new::add_function_type,
  records::overload_resolver_fixture::OverloadResolverFixture,
};

impl OverloadResolverFixture {
  /// 对应 C++ `OverloadResolverFixture::fn`（tests/OverloadResolver.test.cpp:63-66）：
  /// 在 arena 上造一个单态函数类型 `(args) -> rets`。
  pub fn fn_type(&self, args: &[TypeId], rets: &[TypeId]) -> TypeId {
    add_function_type(self.arena, args, rets)
  }
}
