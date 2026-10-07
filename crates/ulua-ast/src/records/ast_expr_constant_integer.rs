#[repr(C)]
#[derive(Debug)]
pub struct AstExprConstantInteger {
  pub base: AstExpr,
  pub value: i64,
  pub parse_result: ConstantNumberParseResult,
}

use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult, records::ast_expr::AstExpr,
};
