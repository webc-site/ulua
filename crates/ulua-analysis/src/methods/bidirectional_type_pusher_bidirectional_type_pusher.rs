use alloc::vec::Vec;
use core::{
  ffi::c_void,
  ptr::{NonNull, null},
};

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    bidirectional_type_pusher::BidirectionalTypePusher, constraint::Constraint,
    constraint_solver::ConstraintSolver, subtyping::Subtyping, unifier_2::Unifier2,
  },
  type_aliases::type_id::TypeId,
};
impl BidirectionalTypePusher {
  /// `BidirectionalTypePusher::BidirectionalTypePusher(...)`
  /// (TableLiteralInference.cpp:96-113).
  pub fn new(
    ast_types: NonNull<DenseHashMap<*const AstExpr, TypeId>>,
    ast_expected_types: NonNull<DenseHashMap<*const AstExpr, TypeId>>,
    solver: NonNull<ConstraintSolver>,
    constraint: NonNull<Constraint>,
    generic_types_and_packs: NonNull<DenseHashSet<*const c_void>>,
    unifier: NonNull<Unifier2>,
    subtyping: NonNull<Subtyping>,
  ) -> Self {
    BidirectionalTypePusher {
      ast_types: ast_types.as_ptr(),
      ast_expected_types: ast_expected_types.as_ptr(),
      solver: solver.as_ptr(),
      constraint: constraint.as_ptr(),
      generic_types_and_packs: generic_types_and_packs.as_ptr(),
      unifier: unifier.as_ptr(),
      subtyping: subtyping.as_ptr(),
      incomplete_inferences: Vec::new(),
      // C++: `seen{{nullptr, nullptr}}` — empty-key sentinel is the null pair.
      seen: DenseHashSet::new((null(), null())),
    }
  }
}
