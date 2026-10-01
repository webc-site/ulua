use ulua_ast::records::ast_node::AstNode;
extern crate alloc;

use alloc::vec::Vec;

use crate::{
  enums::autocomplete_context::AutocompleteContext,
  functions::autocomplete_autocomplete_core::keywords_result,
  records::autocomplete_result::AutocompleteResult,
};

pub fn autocomplete_while_loop_keywords(ancestry: Vec<*mut AstNode>) -> AutocompleteResult {
  keywords_result(&["do", "and", "or"], ancestry, AutocompleteContext::Keyword)
}
