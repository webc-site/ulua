use ulua_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

use crate::{
  macros::prop::PROP,
  records::{ast_json_encoder::AstJsonEncoder, ast_node::AstNode},
};

impl AstJsonEncoder {
  pub fn write_ast_stat_declare_extern_type(&mut self, node: *mut AstStatDeclareExternType) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatDeclareClass", |e| {
      e.write("name", &n.name);
      if let Some(ref super_name) = n.super_name {
        e.write("superName", super_name);
      }
      e.write("props", &n.props);
      e.write("indexer", &n.indexer);
    });
  }
}
