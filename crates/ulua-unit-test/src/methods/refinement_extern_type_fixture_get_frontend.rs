use ulua_analysis::records::frontend::Frontend;

use crate::{
  functions::register_refinement_extern_type_fixture_types::register_refinement_extern_type_fixture_types,
  records::refinement_extern_type_fixture::RefinementExternTypeFixture,
};

impl RefinementExternTypeFixture {
  /// C++ `RefinementExternTypeFixture::getFrontend`
  /// (`tests/TypeInfer.refinements.test.cpp:82`). Lazily builds the base
  /// builtins frontend, then registers the extern types the refinement tests
  /// rely on exactly once.
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let already_initialized = self.base.base.frontend.is_some();

    if !already_initialized {
      // (a) 类 `as *mut Frontend` + `unsafe { &mut *ptr }` 绕道已消除：
      // base.get_frontend() 即 `&mut Frontend`，直传 register 独占登记
      // （cpp Frontend& 同形），借用先于返回值的现取借用结束。
      register_refinement_extern_type_fixture_types(self.base.get_frontend());
    }

    self.base.get_frontend()
  }
}
