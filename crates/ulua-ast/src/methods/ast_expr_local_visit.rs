use crate::{
  records::ast_expr_local::AstExprLocal,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstExprLocal, ExprLocal);
