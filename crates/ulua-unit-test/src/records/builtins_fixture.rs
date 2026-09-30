use ulua_analysis::records::frontend::Frontend;

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

    if !already_initialized {
      // cpp `BuiltinsFixture::getFrontend`：unfreeze → registerBuiltinGlobals
      // （`for_autocomplete` 时补注册一次到 `globalsForAutocomplete`）→
      // registerTestTypes → freeze。
      //
      // 步④：`(&mut Frontend, &mut Frontend.globals)` 的重叠借用已收进
      // ulua-analysis 的单 `&mut Frontend` 门面 [`Frontend::register_builtin_globals`]，
      // 本函数全程安全借用：每条语句经 `base.get_frontend()` 现取借用、止于句尾
      // （它只刷新 resolver 裸句柄，与 cpp `getFrontend()` 同义），故
      // `register_test_types()` 这类 `&mut self.base` 调用可以穿插其间。
      unfreeze_globals_pair(self.base.get_frontend());
      self.base.get_frontend().register_builtin_globals(false);
      if self.base.for_autocomplete {
        self
          .base
          .get_frontend()
          .register_builtin_globals_for_autocomplete();
      }

      self.base.register_test_types();

      freeze_globals_pair(self.base.get_frontend());
    }

    self.base.get_frontend()
  }
}
