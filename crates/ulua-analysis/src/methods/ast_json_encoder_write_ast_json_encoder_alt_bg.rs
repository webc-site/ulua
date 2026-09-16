use ulua_ast::records::{ast_node::AstNode, ast_stat_local::AstStatLocal};

use crate::{macros::prop::PROP, records::ast_json_encoder::AstJsonEncoder};

impl AstJsonEncoder {
  pub fn write_ast_stat_local(&mut self, node: *mut AstStatLocal) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatLocal", |e| {
      PROP!(e, "vars", n.vars);
      PROP!(e, "values", n.values);
    });
  }
}
