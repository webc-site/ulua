use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::{fflag, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{follow_type, get_type},
  records::{
    fuel_initializer::FuelInitializer, intersection_type::IntersectionType,
    metatable_type::MetatableType, never_type::NeverType, normalized_type::NormalizedType,
    normalizer::Normalizer, normalizer_hit_limits::NormalizerHitLimits,
    recursion_counter::RecursionCounter, table_type::TableType, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn is_inhabited_normalized_type(&mut self, norm: &NormalizedType) -> NormalizationResult {
    match catch_unwind(AssertUnwindSafe(|| {
      let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();

      let mut fi = FuelInitializer {
        normalizer: self as *mut Normalizer,
        initialized_fuel: false,
      };
      // Safety: 实参 `self as *mut Normalizer` 来自当前 &mut 借用本身——非空、对齐，
      // 且调用帧（含闭包）比 fi 及其 Drop 中的 clear_fuel 长寿，满足该方法
      // # Safety 契约（直译 C++ `FuelInitializer fi{NotNull{this}}`）。
      unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };
      let _fi = fi;

      self.is_inhabited_normalized_type_set_type_id(norm, &mut seen)
    })) {
      Ok(result) => result,
      Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => {
        NormalizationResult::HitLimits
      }
      Err(payload) => resume_unwind(payload),
    }
  }

  pub fn is_inhabited_normalized_type_set_type_id(
    &mut self,
    norm: &NormalizedType,
    seen: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    // C++ Normalize.cpp:481-486：递归计数 + 资源限制检查 + 燃料消耗。
    // 契约：shared_state 为构造/接线期注入（C++ `NotNull<UnifierSharedState>`；
    // TypeChecker 在对象定址 Box 后即接线），shared_state_mut 断言接线非空。
    // RAII 计数器持有的 &mut 指向全程存活的共享状态字段，单线程串行无别名。
    let _rc = RecursionCounter::recursion_counter_i32(
      &mut self.shared_state_mut().counters.recursion_count,
    );
    if !self.within_resource_limits() {
      return NormalizationResult::HitLimits;
    }
    self.consume_fuel();

    if fflag::LuauIntegerType2.get() {
      if get_type::get::<NeverType>(norm.tops).is_none()
        || get_type::get::<NeverType>(norm.booleans).is_none()
        || get_type::get::<NeverType>(norm.errors).is_none()
        || get_type::get::<NeverType>(norm.nils).is_none()
        || get_type::get::<NeverType>(norm.numbers).is_none()
        || get_type::get::<NeverType>(norm.threads).is_none()
        || get_type::get::<NeverType>(norm.buffers).is_none()
        || !norm.extern_types.is_never()
        || get_type::get::<NeverType>(norm.integers).is_none()
        || !norm.strings.is_never()
        || !norm.functions.is_never()
      {
        return NormalizationResult::True;
      }
    } else {
      if get_type::get::<NeverType>(norm.tops).is_none()
        || get_type::get::<NeverType>(norm.booleans).is_none()
        || get_type::get::<NeverType>(norm.errors).is_none()
        || get_type::get::<NeverType>(norm.nils).is_none()
        || get_type::get::<NeverType>(norm.numbers).is_none()
        || get_type::get::<NeverType>(norm.threads).is_none()
        || get_type::get::<NeverType>(norm.buffers).is_none()
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

  pub fn is_inhabited_type_id(&mut self, ty: TypeId) -> NormalizationResult {
    if self.cache_inhabitance
      && let Some(result) = self.cached_is_inhabited.find(&ty)
    {
      return if *result {
        NormalizationResult::True
      } else {
        NormalizationResult::False
      };
    }

    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();

    match catch_unwind(AssertUnwindSafe(|| {
      let mut fi = FuelInitializer {
        normalizer: self as *mut Normalizer,
        initialized_fuel: false,
      };
      // Safety: 同 is_inhabited_normalized_type——`self as *mut Normalizer` 即当前
      // &mut 借用自身，非空、对齐，闭包帧比 fi 的 Drop（clear_fuel）长寿，
      // 满足 fuel_initializer_not_null_normalizer 的存活契约。
      unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };
      let _fi = fi;

      let result = self.is_inhabited_type_id_set_type_id(ty, &mut seen);

      if self.cache_inhabitance {
        if result == NormalizationResult::True {
          *self.cached_is_inhabited.get_or_insert(ty) = true;
        } else if result == NormalizationResult::False {
          *self.cached_is_inhabited.get_or_insert(ty) = false;
        }
      }

      result
    })) {
      Ok(result) => result,
      Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => {
        NormalizationResult::HitLimits
      }
      Err(payload) => resume_unwind(payload),
    }
  }

  pub fn is_inhabited_type_id_set_type_id(
    &mut self,
    ty: TypeId,
    seen: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    // 契约：shared_state 为构造/接线期注入（C++ `NotNull<UnifierSharedState>`；
    // TypeChecker 在对象定址 Box 后即接线），shared_state_mut 断言接线非空。
    // RAII 计数器持有的 &mut 指向全程存活的共享状态字段，单线程串行无别名。
    let _rc = RecursionCounter::recursion_counter_i32(
      &mut self.shared_state_mut().counters.recursion_count,
    );

    if !self.within_resource_limits() {
      return NormalizationResult::HitLimits;
    }

    self.consume_fuel();

    let ty = follow_type::follow(ty);

    if get_type::get::<NeverType>(ty).is_some() {
      return NormalizationResult::False;
    }

    if get_type::get::<IntersectionType>(ty).is_none()
      && get_type::get::<UnionType>(ty).is_none()
      && get_type::get::<TableType>(ty).is_none()
      && get_type::get::<MetatableType>(ty).is_none()
    {
      return NormalizationResult::True;
    }

    if seen.contains(&ty) {
      return NormalizationResult::True;
    }

    seen.insert(ty);

    if let Some(ttv) = get_type::get::<TableType>(ty) {
      for prop in ttv.props.values() {
        if self.use_new_luau_solver() {
          if let Some(ty) = prop.read_ty {
            let res = self.is_inhabited_type_id_set_type_id(ty, seen);
            if res != NormalizationResult::True {
              return res;
            }
          }
        } else {
          // 对齐 cpp `prop.type_DEPRECATED()`：内部 LUAU_ASSERT(readTy)+解引用。
          let res = self.is_inhabited_type_id_set_type_id(prop.type_deprecated(), seen);
          if res != NormalizationResult::True {
            return res;
          }
        }
      }
      return NormalizationResult::True;
    }

    if let Some(mtv) = get_type::get::<MetatableType>(ty) {
      let res = self.is_inhabited_type_id_set_type_id(mtv.table, seen);
      if res != NormalizationResult::True {
        return res;
      }
      return self.is_inhabited_type_id_set_type_id(mtv.metatable, seen);
    }

    // C++ `normalize` 失败得 nullptr，随后 `isInhabited(nullptr, seen)` 顶部
    // `!norm` 检查返回 HitLimits；若回退空 norm 会被误判为 False（uninhabited）。
    let Some(norm) = self.try_normalize(ty) else {
      return NormalizationResult::HitLimits;
    };
    self.is_inhabited_normalized_type_set_type_id(norm.as_ref(), seen)
  }
}
