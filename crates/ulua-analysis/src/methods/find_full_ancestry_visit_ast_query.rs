use ulua_ast::records::{ast_node::AstNode, ast_type::AstType};

use crate::records::find_full_ancestry::FindFullAncestry;

impl FindFullAncestry {
  pub fn visit_ast_type(&mut self, r#type: *mut AstType) -> bool {
    if self.include_types {
      unsafe { self.visit_ast_node(r#type as *mut AstNode) }
    } else {
      false
    }
  }
}
