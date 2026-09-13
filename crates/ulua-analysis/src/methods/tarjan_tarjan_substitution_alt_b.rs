use crate::records::tarjan::Tarjan;

impl Tarjan {
  pub fn tarjan(&mut self) {
    const PREALLOCATION_SIZE: usize = 16;

    self.nodes.reserve(PREALLOCATION_SIZE);
    self.stack.reserve(PREALLOCATION_SIZE);
    self.edges_ty.reserve(PREALLOCATION_SIZE);
    self.edges_tp.reserve(PREALLOCATION_SIZE);
    self.worklist.reserve(PREALLOCATION_SIZE);
  }
}
