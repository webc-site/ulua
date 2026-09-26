use alloc::boxed::Box;

use ulua_analysis::{
  functions::autocomplete_autocomplete::autocomplete,
  records::{autocomplete_result::AutocompleteResult, frontend_options::FrontendOptions},
  type_aliases::{
    module_name_type::ModuleName, string_completion_callback::StringCompletionCallback,
  },
};
use ulua_ast::{enums::mode::Mode, records::position::Position};

use crate::{
  functions::null_callback_autocomplete_test::null_callback,
  records::ac_fixture_impl::{AcFixtureImpl, IntoMarker},
};

const MAIN_MODULE_NAME: &str = "MainModule";

impl AcFixtureImpl {
  /// 三条 autocomplete 入口的共用前段：NoCheck 模式 + autocomplete 选项
  /// （可选启用 require suggester）→ check → autocomplete。
  fn check_and_autocomplete(
    &mut self,
    module_name: &ModuleName,
    position: Position,
    callback: StringCompletionCallback,
    with_require_suggester: bool,
  ) -> AutocompleteResult {
    let opts = FrontendOptions {
      for_autocomplete: true,
      retain_full_type_graphs: true,
      ..Default::default()
    };
    self.base.config_resolver.default_config.mode = Mode::NoCheck;
    if with_require_suggester {
      self.base.file_resolver.enable_require_suggester();
    }
    let frontend = self.get_frontend();
    frontend.check_module_name_optional_frontend_options(module_name, Some(opts));
    autocomplete(self.get_frontend(), module_name, position, callback)
  }
}

impl AcFixtureImpl {
  pub fn autocomplete_position(&mut self, row: u32, column: u32) -> AutocompleteResult {
    let module_name = ModuleName::from(MAIN_MODULE_NAME);
    let position = Position { line: row, column };
    self.check_and_autocomplete(&module_name, position, Box::new(null_callback), false)
  }
}

impl AcFixtureImpl {
  pub fn autocomplete_marker(&mut self, marker: impl IntoMarker) -> AutocompleteResult {
    self.autocomplete_marker_callback(marker, Box::new(null_callback))
  }

  pub fn autocomplete_marker_callback(
    &mut self,
    marker: impl IntoMarker,
    callback: StringCompletionCallback,
  ) -> AutocompleteResult {
    let module_name = ModuleName::from(MAIN_MODULE_NAME);
    let position = *self.get_position(marker);
    self.check_and_autocomplete(&module_name, position, callback, true)
  }
}

impl AcFixtureImpl {
  pub fn autocomplete_module_name_position_string_completion_callback(
    &mut self,
    name: &ModuleName,
    pos: Position,
    callback: StringCompletionCallback,
  ) -> AutocompleteResult {
    self.check_and_autocomplete(name, pos, callback, true)
  }
}
