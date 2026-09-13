//! Source: `Analysis/src/Normalize.cpp:1796-1967` (hand-ported)
use alloc::{boxed::Box, collections::BTreeMap};

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

/// C++ `Set<TypeId>::erase(key)`. The Rust skeleton maps the seen-set parameter
/// to `DenseHashSet`, which (faithful to `Luau::DenseHashSet`) cannot erase a
/// single slot, whereas C++ here uses `Luau::Set` which can. We reproduce the
/// single-element removal by rebuilding the set without `key`; `clear` preserves
/// the empty-key sentinel, so the rebuilt set stays valid.
use crate::methods::normalized_string_type_reset_to_string::normalized_string_type_reset_to_string;
use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    assert_invariant::assert_invariant, follow_type::follow_type_id,
    get_singleton_type::get_singleton_type, get_type_alt_j::get_type_id,
    is_cacheable_normalize_alt_c::is_cacheable_type_id, tyvar_index::tyvar_index,
  },
  records::{
    any_type::AnyType,
    blocked_type::BlockedType,
    boolean_singleton::BooleanSingleton,
    builtin_types::BuiltinTypes,
    extern_type::ExternType,
    free_type::FreeType,
    function_type::FunctionType,
    generic_type::GenericType,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    no_refine_type::NoRefineType,
    normalized_extern_type::NormalizedExternType,
    normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType,
    normalized_type::NormalizedType,
    normalizer::Normalizer,
    pending_expansion_type::PendingExpansionType,
    primitive_type::{PrimitiveType, Type as PrimType},
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_ids::TypeIds,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{
    error_type::ErrorType, seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId,
  },
};
pub(crate) fn erase_seen(seen: &mut DenseHashSet<TypeId>, key: TypeId) {
  let kept: Vec<TypeId> = seen.iter().copied().filter(|&k| k != key).collect();
  seen.clear();
  for k in kept {
    seen.insert(k);
  }
}

