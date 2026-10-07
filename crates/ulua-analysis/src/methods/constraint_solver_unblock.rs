use ulua_ast::records::location::Location;
use ulua_common::{fflag, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::get_type,
  records::{
    arena_handle::{alias, alias_opt_mut},
    constraint_solver::ConstraintSolver,
  },
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

      if let Some(logger) = alias_opt_mut(self.logger) {
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
      alias(self.cgraph).unblock_type_or_pack_type_id(ty);
    }
  }

  pub(crate) fn unblock_type_pack_id_location(&mut self, tp: TypePackId, _location: Location) {
    if let Some(logger) = alias_opt_mut(self.logger) {
      logger.pop_block_type_pack_id(tp);
    }

    if fflag::LuauConstraintGraph.get() {
      alias(self.cgraph).unblock_type_or_pack_type_pack_id(tp);
    } else {
      self.deprecate_d_unblock_(BlockedConstraintId::V1(tp));
    }
  }
}
