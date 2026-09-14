use alloc::{collections::BTreeMap, sync::Arc};
use core::ptr::{null, null_mut};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  records::{
    fuel_initializer::FuelInitializer, normalized_extern_type::NormalizedExternType,
    normalized_function_type::NormalizedFunctionType, normalized_string_type::NormalizedStringType,
    normalized_type::NormalizedType, normalizer::Normalizer,
    normalizer_hit_limits::NormalizerHitLimits, type_ids::TypeIds,
  },
  type_aliases::{seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId},
};
impl Normalizer {
  pub fn normalize(&mut self, ty: TypeId) -> Arc<NormalizedType> {
    self
      .try_normalize(ty)
      .unwrap_or_else(|| Arc::new(self.empty_normalized_type()))
  }

  pub fn try_normalize(&mut self, ty: TypeId) -> Option<Arc<NormalizedType>> {
    match catch_unwind(AssertUnwindSafe(|| self.normalize_uncaught(ty))) {
      Ok(norm) => norm,
      Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => None,
      Err(payload) => resume_unwind(payload),
    }
  }

  fn normalize_uncaught(&mut self, ty: TypeId) -> Option<Arc<NormalizedType>> {
    if self.arena.is_null() {
      unsafe {
        (*(*self.shared_state).ice_handler).ice_string("Normalizing types outside a module");
      }
    }

    if let Some(shared) = self.cached_normals.get(&ty) {
      return Some(shared.clone());
    }

    let mut norm = self.empty_normalized_type();
    let mut seen_set_types: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());
    let mut seen_table_prop_pairs: SeenTablePropPairs = SeenTablePropPairs::new((null(), null()));

    // FuelInitializer handles initializing and tearing down normalization fuel limits.
    let mut fi = FuelInitializer {
      normalizer: self as *mut Normalizer,
      initialized_fuel: false,
    };
    unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };
    let _ = fi;

    let res = self.union_normal_with_ty(
      &mut norm,
      ty,
      &mut seen_table_prop_pairs,
      &mut seen_set_types,
      -1,
    );

    if res != NormalizationResult::True {
      return None;
    }

    if norm.is_unknown() {
      self.clear_normal(&mut norm);
      norm.tops = unsafe { (*self.builtin_types).unknown_type };
    }

    let shared = Arc::new(norm);

    if shared.is_cacheable {
      self.cached_normals.insert(ty, shared.clone());
    }

    Some(shared)
  }

  fn empty_normalized_type(&self) -> NormalizedType {
    let never_type = unsafe { (*self.builtin_types).never_type };

    NormalizedType {
      builtin_types: self.builtin_types,
      tops: never_type,
      booleans: never_type,
      extern_types: NormalizedExternType {
        extern_types: BTreeMap::new(),
        shape_extensions: TypeIds::new(),
        ordering: Vec::new(),
      },
      errors: never_type,
      nils: never_type,
      numbers: never_type,
      integers: never_type,
      strings: NormalizedStringType::NEVER,
      threads: never_type,
      buffers: never_type,
      tables: TypeIds::new(),
      functions: NormalizedFunctionType {
        is_top: false,
        parts: TypeIds::new(),
      },
      tyvars: BTreeMap::new(),
      is_cacheable: true,
    }
  }
}
