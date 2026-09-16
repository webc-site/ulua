use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    mapped_generic_environment::MappedGenericEnvironment, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
  },
  type_aliases::type_id::TypeId,
};
impl Subtyping {
  pub fn is_subtype_type_id_type_id_not_null_scope(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut env = SubtypingEnvironment {
      parent: null_mut(),
      mapped_generics: DenseHashMap::new(null()),
      mapped_generic_packs: MappedGenericEnvironment {
        frames: Vec::new(),
        current_scope_index: None,
      },
      substitutions: DenseHashMap::new(null()),
      seen_set_cache: DenseHashMap::new((null(), null())),
      iteration_count: 0,
    };

    let result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
      &mut env, sub_ty, super_ty, scope,
    );

    if result.normalization_too_complex {
      if result.is_cacheable {
        self
          .result_cache
          .try_insert((sub_ty, super_ty), result.clone());
      }
      return result;
    }

    if result.is_cacheable {
      self
        .result_cache
        .try_insert((sub_ty, super_ty), result.clone());
    }

    result
  }
}
