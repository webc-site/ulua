use alloc::sync::Arc;

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
  // cpp: `getOrAdd(name.c_str(), name.length())`；长度已给出，内部 `&str` 入口
  // 免去 CString 分配与指针往返。
  let ast_name = unsafe {
    (*(Arc::as_ptr(&globals.global_names.names) as *mut AstNameTable)).get_or_add_str(name)
  };

  let scope_ptr = Arc::as_ptr(scope) as *mut Scope;
  unsafe {
    (*scope_ptr)
      .bindings
      .insert(Symbol::from_global(ast_name), binding);
  }
}
