//! C++ `Frontend& FragmentAutocompleteBuiltinsFixture::getFrontend() override`
//! (tests/FragmentAutocomplete.test.cpp:324-353).
use alloc::string::String;

use ulua_analysis::{
  functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
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
    let frontend_ptr = self.base.base.get_frontend() as *mut Frontend;
    unsafe { unfreeze_globals_pair(&mut *frontend_ptr) };

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
    unsafe {
      unfreeze_globals_pair(&mut *frontend_ptr);

      let game_binding = Binding {
        type_id: any_type,
        location: Location::default(),
        deprecated: false,
        deprecated_suggestion: String::new(),
        documentation_symbol: None,
      };
      add_global_binding_builtin_definitions_alt_b(
        &mut (*frontend_ptr).globals,
        "game",
        game_binding.clone(),
      );
      add_global_binding_builtin_definitions_alt_b(
        &mut (*frontend_ptr).globals_for_autocomplete,
        "game",
        game_binding,
      );

      freeze_globals_pair(&mut *frontend_ptr);
    }

    self.base.base.get_frontend()
  }
}
