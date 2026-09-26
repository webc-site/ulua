use core::ptr::NonNull;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  records::{constraint_graph::ConstraintGraph, constraint_list::ConstraintList},
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};
impl ConstraintGraph {
  pub fn find_reverse_dependency_list(
    &mut self,
    vertex: BlockedConstraintId,
  ) -> NonNull<ConstraintList> {
    if let Some(rdep) = self.reverse_dependencies.find(&vertex) {
      // Safety: reverse_dependencies 值均来自 constraint_lists.push 的槽地址，恒非空。
      return NonNull::new(*rdep).expect("表值由 constraint_lists.push 槽地址登记，恒非空");
    }

    let ptr = self.constraint_lists.push(ConstraintList {
      present: DenseHashMap::default(),
      order: Vec::new(),
      entries: 0,
    });
    // Safety: 存储 arena push 返回槽地址（cpp NotNull 同位），恒非空。
    let newlist = NonNull::new(ptr).expect("constraint_lists.push 返回槽地址恒非空");

    let (_it, fresh) = self
      .reverse_dependencies
      .try_insert(vertex, newlist.as_ptr());
    LUAU_ASSERT!(fresh);
    newlist
  }
}
