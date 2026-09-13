use ulua_ast::records::ast_expr_unary::AstExprUnaryOp;
use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn get_unary_op(&mut self, op: AstExprUnaryOp) -> LuauOpcode {
    match op {
      AstExprUnaryOp::Not => LuauOpcode::LOP_NOT,
      AstExprUnaryOp::Minus => LuauOpcode::LOP_MINUS,
      AstExprUnaryOp::Len => LuauOpcode::LOP_LENGTH,
    }
  }
}
