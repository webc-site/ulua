/// C++ `AutocompleteEntryMap autocompleteProps(...)` (AutocompleteCore.cpp:581-593),
/// the public six-parameter overload returning a fresh map.
use alloc::vec::Vec;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  enums::prop_index_type::PropIndexType,
  functions::autocomplete_props_autocomplete_core_alt_b::autocomplete_props as autocomplete_props_seed,
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
) -> AutocompleteEntryMap {
  let mut result: AutocompleteEntryMap = Default::default();
  autocomplete_props_seed(
    module,
    type_arena,
    builtin_types,
    ty,
    index_type,
    nodes,
    &mut result,
  );
  result
}
