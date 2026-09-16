use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer, subtyping_result::SubtypingResult, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
    type_pair_hash::TypePairHash,
  },
  type_aliases::{
    seen_set_subtyping::SeenSet, seen_type_pack_set::SeenTypePackSet, type_id::TypeId,
  },
};

#[derive(Debug, Clone)]
pub struct Subtyping {
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) arena: *mut TypeArena,
  pub(crate) normalizer: *mut Normalizer,
  pub(crate) type_function_runtime: *mut TypeFunctionRuntime,
  pub(crate) ice_reporter: *mut InternalErrorReporter,
  pub(crate) limits: TypeCheckLimits,
  pub(crate) unique_types: *const DenseHashSet<TypeId>,
  pub(crate) seen_types: SeenSet,
  pub(crate) seen_packs: SeenTypePackSet,
  pub(crate) result_cache: DenseHashMap<(TypeId, TypeId), SubtypingResult, TypePairHash>,
}
