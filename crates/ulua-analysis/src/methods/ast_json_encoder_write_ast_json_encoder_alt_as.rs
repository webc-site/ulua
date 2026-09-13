//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:582:ast_json_encoder_write`
//! Source: `Analysis/src/AstJsonEncoder.cpp:582-593` (hand-ported)
use ulua_ast::records::ast_expr_unary::AstExprUnaryOp;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_expr_unary_op(&mut self, op: AstExprUnaryOp) {
    match op {
      AstExprUnaryOp::Not => self.write_string("Not"),
      AstExprUnaryOp::Minus => self.write_string("Minus"),
      AstExprUnaryOp::Len => self.write_string("Len"),
    }
  }
}
