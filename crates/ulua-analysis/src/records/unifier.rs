//! Source: `Analysis/include/Luau/Unifier.h` (hand-ported; fields only)

use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  enums::variance::Variance,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, count_mismatch::CountMismatchContext,
    normalizer::Normalizer, scope::Scope, txn_log::TxnLog, type_arena::TypeArena,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug)]
pub struct Unifier {
  pub types: Handle<TypeArena>,
  pub builtin_types: Handle<BuiltinTypes>,
  pub normalizer: Handle<Normalizer>,
  pub scope: Handle<Scope>,
  pub log: TxnLog,
  pub failure: bool,
  pub errors: ErrorVec,
  pub location: Location,
  pub variance: Variance,
  pub normalize: bool,
  pub check_inhabited: bool,
  pub ctx: CountMismatchContext,
  pub shared_state: Handle<UnifierSharedState>, // UnifierSharedState&
  pub blocked_types: Vec<TypeId>,
  pub blocked_type_packs: Vec<TypePackId>,
  pub first_pack_error_pos: Option<i32>,
}
