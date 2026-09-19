use ulua_common::{fflag, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, normalized_type::NormalizedType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};

/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`.
struct RecursionCountGuard {
  count: *mut i32,
}

impl RecursionCountGuard {
  fn new(count: *mut i32) -> Self {
    // SAFETY: count 指向 shared_state 内的计数器，存活期覆盖 guard。
    unsafe {
      *count += 1;
    }
    Self { count }
  }
}

impl Drop for RecursionCountGuard {
  fn drop(&mut self) {
    // SAFETY: 同上。
    unsafe {
      *self.count -= 1;
    }
  }
}

impl Normalizer {
  pub fn is_inhabited_normalized_type_set_type_id(
    &mut self,
    norm: &NormalizedType,
    seen: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    // C++ Normalize.cpp:481-486：递归计数 + 资源限制检查 + 燃料消耗。
    let _rc =
      RecursionCountGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });
    if !self.within_resource_limits() {
      return NormalizationResult::HitLimits;
    }
    self.consume_fuel();

    if fflag::LuauIntegerType2.get() {
      if get_type_id::<NeverType>(norm.tops).is_none()
        || get_type_id::<NeverType>(norm.booleans).is_none()
        || get_type_id::<NeverType>(norm.errors).is_none()
        || get_type_id::<NeverType>(norm.nils).is_none()
        || get_type_id::<NeverType>(norm.numbers).is_none()
        || get_type_id::<NeverType>(norm.threads).is_none()
        || get_type_id::<NeverType>(norm.buffers).is_none()
        || !norm.extern_types.is_never()
        || get_type_id::<NeverType>(norm.integers).is_none()
        || !norm.strings.is_never()
        || !norm.functions.is_never()
      {
        return NormalizationResult::True;
      }
    } else {
      if get_type_id::<NeverType>(norm.tops).is_none()
        || get_type_id::<NeverType>(norm.booleans).is_none()
        || get_type_id::<NeverType>(norm.errors).is_none()
        || get_type_id::<NeverType>(norm.nils).is_none()
        || get_type_id::<NeverType>(norm.numbers).is_none()
        || get_type_id::<NeverType>(norm.threads).is_none()
        || get_type_id::<NeverType>(norm.buffers).is_none()
        || !norm.extern_types.is_never()
        || !norm.strings.is_never()
        || !norm.functions.is_never()
      {
        return NormalizationResult::True;
      }
    }

    for intersect in norm.tyvars.values() {
      let res = self.is_inhabited_normalized_type_set_type_id(intersect, seen);
      if res != NormalizationResult::False {
        return res;
      }
    }

    for &table in &norm.tables.order {
      let res = self.is_inhabited_type_id_set_type_id(table, seen);
      if res != NormalizationResult::False {
        return res;
      }
    }

    NormalizationResult::False
  }
}
