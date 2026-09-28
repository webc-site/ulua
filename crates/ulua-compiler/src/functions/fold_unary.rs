use ulua_ast::records::ast_expr_unary::AstExprUnaryOp;

use crate::{functions::c_const::cvar, records::constant::Constant};

/// C++ `foldUnary`：折叠一元运算，未折叠时返回 `Constant::Unknown`
/// （cpp 侧表现为不写出参 `result`）
pub fn fold_unary(op: AstExprUnaryOp, arg: &Constant) -> Constant {
  match op {
    AstExprUnaryOp::Not if !arg.is_unknown() => Constant::Boolean(!arg.is_truthful()),
    AstExprUnaryOp::Minus => match arg {
      Constant::Number(v) => Constant::Number(-v),
      Constant::Vector(v) => Constant::Vector([-v[0], -v[1], -v[2], -v[3]]),
      _ => cvar(),
    },
    AstExprUnaryOp::Len => match arg {
      Constant::Str(s) => Constant::Number(f64::from(s.len)),
      _ => cvar(),
    },
    _ => cvar(),
  }
}
