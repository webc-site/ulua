use ulua_analysis::{
  functions::autocomplete_autocomplete::autocomplete,
  records::{autocomplete_result::AutocompleteResult, frontend_options::FrontendOptions},
  type_aliases::{
    module_name_type::ModuleName, string_completion_callback::StringCompletionCallback,
  },
};
use ulua_ast::{enums::mode::Mode, records::position::Position};

use crate::records::ac_fixture_impl::AcFixtureImpl;

impl AcFixtureImpl {
  pub fn autocomplete_module_name_position_string_completion_callback(
    &mut self,
    name: &ModuleName,
    pos: Position,
    callback: StringCompletionCallback,
  ) -> AutocompleteResult {
    let opts = FrontendOptions {
      for_autocomplete: true,
      retain_full_type_graphs: true,
      ..Default::default()
    };
    self.base.config_resolver.default_config.mode = Mode::NoCheck;
    self.base.file_resolver.enable_require_suggester();

    {
      let frontend = self.get_frontend();
      frontend.check_module_name_optional_frontend_options(name, Some(opts));
    }

    autocomplete(self.get_frontend(), name, pos, callback)
  }
}
