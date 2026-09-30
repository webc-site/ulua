use core::ptr::null_mut;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::variant::Variant3};

use crate::{
  records::{
    blocked_constraint_registry::register_constraint, checkpoint::Checkpoint,
    constraint::Constraint, constraint_generator::ConstraintGenerator,
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
  let mut previous: *mut Constraint = null_mut();

  for i in start.offset..end.offset {
    let constraint = cg.constraints[i];
    let constraint_vertex: BlockedConstraintId =
      Variant3::V2(register_constraint(constraint as *const Constraint));

    // SAFETY: cg.cgraph 为构造期接线的非空 ConstraintGraph 指针且比 cg 长寿（唯一取
    // &mut 的对象）；constraint 与 target 指向约束 arena 中存活 Constraint，建边只把
    // 顶点地址存入邻接表；previous 取自更早迭代的数组元素、与当前 constraint 必为
    // 不同对象；读 (*constraint).c 判别 returns 亦落在同一 arena 存活期内，单线程串行。
    unsafe {
      (*cg.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
        constraint_vertex.clone(),
        target_vertex.clone(),
      );

      if let Some(psc) = PackSubtypeConstraint::get_if(&(*constraint).c)
        && psc.returns
      {
        if !previous.is_null() {
          let previous_vertex: BlockedConstraintId =
            Variant3::V2(register_constraint(previous as *const Constraint));
          (*cg.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
            previous_vertex,
            constraint_vertex,
          );
        }

        previous = constraint;
      }
    }
  }
}
