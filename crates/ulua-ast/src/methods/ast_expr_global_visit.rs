use crate::{
  records::ast_expr_global::AstExprGlobal,
  visit::{AstNodeRefMut, AstVisitable},
};

// cpp `AstExprGlobal::visit`：无子节点可遍历（仅含 AstName 值类型）。
impl_visitable!(AstExprGlobal, ExprGlobal);
