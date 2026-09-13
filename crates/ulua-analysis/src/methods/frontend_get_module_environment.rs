use alloc::{string::String, sync::Arc};
use core::ptr::null;
use std::ffi::CString;

use ulua_ast::records::location::Location;
use ulua_config::records::config::Config;

use crate::{
  records::{
    binding::Binding, frontend::Frontend, scope::Scope, source_module::SourceModule, symbol::Symbol,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl Frontend {
  pub fn get_module_environment(
    &self,
    module: &SourceModule,
    config: &Config,
    for_autocomplete: bool,
  ) -> ScopePtr {
    let mut result = if for_autocomplete {
      self.globals_for_autocomplete.global_scope.clone()
    } else {
      self.globals.global_scope.clone()
    };

    if let Some(environment_name) = &module.environment_name {
      result = self.get_environment_scope(environment_name.clone());
    }

    if !config.globals.is_empty() {
      result = Arc::new(Scope::new(&result, 0));

      for global in &config.globals {
        let global_cstr = CString::new(global.as_str()).expect("global names cannot contain NUL");
        let name = unsafe { module.names.get(global_cstr.as_ptr()) };

        if !name.value.is_null() {
          let scope =
            Arc::get_mut(&mut result).expect("new module environment scope is uniquely owned");
          let binding = scope
            .bindings
            .entry(Symbol::from_global(name))
            .or_insert(Binding {
              type_id: null(),
              location: Location::default(),
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            });
          binding.type_id = unsafe { (*self.builtin_types).any_type };
        }
      }
    }

    result
  }
}
