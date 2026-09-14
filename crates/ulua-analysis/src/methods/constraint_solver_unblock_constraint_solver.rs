use core::ptr::null;

use ulua_ast::records::location::Location;
use ulua_common::{FFlag, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::constraint_solver::ConstraintSolver,
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, bound_type::BoundType, type_id::TypeId,
  },
};
impl ConstraintSolver {
  pub fn unblock_type_id_location(&mut self, ty: TypeId, location: Location) {
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
    let mut progressed = ty;

    loop {
      if seen.contains(&progressed) {
        self.ice_reporter.ice_string_location(
          "ConstraintSolver::unblock encountered a self-bound type!",
          &location,
        );
      }
      seen.insert(progressed);

      if let Some(logger) = unsafe { self.logger.as_mut() } {
        logger.pop_block_type_id(progressed);
      }

      if !FFlag::LuauConstraintGraph.get() {
        self.deprecate_d_unblock_(BlockedConstraintId::V0(progressed));
      }

      let Some(bound) = get_type_id::<BoundType>(progressed) else {
        break;
      };

      progressed = bound.bound_to;
    }

    if FFlag::LuauConstraintGraph.get() {
      unsafe {
        (*self.cgraph).unblock_type_or_pack_type_id(ty);
      }
    }
  }
}
