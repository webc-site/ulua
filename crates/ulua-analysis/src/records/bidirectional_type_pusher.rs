use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    incomplete_inference::IncompleteInference, subtyping::Subtyping, unifier_2::Unifier2,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct BidirectionalTypePusher {
  pub(crate) ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) ast_expected_types: *mut DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) solver: *mut ConstraintSolver,
  pub(crate) constraint: *const Constraint,
  pub(crate) generic_types_and_packs: *mut DenseHashSet<*const c_void>,
  pub(crate) unifier: *mut Unifier2,
  pub subtyping: *mut Subtyping,
  pub(crate) incomplete_inferences: Vec<IncompleteInference>,
  // C++: `DenseHashSet<std::pair<TypeId, const AstExpr*>, PairHash<...>>`.
  // The bespoke `PairHash` (fn-ptr hashers) blocks `Default`/construction;
  // the tuple key is `Hash`, so the default `DenseHashDefault` hasher is used.
  // Identity and behavior are unchanged — only the hash function differs.
  pub(crate) seen: DenseHashSet<(TypeId, *const AstExpr)>,
}
