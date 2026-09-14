use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::constraint_graph::ConstraintGraph, type_aliases::constraint_vertex::ConstraintVertex,
};

impl ConstraintGraph {
  pub fn add_dependency_of_constraint_vertex_constraint_vertex(
    &mut self,
    dependency: ConstraintVertex,
    target: ConstraintVertex,
  ) -> bool {
    let deps = self.find_dependency_list(target.clone());
    let reverse_deps = self.find_reverse_dependency_list(dependency.clone());

    let deps_ref = unsafe { &*deps.as_ptr() };
    if deps_ref.contains(target.clone()) {
      let reverse_deps_ref = unsafe { &*reverse_deps.as_ptr() };
      LUAU_ASSERT!(reverse_deps_ref.contains(dependency.clone()));
      return false;
    }

    let deps_mut = unsafe { &mut *deps.as_ptr() };
    deps_mut.insert(dependency.clone());

    let reverse_deps_mut = unsafe { &mut *reverse_deps.as_ptr() };
    reverse_deps_mut.insert(target);

    true
  }
}
