use crate::{
  functions::add_global_binding_builtin_definitions_alt_d::add_global_binding_builtin_definitions_alt_d,
  records::{binding::Binding, global_types::GlobalTypes},
};

pub fn add_global_binding_builtin_definitions_alt_b(
  globals: &mut GlobalTypes,
  name: &str,
  binding: Binding,
) {
  let scope = globals.global_scope.clone();
  add_global_binding_builtin_definitions_alt_d(globals, &scope, name, binding);
}
