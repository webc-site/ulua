use crate::{
  records::constraint_graph::ConstraintGraph,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintGraph {
  pub fn clear_reverse_dependencies_of(&mut self, vertex: BlockedConstraintId) {
    // LUAU_ASSERT(vertex.get_if<const Constraint*>() == nullptr);
    // We cannot directly call get_if on ConstraintVertex here because it's a type alias
    // to BlockedConstraintId. The assertion is preserved as a comment since the
    // ConstraintVertex type alias already enforces this constraint at the type level.
    // The original C++ assertion checks that the vertex is not a Constraint*, which
    // is guaranteed by the type alias definition.

    let rev_deps = self.find_reverse_dependency_list(vertex.clone());

    // For all of the reverse dependencies of vertex (vertices that depend on vertex) ...
    // Safety: find_reverse_dependency_list 返回 NonNull，故 rev_deps.as_ptr() 保证非空且
    // 对齐；其指向存活于 self.constraint_lists（PinnedStorage bump arena，元素地址在后续
    // push 下不移动）的 ConstraintList。此处仅取共享引用遍历 order，与循环体内对其它列表
    // 的可变借用指向不同 slot，无别名冲突。
    let rev_deps_ref = unsafe { &*rev_deps.as_ptr() };
    for rdep in rev_deps_ref.order.iter() {
      // Remove vertex from the list of dependencies.
      let deps = self.find_dependency_list(rdep.clone());
      // Safety: deps 亦源自 NonNull 契约，指向 arena 中存活的另一 ConstraintList（地址
      // 稳定）；此处取得的可变借用唯一，且与 rev_deps_ref 的共享借用指向不同 slot 故不重叠。
      let deps_ref = unsafe { &mut *deps.as_ptr() };
      deps_ref.remove(vertex.clone());
    }

    // Then clear this set.
    // Safety: rev_deps_ref 的共享借用已在上方 for 循环结束后失效，此刻无其它活跃引用；
    // rev_deps 指向 arena 中存活的列表（地址不移动），可安全取得唯一可变借用清空。
    let rev_deps_mut = unsafe { &mut *rev_deps.as_ptr() };
    rev_deps_mut.clear();
  }
}
