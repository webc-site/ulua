use core::ptr::NonNull;

use crate::{
  records::{
    constraint_graph::ConstraintGraph, constraint_list::ConstraintList, type_ids::TypeIds,
  },
  type_aliases::{blocked_constraint_id::BlockedConstraintId, type_pack_ids::TypePackIds},
};

impl ConstraintGraph {
  pub fn copy_dependencies_to_reachable_types(
    &mut self,
    original_vertex: Option<BlockedConstraintId>,
    source_dependencies: NonNull<ConstraintList>,
    mutated_types: TypeIds,
    mutated_type_packs: TypePackIds,
  ) {
    // Safety: `source_dependencies` 各调用点均由 `find_dependency_list` 取得，
    // 指向本图 `PinnedStorage<ConstraintList>` 中的固定槽位——元素地址 push 后
    // 永不移动、随图存活；迭代期间下方仅增删其他键位的列表（上游 C++ 不变量：
    // 插入目标列表与源 order 不相交），共享只读借用无并发可变访问。
    let source_deps_ref = unsafe { source_dependencies.as_ref() };

    for vertex in source_deps_ref.order.iter() {
      let vertex = vertex.clone();
      let mut vertex_reverse_deps = self.find_reverse_dependency_list(vertex.clone());

      if let Some(ref original) = original_vertex {
        // Safety: NonNull 指向 PinnedStorage 固定槽位、非空存活；as_mut 重建
        // 的独占短借用止于本语句，单线程串行无并存别名。
        unsafe { vertex_reverse_deps.as_mut() }.remove(original.clone());
      }

      for sub_target in mutated_types.order.iter() {
        let sub_target = BlockedConstraintId::V0(*sub_target);
        let mut ty_deps = self.find_dependency_list(sub_target.clone());

        // Safety: 同上——列表在 PinnedStorage 固定槽位，借用随语句结束；
        // 与下一条 reverse 列表写借用时序串行、不并存。
        unsafe { ty_deps.as_mut() }.insert(vertex.clone());
        // Safety: 同上——NonNull 槽位非空存活，独占短借用止于本语句。
        unsafe { vertex_reverse_deps.as_mut() }.insert(sub_target);
      }

      for sub_pack_target in mutated_type_packs.iter() {
        let sub_pack_target = BlockedConstraintId::V1(*sub_pack_target);
        let mut tp_deps = self.find_dependency_list(sub_pack_target.clone());

        // Safety: 同上——PinnedStorage 固定槽位、独占短借用随语句结束。
        unsafe { tp_deps.as_mut() }.insert(vertex.clone());
        // Safety: 同上——reverse 列表写借用与上一条用时序串行。
        unsafe { vertex_reverse_deps.as_mut() }.insert(sub_pack_target);
      }
    }
  }
}
