use ulua_analysis::records::frontend::Frontend;

use crate::{
  functions::register_hidden_types::register_hidden_types,
  records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture,
};

impl NonStrictTypeCheckerFixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let already_initialized = self.base.frontend.is_some();
    let frontend_ptr = self.base.get_frontend() as *mut Frontend;

    if !already_initialized {
      unsafe {
        register_hidden_types(&mut *frontend_ptr);
      }
      self.base.register_test_types();
    }

    unsafe { &mut *frontend_ptr }
  }
}
