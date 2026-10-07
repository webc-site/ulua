use ulua_common::fflag;

use crate::{
  functions::{follow_type, follow_type_pack},
  records::{
    arena_handle::{alias, alias_opt_mut},
    blocked_constraint_registry::register_constraint,
    constraint::Constraint,
    constraint_solver::ConstraintSolver,
  },
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl ConstraintSolver {
  pub fn block_not_null_constraint_not_null_constraint(
    &mut self,
    target: *const Constraint,
    constraint: *const Constraint,
  ) {
    let new_block = if fflag::LuauConstraintGraph.get() {
      alias(self.cgraph).add_dependency_of_constraint_constraint(
        alias(target.cast_mut()),
        alias(constraint.cast_mut()),
      )
    } else {
      self.deprecate_d_block(
        BlockedConstraintId::V2(register_constraint(target)),
        constraint,
      )
    };

    if new_block && let Some(logger) = alias_opt_mut(self.logger) {
      logger.push_block_not_null_constraint_not_null_constraint(constraint, target);
    }
  }

  pub fn block_type_id_not_null_constraint(
    &mut self,
    target: TypeId,
    constraint: *const Constraint,
  ) -> bool {
    let target = follow_type::follow(target);
    let new_block = if fflag::LuauConstraintGraph.get() {
      alias(self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
        BlockedConstraintId::V0(target),
        BlockedConstraintId::V2(register_constraint(constraint)),
      )
    } else {
      self.deprecate_d_block(BlockedConstraintId::V0(target), constraint)
    };

    if new_block && let Some(logger) = alias_opt_mut(self.logger) {
      logger.push_block_not_null_constraint_type_id(constraint, target);
    }
    false
  }

  pub(crate) fn block_type_pack_id_not_null_constraint(
    &mut self,
    target: TypePackId,
    constraint: *const Constraint,
  ) -> bool {
    let target = follow_type_pack::follow(target);
    let new_block = if fflag::LuauConstraintGraph.get() {
      alias(self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
        BlockedConstraintId::V1(target),
        BlockedConstraintId::V2(register_constraint(constraint)),
      )
    } else {
      self.deprecate_d_block(BlockedConstraintId::V1(target), constraint)
    };

    if new_block && let Some(logger) = alias_opt_mut(self.logger) {
      logger.push_block_not_null_constraint_type_pack_id(constraint, target);
    }
    false
  }
}

/// try_dispatch 家族的「blocked 早退」门面：谓词命中即登记「约束阻塞于该
/// 句柄」的依赖并立即 `return`（值仍为助手的 `false`），谓词→登记→早退的
/// 短路次序与收口前逐字同源；`type`/`pack` 选定谓词与助手，四参形态（`=>`
/// 分隔）保留 cpp 同源站点被测与阻塞对象拼写不同（如 `fn_ty`/`c.fn_type`）。
#[macro_export]
macro_rules! blocked_early_exit {
  // TypeId 三参形态。
  ($s:expr, type $t:expr, $c:expr) => {
    if $s.is_blocked_type_id($t) {
      return $s.block_type_id_not_null_constraint($t, $c);
    }
  };
  // TypeId 被测/阻塞对象分离形态。
  ($s:expr, type $t:expr => $b:expr, $c:expr) => {
    if $s.is_blocked_type_id($t) {
      return $s.block_type_id_not_null_constraint($b, $c);
    }
  };
  // TypePackId 形态。
  ($s:expr, pack $t:expr, $c:expr) => {
    if $s.is_blocked_type_pack_id($t) {
      return $s.block_type_pack_id_not_null_constraint($t, $c);
    }
  };
}
