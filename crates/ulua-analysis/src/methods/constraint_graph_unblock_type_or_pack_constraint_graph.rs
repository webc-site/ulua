use crate::{
  functions::follow_type::follow_type_id,
  records::constraint_graph::ConstraintGraph,
  type_aliases::{constraint_vertex::ConstraintVertex, type_id::TypeId},
};

impl ConstraintGraph {
  pub fn unblock_type_or_pack_type_id(&mut self, vertex: TypeId) {
    self.repair_type_references_type_id(vertex);
    let followed = follow_type_id(vertex);
    self.clear_reverse_dependencies_of(ConstraintVertex::V0(followed));
  }
}
