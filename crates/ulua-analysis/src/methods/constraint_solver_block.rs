use ulua_common::fflag;

use crate::{
  functions::{follow_type, follow_type_pack},
  records::{
    blocked_constraint_registry::register_constraint, constraint::Constraint,
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
      // Safety: cgraph 为构造期 SolverParams 注入的非空 ConstraintGraph 裸句柄
      // （LuauConstraintGraph 开启期布线不变量，见 constraint_solver_run 同款
      // 断言），图与 solver 同生命周期；target/constraint 由调用方按 C++ 契约
      // 传入，指向 Box 持有、地址稳定的存活 Constraint 节点，&mut 形态仅为适配
      // 被调签名——add_dependency 内部只 `as *const` 取地址作顶点身份键，不写
      // 约束本体，本窗口内对二者别无活跃借用。
      unsafe {
        (*self.cgraph).add_dependency_of_constraint_constraint(
          &mut *(target as *mut Constraint),
          &mut *(constraint as *mut Constraint),
        )
      }
    } else {
      self.deprecate_d_block(
        BlockedConstraintId::V2(register_constraint(target)),
        constraint,
      )
    };

    // Safety: logger 为构造期注入的可空 DcrLogger 裸句柄；as_mut 自带判空
    // （null → None 不解引用），非空时指向会话期存活 logger，&mut 再借用止于
    // 本次 push 返回，此刻无其它 logger 借用并存。
    if new_block && let Some(logger) = unsafe { self.logger.as_mut() } {
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
      // Safety: cgraph 非空不变量同前——构造期注入、图活过本阻塞调用；
      // BlockedConstraintId 变体是 Copy 句柄/裸指针身份键，仅入图查表，
      // 不经其解引用。
      unsafe {
        (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
          BlockedConstraintId::V0(target),
          BlockedConstraintId::V2(register_constraint(constraint)),
        )
      }
    } else {
      self.deprecate_d_block(BlockedConstraintId::V0(target), constraint)
    };

    // Safety: logger 可空裸句柄，as_mut 判空收敛；非空时为会话期存活单例，
    // &mut 再借用止于本次 push，无并存别名。
    if new_block && let Some(logger) = unsafe { self.logger.as_mut() } {
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
      // Safety: 同 type 分支——cgraph 构造期布线非空（flag 开启期不变量），
      // 图与 solver 同寿；两个 BlockedConstraintId 仅为图顶点身份键，不写节点。
      unsafe {
        (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
          BlockedConstraintId::V1(target),
          BlockedConstraintId::V2(register_constraint(constraint)),
        )
      }
    } else {
      self.deprecate_d_block(BlockedConstraintId::V1(target), constraint)
    };

    // Safety: logger as_mut 判空收敛（null → None）；非空指向会话期存活
    // DcrLogger，可变借用止于本次 push 返回。
    if new_block && let Some(logger) = unsafe { self.logger.as_mut() } {
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
