use core::ptr::NonNull;

use crate::{
  records::{
    constraint_graph::ConstraintGraph, constraint_list::ConstraintList, type_ids::TypeIds,
  },
  type_aliases::{constraint_vertex::ConstraintVertex, type_pack_ids::TypePackIds},
};

impl ConstraintGraph {
  pub fn copy_dependencies_to_reachable_types(
    &mut self,
    original_vertex: Option<ConstraintVertex>,
    source_dependencies: NonNull<ConstraintList>,
    mutated_types: TypeIds,
    mutated_type_packs: TypePackIds,
  ) {
    let source_deps_ref = unsafe { source_dependencies.as_ref() };

    for vertex in source_deps_ref.order.iter() {
      let vertex = vertex.clone();
      let mut vertex_reverse_deps = self.find_reverse_dependency_list(vertex.clone());

      if let Some(ref original) = original_vertex {
        unsafe { vertex_reverse_deps.as_mut() }.remove(original.clone());
      }

      for sub_target in mutated_types.order.iter() {
        let sub_target = ConstraintVertex::V0(*sub_target);
        let mut ty_deps = self.find_dependency_list(sub_target.clone());

        unsafe { ty_deps.as_mut() }.insert(vertex.clone());
        unsafe { vertex_reverse_deps.as_mut() }.insert(sub_target);
      }

      for sub_pack_target in mutated_type_packs.iter() {
        let sub_pack_target = ConstraintVertex::V1(*sub_pack_target);
        let mut tp_deps = self.find_dependency_list(sub_pack_target.clone());

        unsafe { tp_deps.as_mut() }.insert(vertex.clone());
        unsafe { vertex_reverse_deps.as_mut() }.insert(sub_pack_target);
      }
    }
  }
}
