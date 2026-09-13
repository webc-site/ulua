use crate::{
  records::{
    constraint_graph::ConstraintGraph, primitive_type_constraint::PrimitiveTypeConstraint,
  },
  type_aliases::{constraint_v::ConstraintVMember, constraint_vertex::ConstraintVertex},
};

impl ConstraintGraph {
  pub fn has_unsolved_dependencies(&mut self, vertex: ConstraintVertex) -> bool {
    let deps = self.find_dependency_list(vertex.clone());

    if let Some(c) = vertex.get_if_2() {
      let constraint = unsafe { &**c };
      if PrimitiveTypeConstraint::get_if(&constraint.c).is_some() {
        return unsafe { deps.as_ref().size() > 1 };
      }
    }

    unsafe { deps.as_ref().size() > 0 }
  }
}
