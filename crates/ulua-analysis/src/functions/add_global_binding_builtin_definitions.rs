use alloc::{format, string::String};

use ulua_ast::records::location::Location;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{binding::Binding, global_types::GlobalTypes, symbol::Symbol},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

pub fn add_global_binding_builtin_definitions(
  globals: &mut GlobalTypes,
  name: &str,
  ty: TypeId,
  package_name: &str,
) {
  let scope = globals.global_scope.clone();
  add_global_binding_in_scope(globals, &scope, name, ty, package_name);
}

pub fn add_global_binding_value(globals: &mut GlobalTypes, name: &str, binding: Binding) {
  let scope = globals.global_scope.clone();
  add_global_binding_in_scope_value(globals, &scope, name, binding);
}

pub fn add_global_binding_in_scope(
  globals: &mut GlobalTypes,
  scope: &ScopePtr,
  name: &str,
  ty: TypeId,
  package_name: &str,
) {
  let documentation_symbol: String = format!("{}/global/{}", package_name, name);
  add_global_binding_in_scope_value(
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

pub fn add_global_binding_in_scope_value(
  globals: &mut GlobalTypes,
  scope: &ScopePtr,
  name: &str,
  binding: Binding,
) {
  // cpp: `getOrAdd(name.c_str(), name.length())`；长度已给出，内部 `&str` 入口
  // 免去 CString 分配与指针往返。
  let ast_name = unsafe { (*(arc_as_mut(&globals.global_names.names))).get_or_add_str(name) };

  let scope_ptr = arc_as_mut(scope);
  unsafe {
    (*scope_ptr)
      .bindings
      .insert(Symbol::from_global(ast_name), binding);
  }
}
