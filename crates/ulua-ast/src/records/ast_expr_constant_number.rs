use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprConstantNumber {
  pub base: AstExpr,
  pub value: f64,
  pub parse_result: ConstantNumberParseResult,
}

impl AstNodeClass for AstExprConstantNumber {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprConstantNumber");
}
