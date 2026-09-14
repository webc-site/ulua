use core::ptr::null_mut;

use crate::{
  records::{
    constraint_graph::ConstraintGraph, reference_count_initializer::ReferenceCountInitializer,
    type_ids::TypeIds,
  },
  type_aliases::{
    constraint_vertex::ConstraintVertex, type_id::TypeId, type_pack_ids::TypePackIds,
  },
};
impl ConstraintGraph {
  pub fn copy_dependencies_of_type_id(&mut self, source: TypeId, target: TypeId) {
    let source_dependencies = self.find_dependency_list(ConstraintVertex::V0(source));
    let mut mutated_types = TypeIds::new();
    let mut mutated_type_packs = TypePackIds::new(null_mut());

    let _rci = ReferenceCountInitializer::reference_count_initializer_reference_count_initializer(
      &mut mutated_types as *mut TypeIds,
      &mut mutated_type_packs as *mut TypePackIds,
    );

    self.copy_dependencies_to_reachable_types(
      None,
      source_dependencies,
      mutated_types,
      mutated_type_packs,
    );

    let _ = target;
  }
}
