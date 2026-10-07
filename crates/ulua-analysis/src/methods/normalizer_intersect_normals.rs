//! Source: `Analysis/src/Normalize.cpp:3244-3317` (hand-ported)

use alloc::boxed::Box;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{get_type, is_shallow_inhabited::is_shallow_inhabited, tyvar_index::tyvar_index},
  methods::fresh_normalized_type::fresh_normalized_type,
  records::{
    never_type::NeverType, normalized_type::NormalizedType, normalizer::Normalizer,
    recursion_counter::RecursionCounter,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  // See above for an explanation of `ignoreSmallerTyvars`.
  pub fn intersect_normals(
    &mut self,
    here: &mut NormalizedType,
    there: &NormalizedType,
    ignore_smaller_tyvars: i32,
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

    if get_type::get::<NeverType>(there.tops).is_none() {
      here.tops = self.intersection_of_tops(here.tops, there.tops);
      return NormalizationResult::True;
    } else if get_type::get::<NeverType>(here.tops).is_none() {
      self.clear_normal(here);
      return self.union_normals(here, there, ignore_smaller_tyvars);
    }

    for tyvar in there.tyvars.keys() {
      let index = tyvar_index(*tyvar);
      if ignore_smaller_tyvars < index {
        let fresh = !here.tyvars.contains_key(tyvar);
        if fresh {
          let mut entry = Box::new(fresh_normalized_type(self.builtin_types));
          let res = self.union_normals(&mut entry, here, index);
          if res != NormalizationResult::True {
            return res;
          }
          here.tyvars.insert(*tyvar, entry);
        }
      }
    }

    here.booleans = self.intersection_of_bools(here.booleans, there.booleans);

    self.intersect_extern_types(&mut here.extern_types, &there.extern_types);
    here.errors = if get_type::get::<NeverType>(there.errors).is_some() {
      there.errors
    } else {
      here.errors
    };
    here.nils = if get_type::get::<NeverType>(there.nils).is_some() {
      there.nils
    } else {
      here.nils
    };
    here.numbers = if get_type::get::<NeverType>(there.numbers).is_some() {
      there.numbers
    } else {
      here.numbers
    };
    if fflag::LuauIntegerType2.get() {
      here.integers = if get_type::get::<NeverType>(there.integers).is_some() {
        there.integers
      } else {
        here.integers
      };
    }
    self.intersect_strings(&mut here.strings, &there.strings);
    here.threads = if get_type::get::<NeverType>(there.threads).is_some() {
      there.threads
    } else {
      here.threads
    };
    here.buffers = if get_type::get::<NeverType>(there.buffers).is_some() {
      there.buffers
    } else {
      here.buffers
    };
    self.intersect_functions(&mut here.functions, &there.functions);
    self.intersect_tables(&mut here.tables, &there.tables);

    let tyvar_keys: Vec<TypeId> = here.tyvars.keys().copied().collect();
    for tyvar in tyvar_keys {
      let mut inter = match here.tyvars.remove(&tyvar) {
        Some(b) => b,
        None => continue,
      };
      let index = tyvar_index(tyvar);
      LUAU_ASSERT!(ignore_smaller_tyvars < index);
      let res = match there.tyvars.get(&tyvar) {
        None => self.intersect_normals(&mut inter, there, index),
        Some(found) => self.intersect_normals(&mut inter, found, index),
      };
      if res != NormalizationResult::True {
        here.tyvars.insert(tyvar, inter);
        return res;
      }
      if is_shallow_inhabited(&inter) {
        here.tyvars.insert(tyvar, inter);
      }
      // else: drop `inter`, removing the entry (C++ `here.tyvars.erase(it)`).
    }
    NormalizationResult::True
  }
}
