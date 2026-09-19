use ulua_ast::records::ast_stat_break::AstStatBreak;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn visit_ast_stat_break(&mut self, node: *mut AstStatBreak) -> bool {
    {
      self.write_ast_stat_break(node);
    }
    false
  }
}
