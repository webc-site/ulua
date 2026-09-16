use std::{ffi::CString, sync::Arc};

use ulua_ast::records::ast_name_table::AstNameTable;

use crate::records::{binding::Binding, global_types::GlobalTypes, symbol::Symbol};
pub fn try_get_global_binding(globals: &mut GlobalTypes, name: &str) -> Option<Binding> {
  let name = CString::new(name).ok()?;
  let ast_name = unsafe {
    (*(Arc::as_ptr(&globals.global_names.names) as *mut AstNameTable))
      .get_or_add(name.as_ptr(), name.as_bytes().len())
  };
  globals
    .global_scope
    .bindings
    .get(&Symbol::from_global(ast_name))
    .cloned()
}
