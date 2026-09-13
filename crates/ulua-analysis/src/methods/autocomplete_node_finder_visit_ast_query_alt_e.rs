use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;

impl AutocompleteNodeFinder {
  pub fn visit_ast_type_pack(&mut self, _type_pack: *mut AstTypePack) -> bool {
    true
  }
}
