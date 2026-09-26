use core::ptr::NonNull;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  records::{constraint_graph::ConstraintGraph, constraint_list::ConstraintList},
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};
impl ConstraintGraph {
  pub fn find_dependency_list(&mut self, vertex: BlockedConstraintId) -> NonNull<ConstraintList> {
    if let Some(dep) = self.dependencies.find(&vertex) {
      // 唯一写点为本函数尾 `dependencies.try_insert(vertex, newlist.as_ptr())`：
      // newlist 是 PinnedStorage push 返回的存活非空句柄，登记值恒非 null。
      return NonNull::new(*dep).expect("登记值恒为 PinnedStorage 非空句柄的 as_ptr");
    }

    let ptr = self.constraint_lists.push(ConstraintList {
      present: DenseHashMap::default(),
      order: Vec::new(),
      entries: 0,
    });
    // Safety: 存储 arena push 返回槽地址（cpp NotNull 同位），恒非空。
    let newlist = NonNull::new(ptr).expect("constraint_lists.push 返回槽地址恒非空");

    let (_it, fresh) = self.dependencies.try_insert(vertex, newlist.as_ptr());
    LUAU_ASSERT!(fresh);
    newlist
  }
}
