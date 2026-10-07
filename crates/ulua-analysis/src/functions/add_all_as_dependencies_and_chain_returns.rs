use core::ptr::null_mut;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::variant::Variant3};

use crate::{
  records::{
    arena_handle::{alias, alias_ref},
    blocked_constraint_registry::register_constraint,
    checkpoint::Checkpoint,
    constraint::Constraint,
    constraint_generator::ConstraintGenerator,
    pack_subtype_constraint::PackSubtypeConstraint,
  },
  type_aliases::{blocked_constraint_id::BlockedConstraintId, constraint_v::ConstraintVMember},
};
/// 对应 C++ `addAllAsDependenciesAndChainReturns`（ConstraintGenerator.cpp:182-210）：
/// 对 `[start,end)` 区间内的每条约束 C 建边 `C → target`，并把带 `returns` 标志的
/// `PackSubtypeConstraint` 依次串接（前一条 → 当前条）。
///
/// `target` 为 cpp `NotNull<Constraint>` 的直译：仅按地址写入约束图顶点
/// （`BlockedConstraintId::V2`），本函数不解引用该指针；其非空、指向约束 arena
/// （bump 块、地址不移动）存活节点的前提由各调用点（紧邻 `add_constraint` 的
/// 返回值）保证，与 cpp 同一构造点契约。
pub fn add_all_as_dependencies_and_chain_returns(
  start: Checkpoint,
  end: Checkpoint,
  cg: &ConstraintGenerator,
  target: *mut Constraint,
) {
  LUAU_ASSERT!(fflag::LuauConstraintGraph.get());

  let target_vertex: BlockedConstraintId =
    Variant3::V2(register_constraint(target as *const Constraint));
  // 既有约定（review.md §2）：previous 空 = 链头哨兵，与上方注册的 Constraint 指针身份同面，保留。
  let mut previous: *mut Constraint = null_mut();

  for i in start.offset..end.offset {
    let constraint = cg.constraints[i];
    let constraint_vertex: BlockedConstraintId =
      Variant3::V2(register_constraint(constraint as *const Constraint));

    alias(cg.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
      constraint_vertex.clone(),
      target_vertex.clone(),
    );

    if let Some(psc) = PackSubtypeConstraint::get_if(&alias_ref(constraint).c)
      && psc.returns
    {
      if !previous.is_null() {
        let previous_vertex: BlockedConstraintId =
          Variant3::V2(register_constraint(previous as *const Constraint));
        alias(cg.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
          previous_vertex,
          constraint_vertex,
        );
      }

      previous = constraint;
    }
  }
}
