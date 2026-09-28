use ulua_ast::records::{
  ast_expr_global::AstExprGlobal, ast_name::AstName, ast_visitor::AstVisitor,
};
use ulua_common::records::dense_hash_set::DenseHashSet;
#[derive(Debug, Clone)]
pub struct GlobalNameCollector {
  pub(crate) names: DenseHashSet<AstName>,
}

impl AstVisitor for GlobalNameCollector {
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.names.insert(node.name);
    true
  }
}

impl GlobalNameCollector {
  pub fn new() -> Self {
    Self {
      names: DenseHashSet::default(),
    }
  }
}

impl Default for GlobalNameCollector {
  fn default() -> Self {
    Self::new()
  }
}
