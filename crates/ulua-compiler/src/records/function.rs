use ulua_ast::records::ast_local::AstLocal;
use ulua_common::records::dense_hash_table::DenseDefault;

#[derive(Debug, Clone, Default)]
pub struct Function {
  pub(crate) id: u32,
  pub(crate) upvals: Vec<*mut AstLocal>,
  pub(crate) cost_model: u64,
  pub(crate) stack_size: u32,
  pub(crate) can_inline: bool,
  pub(crate) returns_one: bool,
}

impl DenseDefault for Function {
  fn dense_default() -> Self {
    Self::default()
  }
}
