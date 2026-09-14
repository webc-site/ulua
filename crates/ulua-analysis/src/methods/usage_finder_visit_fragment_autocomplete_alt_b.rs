use ulua_ast::records::ast_type::AstType;

use crate::records::usage_finder::UsageFinder;

impl UsageFinder {
  pub fn visit_ast_type(&mut self, _node: *mut AstType) -> bool {
    true
  }
}
