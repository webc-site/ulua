use alloc::vec::Vec;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  enums::autocomplete_context::AutocompleteContext,
  type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};
#[derive(Debug, Clone)]
pub struct AutocompleteResult {
  pub entry_map: AutocompleteEntryMap,
  pub ancestry: Vec<*mut AstNode>,
  pub context: AutocompleteContext,
}
