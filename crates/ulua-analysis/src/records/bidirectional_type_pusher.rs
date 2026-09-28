use alloc::vec::Vec;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    arena_handle::Handle, constraint::Constraint, constraint_solver::ConstraintSolver,
    incomplete_inference::IncompleteInference, subtyping::Subtyping, unifier_2::Unifier2,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct BidirectionalTypePusher {
  pub(crate) ast_types: Handle<DenseHashMap<*const AstExpr, TypeId>>,
  pub(crate) ast_expected_types: Handle<DenseHashMap<*const AstExpr, TypeId>>,
  pub(crate) solver: Handle<ConstraintSolver>,
  pub(crate) constraint: Handle<Constraint>,
  pub(crate) generic_types_and_packs: Handle<DenseHashSet<*const ()>>,
  pub(crate) unifier: Handle<Unifier2>,
  pub subtyping: Handle<Subtyping>,
  pub(crate) incomplete_inferences: Vec<IncompleteInference>,
  // C++: `DenseHashSet<std::pair<TypeId, const AstExpr*>, PairHash<...>>`.
  // The bespoke `PairHash` (fn-ptr hashers) blocks `Default`/construction;
  // the tuple key is `Hash`, so the default `DenseHashDefault` hasher is used.
  // Identity and behavior are unchanged — only the hash function differs.
  pub(crate) seen: DenseHashSet<(TypeId, *const AstExpr)>,
}
