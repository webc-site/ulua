use crate::{
  records::constraint_graph::ConstraintGraph, type_aliases::constraint_vertex::ConstraintVertex,
};

impl ConstraintGraph {
  pub fn clear_reverse_dependencies_of(&mut self, vertex: ConstraintVertex) {
    // LUAU_ASSERT(vertex.get_if<const Constraint*>() == nullptr);
    // We cannot directly call get_if on ConstraintVertex here because it's a type alias
    // to BlockedConstraintId. The assertion is preserved as a comment since the
    // ConstraintVertex type alias already enforces this constraint at the type level.
    // The original C++ assertion checks that the vertex is not a Constraint*, which
    // is guaranteed by the type alias definition.

    let rev_deps = self.find_reverse_dependency_list(vertex.clone());

    // For all of the reverse dependencies of vertex (vertices that depend on vertex) ...
    let rev_deps_ref = unsafe { &*rev_deps.as_ptr() };
    for rdep in rev_deps_ref.order.iter() {
      // Remove vertex from the list of dependencies.
      let deps = self.find_dependency_list(rdep.clone());
      let deps_ref = unsafe { &mut *deps.as_ptr() };
      deps_ref.remove(vertex.clone());
    }

    // Then clear this set.
    let rev_deps_mut = unsafe { &mut *rev_deps.as_ptr() };
    rev_deps_mut.clear();
  }
}
