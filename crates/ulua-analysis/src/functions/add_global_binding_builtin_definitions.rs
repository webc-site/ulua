use crate::{
  functions::add_global_binding_builtin_definitions_alt_c::add_global_binding_builtin_definitions_alt_c,
  records::global_types::GlobalTypes, type_aliases::type_id::TypeId,
};

pub fn add_global_binding_builtin_definitions(
  globals: &mut GlobalTypes,
  name: &str,
  ty: TypeId,
  package_name: &str,
) {
  let scope = globals.global_scope.clone();
  add_global_binding_builtin_definitions_alt_c(globals, &scope, name, ty, package_name);
}
