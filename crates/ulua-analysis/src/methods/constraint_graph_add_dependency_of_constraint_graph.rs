use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::variant::Variant3};

use crate::{
  records::{
    blocked_constraint_registry::register_constraint, constraint::Constraint,
    constraint_graph::ConstraintGraph,
  },
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintGraph {
  pub fn add_dependency_of_constraint_vertex_constraint_vertex(
    &mut self,
    dependency: BlockedConstraintId,
    target: BlockedConstraintId,
  ) -> bool {
    let deps = self.find_dependency_list(target.clone());
    let reverse_deps = self.find_reverse_dependency_list(dependency.clone());

    // Safety: find_dependency_list 返回 NonNull，故 deps.as_ptr() 保证非空且对齐；其指向
    // 存活于 self.constraint_lists（bump arena，元素地址在后续 push 下不移动）的 ConstraintList。
    // 单线程此处仅取共享引用，与下一行的共享引用可并存。
    let deps_ref = unsafe { &*deps.as_ptr() };
    if deps_ref.contains(target.clone()) {
      // Safety: 同上，reverse_deps 亦源自 NonNull 契约，指向 arena 中存活的列表；
      // 与 deps_ref 同为共享借用，可重叠持有。
      let reverse_deps_ref = unsafe { &*reverse_deps.as_ptr() };
      LUAU_ASSERT!(reverse_deps_ref.contains(dependency.clone()));
      return false;
    }

    // Safety: deps_ref 共享借用已在上方 if 分支结束后失效（或于 return 前退出），此刻无
    // 其它活跃引用，可安全取得唯一可变借用；地址在 arena 中稳定，单线程无并发写。
    let deps_mut = unsafe { &mut *deps.as_ptr() };
    deps_mut.insert(dependency.clone());

    // Safety: deps_mut 的可变借用于 insert 后即失效，reverse_deps 指向 arena 中存活的列表
    // （可能与 deps 同址，但此刻无重叠活跃借用），故该可变引用唯一且有效。
    let reverse_deps_mut = unsafe { &mut *reverse_deps.as_ptr() };
    reverse_deps_mut.insert(target);

    true
  }

  pub fn add_dependency_of_constraint_constraint(
    &mut self,
    dependency: &mut Constraint,
    target: &mut Constraint,
  ) -> bool {
    let dep_vertex = Variant3::V2(register_constraint(dependency as *const Constraint));
    let target_vertex = Variant3::V2(register_constraint(target as *const Constraint));

    self.add_dependency_of_constraint_vertex_constraint_vertex(dep_vertex, target_vertex)
  }
}
