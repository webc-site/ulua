use ulua_analysis::records::overload_resolver::OverloadResolver;

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn mk_resolver(&self) -> OverloadResolver {
    unsafe {
      OverloadResolver::new(
        self.builtin_types.as_ref() as *const _ as *mut _,
        self.arena,
        self.normalizer.as_ref() as *const _ as *mut _,
        self.type_function_runtime.as_ref() as *const _ as *mut _,
        self.root_scope.as_ref() as *const _ as *mut _,
        self.ice_reporter.as_ref() as *const _ as *mut _,
        self.limits.as_ref() as *const _ as *mut _,
        self.call_location,
      )
    }
  }
}
