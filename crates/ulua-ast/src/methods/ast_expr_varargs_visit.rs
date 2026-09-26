use crate::{
  records::ast_expr_varargs::AstExprVarargs,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstExprVarargs, ExprVarargs);
