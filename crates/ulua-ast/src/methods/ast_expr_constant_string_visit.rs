use crate::{
  records::ast_expr_constant_string::AstExprConstantString,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstExprConstantString, ExprConstantString);
