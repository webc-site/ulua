use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult, records::ast_expr::AstExpr,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprConstantNumber {
  pub base: AstExpr,
  pub value: f64,
  pub parse_result: ConstantNumberParseResult,
}
