use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::records::usage_finder::UsageFinder;

impl UsageFinder {
  pub fn visit_ast_type_pack(&mut self, _node: *mut AstTypePack) -> bool {
    true
  }
}