/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`:
/// increments the shared recursion counter on construction, decrements on drop.
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

fn fresh_normalized_type(builtin_types: *mut BuiltinTypes) -> NormalizedType {
  // SAFETY: builtin_types 指向调用方保证有效的 BuiltinTypes（C++ sharedState 生命周期）
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

impl Normalizer {
  // See above for an explanation of `ignoreSmallerTyvars`.
  pub fn union_normal_with_ty(
    &mut self,
    here: &mut NormalizedType,
    there: TypeId,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set_types: &mut DenseHashSet<TypeId>,
    ignore_smaller_tyvars: i32,
  ) -> NormalizationResult {
    let _rc = RcGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });
    if !self.within_resource_limits() {
      return NormalizationResult::HitLimits;
    }

    self.consume_fuel();

    let there = follow_type_id(there);

    if get_type_id::<AnyType>(there).is_some() || get_type_id::<UnknownType>(there).is_some() {
      let mut tops = self.union_of_tops(here.tops, there);
      if get_type_id::<UnknownType>(tops).is_some()
        && get_type_id::<ErrorType>(here.errors).is_some()
      {
        // SAFETY: self.builtin_types 指向常驻的 BuiltinTypes
        tops = unsafe { (*self.builtin_types).any_type };
      }
      self.clear_normal(here);
      here.tops = tops;
      return NormalizationResult::True;
    } else if get_type_id::<NeverType>(there).is_some()
      || get_type_id::<AnyType>(here.tops).is_some()
    {
      return NormalizationResult::True;
    } else if get_type_id::<ErrorType>(there).is_some()
      && get_type_id::<UnknownType>(here.tops).is_some()
    {
      // SAFETY: self.builtin_types 指向常驻的 BuiltinTypes
      here.tops = unsafe { (*self.builtin_types).any_type };
      return NormalizationResult::True;
    } else if let Some(utv) = get_type_id::<UnionType>(there) {
      if seen_set_types.contains(&there) {
        return NormalizationResult::True;
      }
      seen_set_types.insert(there);

      // 克隆后再递归：递归中 visit 可能改写该 union 的 options
      let options = utv.options.clone();
      for opt in options {
        let res = self.union_normal_with_ty(here, opt, seen_table_prop_pairs, seen_set_types, -1);
        if res != NormalizationResult::True {
          erase_seen(seen_set_types, there);
          return res;
        }
      }

      erase_seen(seen_set_types, there);
      return NormalizationResult::True;
    } else if let Some(itv) = get_type_id::<IntersectionType>(there) {
      if seen_set_types.contains(&there) {
        return NormalizationResult::True;
      }
      seen_set_types.insert(there);

      let mut norm = fresh_normalized_type(self.builtin_types);
      // SAFETY: self.builtin_types 指向常驻的 BuiltinTypes
      norm.tops = unsafe { (*self.builtin_types).unknown_type };
      let parts = itv.parts.clone();
      for part in parts {
        let res =
          self.intersect_normal_with_ty(&mut norm, part, seen_table_prop_pairs, seen_set_types);
        if res != NormalizationResult::True {
          erase_seen(seen_set_types, there);
          return res;
        }
      }

      erase_seen(seen_set_types, there);

      return self.union_normals(here, &norm, -1);
    } else if get_type_id::<UnknownType>(here.tops).is_some() {
      return NormalizationResult::True;
    } else if get_type_id::<GenericType>(there).is_some()
      || get_type_id::<FreeType>(there).is_some()
      || get_type_id::<BlockedType>(there).is_some()
      || get_type_id::<PendingExpansionType>(there).is_some()
      || get_type_id::<TypeFunctionInstanceType>(there).is_some()
    {
      if tyvar_index(there) <= ignore_smaller_tyvars {
        return NormalizationResult::True;
      }
      let mut inter = fresh_normalized_type(self.builtin_types);
      // SAFETY: self.builtin_types 指向常驻的 BuiltinTypes
      inter.tops = unsafe { (*self.builtin_types).unknown_type };
      here.tyvars.insert(there, Box::new(inter));

      if !is_cacheable_type_id(there) {
        here.is_cacheable = false;
      }
    } else if get_type_id::<FunctionType>(there).is_some() {
      self.union_functions_with_function(&mut here.functions, there);
    } else if get_type_id::<TableType>(there).is_some()
      || get_type_id::<MetatableType>(there).is_some()
    {
      self.union_tables_with_table(&mut here.tables, there);
    } else if get_type_id::<ExternType>(there).is_some() {
      self.union_extern_types_with_extern_type_normalized_extern_type_type_id(
        &mut here.extern_types,
        there,
      );
    } else if get_type_id::<ErrorType>(there).is_some() {
      here.errors = there;
    } else if let Some(ptv) = get_type_id::<PrimitiveType>(there) {
      match ptv.r#type {
        PrimType::Boolean => here.booleans = there,
        PrimType::NilType => here.nils = there,
        PrimType::Number => here.numbers = there,
        PrimType::Integer if FFlag::LuauIntegerType2.get() => here.integers = there,
        PrimType::String => normalized_string_type_reset_to_string(&mut here.strings),
        PrimType::Thread => here.threads = there,
        PrimType::Buffer => here.buffers = there,
        PrimType::Function => here.functions.reset_to_top(),
        PrimType::Table => {
          here.tables.clear();
          here.tables.insert_type_id(there);
        }
        _ => LUAU_ASSERT!(false),
      }
    } else if let Some(stv) = get_type_id::<SingletonType>(there) {
      if get_singleton_type::<BooleanSingleton>(stv).is_some() {
        here.booleans = self.union_of_bools(here.booleans, there);
      } else if let Some(sstv) = get_singleton_type::<StringSingleton>(stv) {
        if here.strings.is_cofinite {
          if here.strings.singletons.contains_key(&sstv.value) {
            here.strings.singletons.remove(&sstv.value);
          }
        } else {
          here.strings.singletons.insert(sstv.value.clone(), there);
        }
      } else {
        LUAU_ASSERT!(false);
      }
    } else if let Some(ntv) = get_type_id::<NegationType>(there) {
      let ntv_ty = ntv.ty;

      let there_normal = self.normalize(ntv_ty);
      let tn = self.negate_normal(&there_normal);

      let tn = match tn {
        Some(t) => t,
        None => return NormalizationResult::False,
      };

      let res = self.union_normals(here, &tn, -1);
      if res != NormalizationResult::True {
        return res;
      }
    } else if get_type_id::<PendingExpansionType>(there).is_some()
      || get_type_id::<TypeFunctionInstanceType>(there).is_some()
      || get_type_id::<NoRefineType>(there).is_some()
    {
      // nothing
    } else {
      LUAU_ASSERT!(false);
    }

    let tyvar_keys: Vec<TypeId> = here.tyvars.keys().copied().collect();
    for tyvar in tyvar_keys {
      if let Some(mut intersect) = here.tyvars.remove(&tyvar) {
        let res = self.union_normal_with_ty(
          &mut intersect,
          there,
          seen_table_prop_pairs,
          seen_set_types,
          tyvar_index(tyvar),
        );
        here.tyvars.insert(tyvar, intersect);
        if res != NormalizationResult::True {
          return res;
        }
      }
    }

    assert_invariant(here);
    NormalizationResult::True
  }
}
