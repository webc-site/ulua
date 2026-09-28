use alloc::vec::Vec;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  enums::autocomplete_context::AutocompleteContext,
  records::autocomplete_result::AutocompleteResult,
  type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};

impl AutocompleteResult {
  pub fn new() -> Self {
    Self::autocomplete_result_autocomplete_entry_map_vector_ast_node_autocomplete_context(
      AutocompleteEntryMap::default(),
      Vec::new(),
      AutocompleteContext::Unknown,
    )
  }
}

impl Default for AutocompleteResult {
  fn default() -> Self {
    Self::new()
  }
}

impl AutocompleteResult {
  pub fn autocomplete_result_autocomplete_entry_map_vector_ast_node_autocomplete_context(
    entry_map: AutocompleteEntryMap,
    ancestry: Vec<*mut AstNode>,
    context: AutocompleteContext,
  ) -> Self {
    Self {
      entry_map,
      ancestry,
      context,
    }
  }
}
