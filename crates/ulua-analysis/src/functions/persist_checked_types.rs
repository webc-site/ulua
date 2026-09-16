use alloc::{ffi::CString, string::String, sync::Arc, vec::Vec};
use core::ptr::null;

use ulua_ast::records::{ast_name_table::AstNameTable, location::Location};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    clone_clone_alt_b::clone as clone_type, clone_clone_alt_c::clone as clone_type_fun,
    generate_documentation_symbols::generate_documentation_symbols, persist_type::persist,
  },
  records::{
    binding::Binding, clone_state::CloneState, global_types::GlobalTypes, module::Module,
    scope::Scope, symbol::Symbol,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
pub fn persist_checked_types(
  checked_module: Arc<Module>,
  globals: &mut GlobalTypes,
  target_scope: ScopePtr,
  package_name: String,
) {
  let mut clone_state = CloneState {
    builtin_types: globals.builtin_types.as_ptr(),
    seen_types: DenseHashMap::new(null()),
    seen_type_packs: DenseHashMap::new(null()),
  };

  let mut types_to_persist: Vec<TypeId> = Vec::with_capacity(
    checked_module.declared_globals.len() + checked_module.exported_type_bindings.len(),
  );

  let target_scope_ptr = Arc::as_ptr(&target_scope) as *mut Scope;
  let names_ptr = Arc::as_ptr(&globals.global_names.names) as *mut AstNameTable;

  for (name, ty) in &checked_module.declared_globals {
    let global_ty = unsafe { clone_type(*ty, &mut globals.global_types, &mut clone_state) };

    const INFIX: &str = "/global/";
    let mut documentation_symbol =
      String::with_capacity(package_name.len() + INFIX.len() + name.len());
    documentation_symbol.push_str(&package_name);
    documentation_symbol.push_str(INFIX);
    documentation_symbol.push_str(name);

    unsafe { generate_documentation_symbols(global_ty, documentation_symbol.clone()) };

    let name_cstr = CString::new(name.as_str()).unwrap();
    let ast_name =
      unsafe { (*names_ptr).get_or_add(name_cstr.as_ptr(), name_cstr.as_bytes().len()) };
    let binding = Binding {
      type_id: global_ty,
      location: Location::default(),
      deprecated: false,
      deprecated_suggestion: String::new(),
      documentation_symbol: Some(documentation_symbol),
    };
    unsafe {
      (*target_scope_ptr)
        .bindings
        .insert(Symbol::from_global(ast_name), binding);
    }

    types_to_persist.push(global_ty);
  }

  for (name, ty) in &checked_module.exported_type_bindings {
    let global_ty = clone_type_fun(ty, &mut globals.global_types, &mut clone_state);

    const INFIX: &str = "/globaltype/";
    let mut documentation_symbol =
      String::with_capacity(package_name.len() + INFIX.len() + name.len());
    documentation_symbol.push_str(&package_name);
    documentation_symbol.push_str(INFIX);
    documentation_symbol.push_str(name);

    unsafe { generate_documentation_symbols(global_ty.r#type, documentation_symbol) };

    let global_ty_type = global_ty.r#type;
    unsafe {
      (*target_scope_ptr)
        .exported_type_bindings
        .insert(name.clone(), global_ty);
    }

    types_to_persist.push(global_ty_type);
  }

  for ty in types_to_persist {
    persist(ty);
  }
}
