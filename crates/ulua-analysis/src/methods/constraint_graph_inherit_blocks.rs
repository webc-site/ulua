use crate::{
  records::constraint_graph::ConstraintGraph, type_aliases::constraint_vertex::ConstraintVertex,
};

impl ConstraintGraph {
  pub fn inherit_blocks(
    &mut self,
    existing_vertex: ConstraintVertex,
    new_vertex: ConstraintVertex,
  ) {
    let existing_reverse_deps = self.find_reverse_dependency_list(existing_vertex.clone());
    let mut new_reverse_deps = self.find_reverse_dependency_list(new_vertex.clone());

    let existing_reverse_deps_ref = unsafe { existing_reverse_deps.as_ref() };

    for existing_rdep in existing_reverse_deps_ref.order.iter() {
      let existing_rdep = existing_rdep.clone();

      unsafe { new_reverse_deps.as_mut() }.insert(existing_rdep.clone());

      let mut new_deps = self.find_dependency_list(existing_rdep.clone());
      unsafe { new_deps.as_mut() }.insert(new_vertex.clone());
    }
  }
}
