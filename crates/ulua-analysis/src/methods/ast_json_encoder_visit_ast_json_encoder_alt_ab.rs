use ulua_ast::records::ast_stat_continue::AstStatContinue;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn visit_ast_stat_continue(&mut self, node: *mut AstStatContinue) -> bool {
    {
      self.write_ast_stat_continue(node);
    }
    false
  }
}
