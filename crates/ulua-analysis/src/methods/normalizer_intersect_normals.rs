//! Source: `Analysis/src/Normalize.cpp:3244-3317` (hand-ported)
use alloc::boxed::Box;
/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`.
use alloc::collections::BTreeMap;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    get_type_alt_j::get_type_id, is_shallow_inhabited::is_shallow_inhabited,
    tyvar_index::tyvar_index,
  },
  records::{
    builtin_types::BuiltinTypes, never_type::NeverType,
    normalized_extern_type::NormalizedExternType, normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType, normalized_type::NormalizedType,
    normalizer::Normalizer, type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};
struct RcGuard {
  count: *mut i32,
}

impl RcGuard {
  fn new(count: *mut i32) -> Self {
    unsafe {
      *count += 1;
    }
    RcGuard { count }
  }
}

impl Drop for RcGuard {
  fn drop(&mut self) {
    unsafe {
      *self.count -= 1;
    }
  }
}

impl Normalizer {
  // See above for an explanation of `ignoreSmallerTyvars`.
  pub fn intersect_normals(
    &mut self,
    here: &mut NormalizedType,
    there: &NormalizedType,
    ignore_smaller_tyvars: i32,
  ) -> NormalizationResult {
    let _rc = RcGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });
    if !self.within_resource_limits() {
      return NormalizationResult::HitLimits;
    }

    self.consume_fuel();

    if get_type_id::<NeverType>(there.tops).is_none() {
      here.tops = self.intersection_of_tops(here.tops, there.tops);
      return NormalizationResult::True;
    } else if get_type_id::<NeverType>(here.tops).is_none() {
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
    here.errors = if !get_type_id::<NeverType>(there.errors).is_none() {
      there.errors
    } else {
      here.errors
    };
    here.nils = if !get_type_id::<NeverType>(there.nils).is_none() {
      there.nils
    } else {
      here.nils
    };
    here.numbers = if !get_type_id::<NeverType>(there.numbers).is_none() {
      there.numbers
    } else {
      here.numbers
    };
    if FFlag::LuauIntegerType2.get() {
      here.integers = if !get_type_id::<NeverType>(there.integers).is_none() {
        there.integers
      } else {
        here.integers
      };
    }
    self.intersect_strings(&mut here.strings, &there.strings);
    here.threads = if !get_type_id::<NeverType>(there.threads).is_none() {
      there.threads
    } else {
      here.threads
    };
    here.buffers = if !get_type_id::<NeverType>(there.buffers).is_none() {
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

fn fresh_normalized_type(builtin_types: *mut BuiltinTypes) -> NormalizedType {
  let never_type = unsafe { (*builtin_types).never_type };
  NormalizedType {
    builtin_types,
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
