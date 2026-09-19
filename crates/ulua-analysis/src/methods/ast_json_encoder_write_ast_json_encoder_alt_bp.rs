use ulua_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

use crate::{
  macros::prop::PROP,
  records::{ast_json_encoder::AstJsonEncoder, ast_node::AstNode},
};

impl AstJsonEncoder {
  pub fn write_ast_stat_declare_global(&mut self, node: *mut AstStatDeclareGlobal) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatDeclareGlobal", |e| {
      PROP!(e, name);
      PROP!(e, name_location);
      PROP!(e, type_);
    });
  }
}
