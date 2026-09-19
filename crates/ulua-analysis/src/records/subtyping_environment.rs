//! Source: `Analysis/include/Luau/Subtyping.h` (hand-ported; fields only)

use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    generic_bounds::GenericBounds, mapped_generic_environment::MappedGenericEnvironment,
    subtyping_result::SubtypingResult, type_pair_hash::TypePairHash,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug)]
pub struct SubtypingEnvironment {
  pub parent: *mut SubtypingEnvironment,
  pub mapped_generics: DenseHashMap<TypeId, Vec<GenericBounds>>,
  pub mapped_generic_packs: MappedGenericEnvironment,
  pub substitutions: DenseHashMap<TypeId, TypeId>,
  pub seen_set_cache: DenseHashMap<(TypeId, TypeId), SubtypingResult, TypePairHash>,
  pub iteration_count: i32,
}
