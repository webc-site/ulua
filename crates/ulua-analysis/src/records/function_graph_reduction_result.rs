use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct FunctionGraphReductionResult {
  pub errors: ErrorVec,
  pub messages: ErrorVec,
  pub blocked_types: DenseHashSet<TypeId>,
  pub blocked_packs: DenseHashSet<TypePackId>,
  pub reduced_types: DenseHashSet<TypeId>,
  pub reduced_packs: DenseHashSet<TypePackId>,
  pub irreducible_types: DenseHashSet<TypeId>,
}
