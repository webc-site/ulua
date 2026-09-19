use core::ptr::null_mut;

use crate::{
  records::{
    constraint_graph::ConstraintGraph, generic_type_visitor::GenericTypeVisitorTrait,
    reference_count_initializer::ReferenceCountInitializer, type_ids::TypeIds,
  },
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, type_id::TypeId, type_pack_ids::TypePackIds,
  },
};
impl ConstraintGraph {
  pub fn copy_dependencies_of_type_id(&mut self, source: TypeId, target: TypeId) {
    let source_dependencies = self.find_dependency_list(BlockedConstraintId::V0(source));
    let mut mutated_types = TypeIds::new();
    let mut mutated_type_packs = TypePackIds::new(null_mut());

    let mut rci =
      ReferenceCountInitializer::reference_count_initializer_reference_count_initializer(
        &mut mutated_types as *mut TypeIds,
        &mut mutated_type_packs as *mut TypePackIds,
      );
    // C++ `copyDependenciesOf`：遍历 target 收集其内部的 free/blocked/PE 类型，
    // 使 target 的内层类型承接 source 原有的依赖边（此处不删除 source 自身）。
    rci.traverse_type_id(target);

    self.copy_dependencies_to_reachable_types(
      None,
      source_dependencies,
      mutated_types,
      mutated_type_packs,
    );
  }
}
