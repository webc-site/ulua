use alloc::string::String;

use ulua_analysis::{
  functions::add_global_binding_builtin_definitions::add_global_binding_value,
  records::{binding::Binding, frontend::Frontend},
};
use ulua_ast::records::location::Location;

use crate::{
  functions::{
    freeze_globals_pair::freeze_globals_pair, unfreeze_globals_pair::unfreeze_globals_pair,
  },
  records::ac_fixture_impl::AcFixtureImpl,
};

impl AcFixtureImpl {
  /// C++ 补全 fixture 的 `getFrontend`：首轮把 builtin globals 注册进 `frontend`
  /// （或退化为只挂 `table`/`math` 两个 binding），注册完 `registerTestTypes()` 后重冻。
  ///
  /// 步④：cpp `registerBuiltinGlobals(frontend, frontend.globals)` 与
  /// `registerBuiltinGlobals(frontend, frontend.globalsForAutocomplete, true)`
  /// 的「整体 + 字段」重叠借用已收进 ulua-analysis 的单 `&mut Frontend` 门面
  /// [`Frontend::register_builtin_globals`] /
  /// [`Frontend::register_builtin_globals_for_autocomplete`]，本函数全程安全借用：
  /// 每条语句经 `base.get_frontend()` 现取借用、止于句尾（它只刷新 resolver 裸
  /// 句柄，语义同 cpp `getFrontend()`），故 `register_test_types()` 可穿插其间。
  pub fn get_frontend(&mut self) -> &mut Frontend {
    if !self.autocomplete_globals_registered {
      if self.register_builtins {
        unfreeze_globals_pair(self.base.get_frontend());
        self.base.get_frontend().register_builtin_globals(false);
        self
          .base
          .get_frontend()
          .register_builtin_globals_for_autocomplete();

        self.base.register_test_types();
        freeze_globals_pair(self.base.get_frontend());
      } else {
        let frontend = self.base.get_frontend();
        let any_type = frontend.builtin_types_ref().any_type;
        let make_binding = || Binding {
          type_id: any_type,
          location: Location::default(),
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        };

        add_global_binding_value(&mut frontend.globals, "table", make_binding());
        add_global_binding_value(&mut frontend.globals, "math", make_binding());
        add_global_binding_value(
          &mut frontend.globals_for_autocomplete,
          "table",
          make_binding(),
        );
        add_global_binding_value(
          &mut frontend.globals_for_autocomplete,
          "math",
          make_binding(),
        );
      }

      self.autocomplete_globals_registered = true;
    }

    self.base.get_frontend()
  }
}
