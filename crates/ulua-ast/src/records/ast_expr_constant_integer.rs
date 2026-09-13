#[repr(C)]
#[derive(Debug)]
pub struct AstExprConstantInteger {
  pub base: AstExpr,
  pub value: i64,
  pub parse_result: ConstantNumberParseResult,
}

impl AstNodeClass for AstExprConstantInteger {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprConstantInteger");
}
use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};
