use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    arena_handle::Handle, bidirectional_type_pusher::BidirectionalTypePusher,
    constraint::Constraint, constraint_solver::ConstraintSolver, subtyping::Subtyping,
    unifier_2::Unifier2,
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
    generic_types_and_packs: NonNull<DenseHashSet<*const ()>>,
    unifier: NonNull<Unifier2>,
    subtyping: NonNull<Subtyping>,
  ) -> Self {
    BidirectionalTypePusher {
      ast_types: Handle::from_nonnull(ast_types),
      ast_expected_types: Handle::from_nonnull(ast_expected_types),
      solver: Handle::from_nonnull(solver),
      constraint: Handle::from_nonnull(constraint),
      generic_types_and_packs: Handle::from_nonnull(generic_types_and_packs),
      unifier: Handle::from_nonnull(unifier),
      subtyping: Handle::from_nonnull(subtyping),
      incomplete_inferences: Vec::new(),
      // C++: `seen{{nullptr, nullptr}}` — empty-key sentinel is the null pair.
      seen: DenseHashSet::default(),
    }
  }
}
