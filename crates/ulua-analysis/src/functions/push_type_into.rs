use core::{ffi::c_void, ptr::NonNull};

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    bidirectional_type_pusher::BidirectionalTypePusher, constraint::Constraint,
    constraint_solver::ConstraintSolver, push_type_result::PushTypeResult, subtyping::Subtyping,
    unifier_2::Unifier2,
  },
  type_aliases::type_id::TypeId,
};

pub fn push_type_into(
  ast_types: NonNull<DenseHashMap<*const AstExpr, TypeId>>,
  ast_expected_types: NonNull<DenseHashMap<*const AstExpr, TypeId>>,
  solver: NonNull<ConstraintSolver>,
  constraint: NonNull<Constraint>,
  generic_types_and_packs: NonNull<DenseHashSet<*const c_void>>,
  unifier: NonNull<Unifier2>,
  subtyping: NonNull<Subtyping>,
  expected_type: TypeId,
  expr: *const AstExpr,
) -> PushTypeResult {
  let mut btp = BidirectionalTypePusher::new(
    ast_types,
    ast_expected_types,
    solver,
    constraint,
    generic_types_and_packs,
    unifier,
    subtyping,
  );

  btp.push_type(expected_type, expr);

  PushTypeResult {
    incomplete_types: btp.incomplete_inferences,
  }
}
