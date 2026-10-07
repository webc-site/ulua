use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::variant::Variant3};

use crate::{
  records::{
    arena_handle::alias, blocked_constraint_registry::register_constraint, checkpoint::Checkpoint,
    constraint::Constraint, constraint_generator::ConstraintGenerator,
  },
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

/// `addAllAsDependencies`/`addAllAsReverseDependencies` 同构体（cpp
/// ConstraintGenerator.cpp:140-174 相邻两函数，仅边向相反）：对 `[start,end)`
/// 区间内每条约束与 `target` 建边。`reverse=false` 时边为 `D → target`
/// （区间全部成为 target 的依赖），`reverse=true` 时为 `target → C`（阻塞
/// target 的分发）。
///
/// `target` 为 cpp `NotNull<Constraint>` 的直译：仅按地址写入约束图顶点
/// （`BlockedConstraintId::V2`），本函数不解引用；其非空、指向约束 arena
/// （bump 块、地址不移动）存活节点的前提由各调用点（紧邻
/// `add_constraint` 的返回值）保证，与 cpp 同一构造点契约。
pub(super) fn add_all_as_edges(
  start: Checkpoint,
  end: Checkpoint,
  cg: &ConstraintGenerator,
  target: *mut Constraint,
  reverse: bool,
) {
  LUAU_ASSERT!(fflag::LuauConstraintGraph.get());

  let target_vertex: BlockedConstraintId =
    Variant3::V2(register_constraint(target as *const Constraint));

  for i in start.offset..end.offset {
    let constraint = cg.constraints[i];
    let constraint_vertex: BlockedConstraintId =
      Variant3::V2(register_constraint(constraint as *const Constraint));

    if reverse {
      alias(cg.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
        target_vertex.clone(),
        constraint_vertex,
      );
    } else {
      alias(cg.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
        constraint_vertex,
        target_vertex.clone(),
      );
    }
  }
}

/// 对应 C++ `addAllAsDependencies`（ConstraintGenerator.cpp:140-151）：对
/// `[start,end)` 区间内每条约束 D 建边 `D → target`，即区间全部成为 target 的依赖。
pub fn add_all_as_dependencies(
  start: Checkpoint,
  end: Checkpoint,
  cg: &ConstraintGenerator,
  target: *mut Constraint,
) {
  add_all_as_edges(start, end, cg, target, false);
}
