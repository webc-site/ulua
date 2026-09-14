//! Source: `Analysis/src/AstJsonEncoder.cpp:1495-1499` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_type_pack::AstTypePack};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn visit_ast_type_pack(&mut self, node: *mut AstTypePack) -> bool {
    unsafe { self.write_ast_node(node as *mut AstNode) };
    false
  }
}
