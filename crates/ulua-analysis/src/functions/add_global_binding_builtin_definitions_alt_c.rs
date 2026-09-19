use alloc::{format, string::String};

use ulua_ast::records::location::Location;

use crate::{
  functions::add_global_binding_builtin_definitions_alt_d::add_global_binding_builtin_definitions_alt_d,
  records::{binding::Binding, global_types::GlobalTypes},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

pub fn add_global_binding_builtin_definitions_alt_c(
  globals: &mut GlobalTypes,
  scope: &ScopePtr,
  name: &str,
  ty: TypeId,
  package_name: &str,
) {
  let documentation_symbol: String = format!("{}/global/{}", package_name, name);
  add_global_binding_builtin_definitions_alt_d(
    globals,
    scope,
    name,
    Binding {
      type_id: ty,
      location: Location::default(),
      deprecated: false,
      deprecated_suggestion: String::new(),
      documentation_symbol: Some(documentation_symbol),
    },
  );
}
