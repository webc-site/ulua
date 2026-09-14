use alloc::string::String;

use ulua_analysis::{
  functions::{
    add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b,
    freeze::freeze, register_builtin_globals::register_builtin_globals, unfreeze::unfreeze,
  },
  records::{binding::Binding, frontend::Frontend},
};
use ulua_ast::records::location::Location;

use crate::records::ac_fixture_impl::AcFixtureImpl;

impl AcFixtureImpl {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let frontend_ptr = self.base.get_frontend() as *mut Frontend;

    if !self.autocomplete_globals_registered {
      if self.register_builtins {
        unsafe {
          unfreeze((*frontend_ptr).globals.global_types_mut());
          unfreeze((*frontend_ptr).globals_for_autocomplete.global_types_mut());

          register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
          register_builtin_globals(
            &mut *frontend_ptr,
            &mut (*frontend_ptr).globals_for_autocomplete,
            true,
          );
        }
        self.base.register_test_types();
        unsafe {
          freeze((*frontend_ptr).globals.global_types_mut());
          freeze((*frontend_ptr).globals_for_autocomplete.global_types_mut());
        }
      } else {
        let any_type = unsafe { (*(*frontend_ptr).builtin_types).any_type };
        let make_binding = || Binding {
          type_id: any_type,
          location: Location::default(),
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        };

        unsafe {
          add_global_binding_builtin_definitions_alt_b(
            &mut (*frontend_ptr).globals,
            "table",
            make_binding(),
          );
          add_global_binding_builtin_definitions_alt_b(
            &mut (*frontend_ptr).globals,
            "math",
            make_binding(),
          );
          add_global_binding_builtin_definitions_alt_b(
            &mut (*frontend_ptr).globals_for_autocomplete,
            "table",
            make_binding(),
          );
          add_global_binding_builtin_definitions_alt_b(
            &mut (*frontend_ptr).globals_for_autocomplete,
            "math",
            make_binding(),
          );
        }
      }

      self.autocomplete_globals_registered = true;
    }

    unsafe { &mut *frontend_ptr }
  }
}
