use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::variant::Variant3};

use crate::{
  records::{
    blocked_constraint_registry::register_constraint, checkpoint::Checkpoint,
    constraint::Constraint, constraint_generator::ConstraintGenerator,
  },
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};
/// 对应 C++ `addAllAsDependencies`（ConstraintGenerator.cpp:140-151）：对
/// `[start,end)` 区间内每条约束 D 建边 `D → target`，即区间全部成为 target 的依赖。
///
/// `target` 为 cpp `NotNull<Constraint>` 的直译：仅按地址写入约束图顶点
/// （`BlockedConstraintId::V2`），本函数不解引用；其非空、指向约束 arena
/// （bump 块、地址不移动）存活节点的前提由各调用点（紧邻
/// `add_constraint` 的返回值）保证，与 cpp 同一构造点契约。
pub fn add_all_as_dependencies(
  start: Checkpoint,
  end: Checkpoint,
  cg: &ConstraintGenerator,
  target: *mut Constraint,
) {
  LUAU_ASSERT!(fflag::LuauConstraintGraph.get());

  let target_vertex: BlockedConstraintId =
    Variant3::V2(register_constraint(target as *const Constraint));

  for i in start.offset..end.offset {
    let dependency = cg.constraints[i];
    let dep_vertex: BlockedConstraintId =
      Variant3::V2(register_constraint(dependency as *const Constraint));

    // SAFETY: cg.cgraph 为构造期接线的非空 ConstraintGraph 指针且比 cg 长寿（唯一解引用
    // 点，仅取 &mut 调图方法）；dependency 与 target 均指向约束 arena 中存活 Constraint，
    // 加边只把顶点地址存入邻接表，不触碰节点内容，单线程串行无并发写。
    unsafe {
      (*cg.cgraph)
        .add_dependency_of_constraint_vertex_constraint_vertex(dep_vertex, target_vertex.clone());
    }
  }
}
