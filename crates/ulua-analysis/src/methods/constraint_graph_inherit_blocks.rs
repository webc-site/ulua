use crate::{
  records::constraint_graph::ConstraintGraph,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintGraph {
  pub fn inherit_blocks(
    &mut self,
    existing_vertex: BlockedConstraintId,
    new_vertex: BlockedConstraintId,
  ) {
    let existing_reverse_deps = self.find_reverse_dependency_list(existing_vertex.clone());
    let mut new_reverse_deps = self.find_reverse_dependency_list(new_vertex.clone());

    // Safety: `find_reverse_dependency_list` 返回值经 `NonNull::new(..).unwrap()`
    // 证非空且对齐；其指向 `constraint_lists`（PinnedStorage，`Box` 节点扩容不移动
    // 地址）中的存活 `ConstraintList`。此处共享借用贯穿循环，只读 `order` 迭代。
    let existing_reverse_deps_ref = unsafe { existing_reverse_deps.as_ref() };

    for existing_rdep in existing_reverse_deps_ref.order.iter() {
      let existing_rdep = existing_rdep.clone();

      // Safety: 写目标 `new_reverse_deps` 与迭代源指向同一 pinned arena 中的
      // 另一列表（各调用点的 new_vertex 均为刚登记的新约束，vertex→列表为双射，
      // 二者不同对象），故 `as_mut` 重建的可变借用与上方共享借用不别名；
      // 单线程串行，写仅在 insert 语句内活跃。
      unsafe { new_reverse_deps.as_mut() }.insert(existing_rdep.clone());

      let mut new_deps = self.find_dependency_list(existing_rdep.clone());
      // Safety: 同上——`find_dependency_list` 亦返回指向 pinned arena 存活列表的
      // NonNull；该可变借用于单行 insert 后随语句结束，与其它借用窗口不交叠。
      unsafe { new_deps.as_mut() }.insert(new_vertex.clone());
    }
  }
}
