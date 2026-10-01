use crate::{
  records::ast_expr_constant_nil::AstExprConstantNil,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstExprConstantNil, ExprConstantNil);
