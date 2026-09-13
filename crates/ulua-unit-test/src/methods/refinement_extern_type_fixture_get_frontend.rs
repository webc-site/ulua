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
    let frontend_ptr = self.base.get_frontend() as *mut Frontend;

    if !already_initialized {
      register_refinement_extern_type_fixture_types(unsafe { &mut *frontend_ptr });
    }

    unsafe { &mut *frontend_ptr }
  }
}
