use core::ptr::null_mut;

use crate::{
  records::{
    constraint_graph::ConstraintGraph, reference_count_initializer::ReferenceCountInitializer,
    type_ids::TypeIds,
  },
  type_aliases::{
    constraint_vertex::ConstraintVertex, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_ids::TypePackIds,
  },
};
impl ConstraintGraph {
  pub fn shift_references_type_id(&mut self, source: TypeId, target: TypeId) {
    if source == target {
      return;
    }

    let source_dependencies = self.find_dependency_list(ConstraintVertex::V0(source));

    let mut mutated_types = TypeIds::new();
    let mut mutated_type_packs = TypePackIds::new(null_mut());

    let _rci = ReferenceCountInitializer::reference_count_initializer_reference_count_initializer(
      &mut mutated_types as *mut TypeIds,
      &mut mutated_type_packs as *mut TypePackIds,
    );

    self.copy_dependencies_to_reachable_types(
      Some(ConstraintVertex::V0(source)),
      source_dependencies,
      mutated_types,
      mutated_type_packs,
    );

    self.clear_reverse_dependencies_of(ConstraintVertex::V0(source));

    let _ = target;
  }

  pub fn shift_references_type_pack_id(&mut self, source: TypePackId, target: TypePackId) {
    if source == target {
      return;
    }

    let source_dependencies = self.find_dependency_list(ConstraintVertex::V1(source));

    let mut mutated_types = TypeIds::new();
    let mut mutated_type_packs = TypePackIds::new(null_mut());

    let _rci = ReferenceCountInitializer::reference_count_initializer_reference_count_initializer(
      &mut mutated_types as *mut TypeIds,
      &mut mutated_type_packs as *mut TypePackIds,
    );

    self.copy_dependencies_to_reachable_types(
      Some(ConstraintVertex::V1(source)),
      source_dependencies,
      mutated_types,
      mutated_type_packs,
    );

    self.clear_reverse_dependencies_of(ConstraintVertex::V1(source));

    let _ = target;
  }
}
