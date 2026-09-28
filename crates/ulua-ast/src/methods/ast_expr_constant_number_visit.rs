use crate::{
  records::ast_expr_constant_number::AstExprConstantNumber,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstExprConstantNumber, ExprConstantNumber);
