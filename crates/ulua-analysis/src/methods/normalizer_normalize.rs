use alloc::sync::Arc;
use core::ptr::{null, null_mut};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  methods::fresh_normalized_type::fresh_normalized_type,
  records::{
    fuel_initializer::FuelInitializer, normalized_type::NormalizedType, normalizer::Normalizer,
    normalizer_hit_limits::NormalizerHitLimits,
  },
  type_aliases::{seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId},
};
impl Normalizer {
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

    let mut norm = fresh_normalized_type(self.builtin_types);
    let mut seen_set_types: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());
    let mut seen_table_prop_pairs: SeenTablePropPairs = SeenTablePropPairs::new((null(), null()));

    // FuelInitializer handles initializing and tearing down normalization fuel limits.
    let mut fi = FuelInitializer {
      normalizer: self as *mut Normalizer,
      initialized_fuel: false,
    };
    unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };
    // 必须绑定命名变量：`let _ = fi` 会立即 drop，FuelInitializer 析构将刚初始化的
    // fuel 清空，导致整个 normalize 子树燃料计量失效（对齐 cpp `FuelInitializer fi{...}`）。
    let _fi = fi;

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
}
