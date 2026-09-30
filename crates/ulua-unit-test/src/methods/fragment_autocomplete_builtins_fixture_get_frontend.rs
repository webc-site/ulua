//! C++ `Frontend& FragmentAutocompleteBuiltinsFixture::getFrontend() override`
//! (tests/FragmentAutocomplete.test.cpp:324-353).
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
  records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
};

impl FragmentAutocompleteBuiltinsFixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    // C++: if (frontend) return *frontend;
    if self.base.base.base.frontend.is_some() {
      return self.base.base.get_frontend();
    }

    // C++: Frontend& f = BuiltinsFixture::getFrontend(); then unfreeze globals.
    // (a) 类 `as *mut Frontend` + `unsafe { &mut *ptr }` 绕道已消除：每步经
    // base.get_frontend() 现取 `&mut Frontend` 直传（它只刷新 resolver 裸句柄，
    // 语义同 cpp getFrontend()），借用句尾归还，unfreeze/add_binding/freeze
    // 全程安全顺序变更。
    unfreeze_globals_pair(self.base.base.get_frontend());

    let fake_vec_decl = String::from(
      r#"
declare extern type FakeVec with
    function dot(self, x: FakeVec) : FakeVec
    zero : FakeVec
end
"#,
    );
    // Load the definition into both the 'globals'/'resolver' and the 'for autocomplete' equivalent.
    self.base.base.base.load_definition(&fake_vec_decl, false);
    self.base.base.base.load_definition(&fake_vec_decl, true);

    let any_type = self.base.base.base.get_builtins().any_type;
    let frontend = self.base.base.get_frontend();
    unfreeze_globals_pair(frontend);

    let game_binding = Binding {
      type_id: any_type,
      location: Location::default(),
      deprecated: false,
      deprecated_suggestion: String::new(),
      documentation_symbol: None,
    };
    add_global_binding_value(&mut frontend.globals, "game", game_binding.clone());
    add_global_binding_value(&mut frontend.globals_for_autocomplete, "game", game_binding);

    freeze_globals_pair(frontend);

    self.base.base.get_frontend()
  }
}
