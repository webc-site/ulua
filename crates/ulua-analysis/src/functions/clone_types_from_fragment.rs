//! C++ `void cloneTypesFromFragment(...)` (FragmentAutocomplete.cpp:671-795).
//!
//! Runs the `UsageFinder` traversal on the fragment and grabs all of the types
//! that are referenced in the fragment. We clone these and place them in the
//! appropriate spots in the scope so that they are available during
//! typechecking.
// C++ `Binding{type}` aggregate initialization with the remaining fields defaulted.
use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::{
  records::{ast_stat_block::AstStatBlock, location::Location},
  visit::ast_stat_block_visit,
};
use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  functions::{
    clone_incremental_clone::clone_incremental as clone_incremental_type_pack,
    clone_incremental_clone_alt_b::clone_incremental as clone_incremental_type,
    clone_incremental_clone_alt_c::clone_incremental as clone_incremental_type_fun,
    clone_incremental_clone_alt_d::clone_incremental as clone_incremental_binding,
  },
  records::{
    binding::Binding, blocked_type::BlockedType, builtin_types::BuiltinTypes,
    clone_state::CloneState, data_flow_graph::DataFlowGraph, def::Def, scope::Scope,
    symbol::Symbol, type_arena::TypeArena, usage_finder::UsageFinder,
  },
  type_aliases::{l_value::LValue, module_ptr_module::ModulePtr, type_id::TypeId},
};
fn binding_from_type(type_id: TypeId) -> Binding {
  Binding {
    type_id,
    location: Location::default(),
    deprecated: false,
    deprecated_suggestion: String::new(),
    documentation_symbol: None,
  }
}

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn clone_types_from_fragment(
  clone_state: &mut CloneState,
  stale_scope: *const Scope,
  stale_module: &ModulePtr,
  dest_arena: *mut TypeArena,
  dfg: *mut DataFlowGraph,
  _builtins: *mut BuiltinTypes,
  program: *mut AstStatBlock,
  dest_scope: *mut Scope,
) {
  LUAU_TIMETRACE_SCOPE!("Luau::cloneTypesFromFragment", "FragmentAutocomplete");

  let mut f = UsageFinder::new(dfg);
  ast_stat_block_visit(unsafe { &*program }, &mut f);

  let dest = unsafe { &mut *dest_arena };
  let stale = unsafe { &*stale_scope };

  // These are defs that have been mentioned. find the appropriate lvalue type and rvalue types and place them in the scope
  // First - any locals that have been mentioned in the fragment need to be placed in the bindings and lvalueTypes sections.
  for d in f.mentioned_defs.iter() {
    let d: *const Def = *d;
    if let Some(r_value_refinement) = stale.lookup_r_value_refinement_type(d) {
      let cloned =
        unsafe { clone_incremental_type(r_value_refinement, dest, clone_state, dest_scope) };
      unsafe {
        *(*dest_scope).rvalue_refinements.get_or_insert(d) = cloned;
      }
    }

    if let Some(l_value) = stale.lookup_unrefined_type(d) {
      let cloned = unsafe { clone_incremental_type(l_value, dest, clone_state, dest_scope) };
      unsafe {
        *(*dest_scope).lvalue_types.get_or_insert(d) = cloned;
      }
    }
  }

  for (d, loc) in f.local_bindings_referenced.iter() {
    let d: *const Def = *d;
    let loc = *loc;
    let name = unsafe {
      CStr::from_ptr((*loc).name.value)
        .to_string_lossy()
        .into_owned()
    };
    if let Some((sym, binding)) = stale.linear_search_for_binding_pair(&name, true) {
      let cloned_ty =
        unsafe { clone_incremental_type(binding.type_id, dest, clone_state, dest_scope) };
      let cloned_binding = clone_incremental_binding(&binding, dest, clone_state, dest_scope);
      unsafe {
        *(*dest_scope).lvalue_types.get_or_insert(d) = cloned_ty;
        (*dest_scope).bindings.insert(sym, cloned_binding);
      }
    }
  }

  for (d, syms) in f.symbols_to_refine.iter() {
    let d: *const Def = *d;
    let syms = LValue::Symbol(syms.clone());
    let mut current: Option<&Scope> = Some(stale);
    while let Some(scope) = current {
      if let Some(res) = scope.refinements.get(&syms) {
        let cloned = unsafe { clone_incremental_type(*res, dest, clone_state, dest_scope) };
        unsafe {
          *(*dest_scope).rvalue_refinements.get_or_insert(d) = cloned;
        }
        // If we've found a refinement, just break, otherwise we might end up doing the wrong thing.
        // We want the most "narrow" refinement here.
        break;
      }
      current = scope.parent.as_ref().map(|p| p.as_ref());
    }
  }

  // Second - any referenced type alias bindings need to be placed in scope so type annotation can be resolved.
  // If the actual type alias appears in the fragment on the lhs as a definition (in declaredAliases), it will be processed during typechecking anyway
  for x in f.referenced_bindings.iter() {
    if f.declared_aliases.contains(x) {
      continue;
    }
    if let Some(tf) = stale.lookup_type(x) {
      let cloned = clone_incremental_type_fun(&tf, dest, clone_state, dest_scope);
      unsafe {
        (*dest_scope)
          .private_type_bindings
          .insert(x.clone(), cloned);
      }
    }
  }

  // Third - any referenced imported type bindings need to be imported in
  for (md, name) in f.referenced_imported_bindings.iter() {
    if let Some(tf) = stale.lookup_imported_type(md, name) {
      let cloned = clone_incremental_type_fun(&tf, dest, clone_state, dest_scope);
      unsafe {
        (*dest_scope)
          .imported_type_bindings
          .entry(md.clone())
          .or_default()
          .insert(name.clone(), cloned);
      }
    }
  }

  let module_scope = stale_module.get_module_scope();

  // Fourth - prepopulate the global function types
  for name in f.global_functions_referenced.iter() {
    let name = *name;
    if let Some(ty) = module_scope.lookup_symbol(Symbol::from_global(name)) {
      let cloned = unsafe { clone_incremental_type(ty, dest, clone_state, dest_scope) };
      unsafe {
        (*dest_scope)
          .bindings
          .insert(Symbol::from_global(name), binding_from_type(cloned));
      }
    } else {
      let bt = dest.add_type(BlockedType::default());
      unsafe {
        (*dest_scope)
          .bindings
          .insert(Symbol::from_global(name), binding_from_type(bt));
      }
    }
  }

  // Fifth - prepopulate the globals here
  for (name, def) in f.global_defs_to_pre_populate.iter() {
    let name = *name;
    let def: *const Def = *def;
    if let Some(ty) = module_scope.lookup_symbol(Symbol::from_global(name)) {
      let cloned = unsafe { clone_incremental_type(ty, dest, clone_state, dest_scope) };
      unsafe {
        *(*dest_scope).lvalue_types.get_or_insert(def) = cloned;
      }
    } else if let Some(ty) = unsafe { (*dest_scope).lookup_symbol(Symbol::from_global(name)) } {
      // This branch is a little strange - we are looking up a symbol in the destScope.
      // This scope has no parent pointer, and only cloned types are written to it, so this is a
      // safe operation to do without cloning.
      unsafe {
        *(*dest_scope).lvalue_types.get_or_insert(def) = ty;
      }
    }
  }

  // Finally, clone the returnType on the staleScope. This helps avoid potential leaks of free types.
  if !stale.return_type.is_null() {
    let cloned =
      unsafe { clone_incremental_type_pack(stale.return_type, dest, clone_state, dest_scope) };
    unsafe {
      (*dest_scope).return_type = cloned;
    }
  }
}
