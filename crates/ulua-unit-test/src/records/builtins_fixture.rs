use ulua_analysis::{
  functions::register_builtin_globals::register_builtin_globals, records::frontend::Frontend,
};

use crate::{
  functions::{
    freeze_globals_pair::freeze_globals_pair, unfreeze_globals_pair::unfreeze_globals_pair,
  },
  records::fixture::Fixture,
};

#[derive(Debug)]
pub struct BuiltinsFixture {
  pub base: Fixture,
}

impl Default for BuiltinsFixture {
  fn default() -> Self {
    Self {
      base: Fixture::fixture_bool(false),
    }
  }
}

impl BuiltinsFixture {
  pub fn builtins_fixture_builtins_fixture(&mut self, prepare_autocomplete: bool) {
    self.base = Fixture::fixture_bool(prepare_autocomplete);
  }

  pub fn get_frontend(&mut self) -> &mut Frontend {
    let already_initialized = self.base.frontend.is_some();
    let frontend_ptr = self.base.get_frontend() as *mut Frontend;

    if !already_initialized {
      unsafe {
        unfreeze_globals_pair(&mut *frontend_ptr);

        register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);

        if self.base.for_autocomplete {
          register_builtin_globals(
            &mut *frontend_ptr,
            &mut (*frontend_ptr).globals_for_autocomplete,
            true,
          );
        }
      }

      self.base.register_test_types();

      unsafe { freeze_globals_pair(&mut *frontend_ptr) };
    }

    unsafe { &mut *frontend_ptr }
  }
}
