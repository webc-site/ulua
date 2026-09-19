use alloc::vec::Vec;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  enums::autocomplete_context::AutocompleteContext,
  records::autocomplete_result::AutocompleteResult,
  type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};

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
