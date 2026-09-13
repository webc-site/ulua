//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:608:ast_json_encoder_write`
//! Source: `Analysis/src/AstJsonEncoder.cpp:608-647` (hand-ported)
use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_expr_binary_op(&mut self, op: AstExprBinaryOp) {
    match op {
      AstExprBinaryOp::Add => self.write_string("Add"),
      AstExprBinaryOp::Sub => self.write_string("Sub"),
      AstExprBinaryOp::Mul => self.write_string("Mul"),
      AstExprBinaryOp::Div => self.write_string("Div"),
      AstExprBinaryOp::FloorDiv => self.write_string("FloorDiv"),
      AstExprBinaryOp::Mod => self.write_string("Mod"),
      AstExprBinaryOp::Pow => self.write_string("Pow"),
      AstExprBinaryOp::Concat => self.write_string("Concat"),
      AstExprBinaryOp::CompareNe => self.write_string("CompareNe"),
      AstExprBinaryOp::CompareEq => self.write_string("CompareEq"),
      AstExprBinaryOp::CompareLt => self.write_string("CompareLt"),
      AstExprBinaryOp::CompareLe => self.write_string("CompareLe"),
      AstExprBinaryOp::CompareGt => self.write_string("CompareGt"),
      AstExprBinaryOp::CompareGe => self.write_string("CompareGe"),
      AstExprBinaryOp::And => self.write_string("And"),
      AstExprBinaryOp::Or => self.write_string("Or"),
      _ => LUAU_ASSERT!(false),
    }
  }
}
