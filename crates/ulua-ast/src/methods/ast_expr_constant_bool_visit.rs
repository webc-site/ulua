use crate::{
  records::ast_expr_constant_bool::AstExprConstantBool,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstExprConstantBool, ExprConstantBool);
