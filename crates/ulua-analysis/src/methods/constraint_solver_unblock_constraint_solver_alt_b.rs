use ulua_ast::records::location::Location;
use ulua_common::FFlag;

use crate::{
  records::constraint_solver::ConstraintSolver,
  type_aliases::{blocked_constraint_id::BlockedConstraintId, type_pack_id::TypePackId},
};

impl ConstraintSolver {
  pub(crate) fn unblock_type_pack_id_location(&mut self, tp: TypePackId, _location: Location) {
    if let Some(logger) = unsafe { self.logger.as_mut() } {
      logger.pop_block_type_pack_id(tp);
    }

    if FFlag::LuauConstraintGraph.get() {
      unsafe {
        (*self.cgraph).unblock_type_or_pack_type_pack_id(tp);
      }
    } else {
      self.deprecate_d_unblock_(BlockedConstraintId::V1(tp));
    }
  }
}
