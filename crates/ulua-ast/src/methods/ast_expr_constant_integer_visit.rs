use crate::{
  records::ast_expr_constant_integer::AstExprConstantInteger,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstExprConstantInteger, ExprConstantInteger);
