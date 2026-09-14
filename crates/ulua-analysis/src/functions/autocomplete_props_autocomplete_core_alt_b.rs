/// C++ `static void autocompleteProps(...)` (AutocompleteCore.cpp:567-579), the
/// seven-parameter overload that seeds `seen` and forwards to the recursive one.
use alloc::vec::Vec;
use std::collections::HashSet;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  enums::prop_index_type::PropIndexType,
  functions::autocomplete_props_autocomplete_core::{
    AutocompletePropsCtx, AutocompletePropsStep, autocomplete_props as autocomplete_props_full,
  },
  records::{builtin_types::BuiltinTypes, module::Module, type_arena::TypeArena},
  type_aliases::{autocomplete_entry_map::AutocompleteEntryMap, type_id::TypeId},
};
pub fn autocomplete_props(
  module: &Module,
  type_arena: *mut TypeArena,
  builtin_types: &BuiltinTypes,
  ty: TypeId,
  index_type: PropIndexType,
  nodes: &Vec<*mut AstNode>,
  result: &mut AutocompleteEntryMap,
) {
  let mut seen: HashSet<TypeId> = HashSet::new();
  autocomplete_props_full(
    &AutocompletePropsCtx {
      module,
      type_arena,
      builtin_types,
      root_ty: ty,
      index_type,
      nodes,
    },
    AutocompletePropsStep {
      ty,
      result,
      seen: &mut seen,
      containing_extern_type: None,
    },
  );
}
