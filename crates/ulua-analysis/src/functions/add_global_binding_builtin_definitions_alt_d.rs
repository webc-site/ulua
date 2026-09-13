use alloc::{ffi::CString, sync::Arc};

use ulua_ast::records::ast_name_table::AstNameTable;

use crate::{
  records::{binding::Binding, global_types::GlobalTypes, scope::Scope, symbol::Symbol},
  type_aliases::scope_ptr_type::ScopePtr,
};

pub fn add_global_binding_builtin_definitions_alt_d(
  globals: &mut GlobalTypes,
  scope: &ScopePtr,
  name: &str,
  binding: Binding,
) {
  let name_cstr = CString::new(name).unwrap();
  let ast_name = unsafe {
    (*(Arc::as_ptr(&globals.global_names.names) as *mut AstNameTable))
      .get_or_add(name_cstr.as_ptr(), name_cstr.as_bytes().len())
  };

  let scope_ptr = Arc::as_ptr(scope) as *mut Scope;
  unsafe {
    (*scope_ptr)
      .bindings
      .insert(Symbol::from_global(ast_name), binding);
  }
}
