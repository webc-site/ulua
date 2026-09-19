//! Source: `tests/Autocomplete.test.cpp`

use alloc::{boxed::Box, string::String};

use ulua_analysis::{
  functions::autocomplete_autocomplete::autocomplete,
  records::{autocomplete_result::AutocompleteResult, frontend_options::FrontendOptions},
  type_aliases::string_completion_callback::StringCompletionCallback,
};
use ulua_ast::enums::mode::Mode;

use crate::{
  functions::null_callback_autocomplete_test::null_callback,
  records::ac_fixture_impl::{AcFixtureImpl, IntoMarker},
};

const MAIN_MODULE_NAME: &str = "MainModule";

impl AcFixtureImpl {
  pub fn autocomplete_marker(&mut self, marker: impl IntoMarker) -> AutocompleteResult {
    self.autocomplete_marker_callback(marker, Box::new(null_callback))
  }

  pub fn autocomplete_marker_callback(
    &mut self,
    marker: impl IntoMarker,
    callback: StringCompletionCallback,
  ) -> AutocompleteResult {
    let marker = marker.into_marker();
    let opts = FrontendOptions {
      for_autocomplete: true,
      retain_full_type_graphs: true,
      ..Default::default()
    };
    self.base.config_resolver.default_config.mode = Mode::NoCheck;
    self.base.file_resolver.enable_require_suggester();
    let module_name = String::from(MAIN_MODULE_NAME);
    let position = *self.get_position(marker);

    {
      let frontend = self.get_frontend();
      frontend.check_module_name_optional_frontend_options(&module_name, Some(opts));
    }

    autocomplete(self.get_frontend(), &module_name, position, callback)
  }
}
