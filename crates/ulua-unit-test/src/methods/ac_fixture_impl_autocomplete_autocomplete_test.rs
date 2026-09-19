//! Source: `tests/Autocomplete.test.cpp`

use alloc::{boxed::Box, string::String};

use ulua_analysis::{
  functions::autocomplete_autocomplete::autocomplete,
  records::{autocomplete_result::AutocompleteResult, frontend_options::FrontendOptions},
};
use ulua_ast::{enums::mode::Mode, records::position::Position};

use crate::{
  functions::null_callback_autocomplete_test::null_callback,
  records::ac_fixture_impl::AcFixtureImpl,
};

const MAIN_MODULE_NAME: &str = "MainModule";

impl AcFixtureImpl {
  pub fn autocomplete_position(&mut self, row: u32, column: u32) -> AutocompleteResult {
    let opts = FrontendOptions {
      for_autocomplete: true,
      retain_full_type_graphs: true,
      ..Default::default()
    };
    self.base.config_resolver.default_config.mode = Mode::NoCheck;
    let module_name = String::from(MAIN_MODULE_NAME);

    {
      let frontend = self.get_frontend();
      frontend.check_module_name_optional_frontend_options(&module_name, Some(opts));
    }

    autocomplete(
      self.get_frontend(),
      &module_name,
      Position { line: row, column },
      Box::new(null_callback),
    )
  }
}
