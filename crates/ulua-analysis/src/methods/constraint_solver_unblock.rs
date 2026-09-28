use ulua_ast::records::location::Location;
use ulua_common::{fflag, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::get_type,
  records::constraint_solver::ConstraintSolver,
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, bound_type::BoundType, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl ConstraintSolver {
  pub fn unblock_type_id_location(&mut self, ty: TypeId, location: Location) {
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
    let mut progressed = ty;

    loop {
      if seen.contains(&progressed) {
        self.ice_reporter.ice_string_location(
          "ConstraintSolver::unblock encountered a self-bound type!",
          &location,
        );
      }
      seen.insert(progressed);

      // Safety: self.logger 为可空 `*mut DcrLogger`（对应 C++ `Logger*`，默认 null）；`as_mut()`
      // 内建判空，非空时指向会话期存活的 DcrLogger，重建的独占 &mut 借用仅存活至 pop_block 调用返回，
      // 单线程无别名冲突。
      if let Some(logger) = unsafe { self.logger.as_mut() } {
        logger.pop_block_type_id(progressed);
      }

      if !fflag::LuauConstraintGraph.get() {
        self.deprecate_d_unblock_(BlockedConstraintId::V0(progressed));
      }

      let Some(bound) = get_type::get::<BoundType>(progressed) else {
        break;
      };

      progressed = bound.bound_to;
    }

    if fflag::LuauConstraintGraph.get() {
      // Safety: 仅当 LuauConstraintGraph 开启时进入——此时 self.cgraph 在构造期被接线为指向存活
      // ConstraintGraph 的非空指针（关旗时保持 null 且本分支不进入，与 block/has_unresolved 等同旗
      // 守卫一致）；重建 &mut 调用 unblock，单线程下此刻无并存图借用。
      unsafe {
        (*self.cgraph).unblock_type_or_pack_type_id(ty);
      }
    }
  }

  pub(crate) fn unblock_type_pack_id_location(&mut self, tp: TypePackId, _location: Location) {
    // Safety: self.logger 为可空 `*mut DcrLogger`；`as_mut()` 内建判空，非空时指向会话存活
    // DcrLogger，&mut 借用止于 pop_block 调用（单线程无别名）。
    if let Some(logger) = unsafe { self.logger.as_mut() } {
      logger.pop_block_type_pack_id(tp);
    }

    if fflag::LuauConstraintGraph.get() {
      // Safety: 仅 LuauConstraintGraph 开启时进入，此时 self.cgraph 为构造期接线的存活 ConstraintGraph
      // 非空指针（关旗时为 null 且不进入本分支）；重建 &mut 调用 unblock，单线程无并存图借用。
      unsafe {
        (*self.cgraph).unblock_type_or_pack_type_pack_id(tp);
      }
    } else {
      self.deprecate_d_unblock_(BlockedConstraintId::V1(tp));
    }
  }
}
