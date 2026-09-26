use crate::records::count_ast_nodes::CountAstNodes;

impl CountAstNodes {
  pub fn visit(&mut self) -> bool {
    self.count += 1;
    true
  }
}
