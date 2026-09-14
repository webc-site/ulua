//! Source: `Analysis/src/Normalize.cpp:3319-3564` (hand-ported)
use alloc::{boxed::Box, collections::BTreeMap};
use core::mem;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`.
use crate::records::builtin_types::BuiltinTypes;
use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    follow_type::follow_type_id, get_singleton_type::get_singleton_type,
    get_type_alt_j::get_type_id, should_early_exit::should_early_exit,
  },
  records::{
    any_type::AnyType,
    blocked_type::BlockedType,
    boolean_singleton::BooleanSingleton,
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
    error_type::ErrorType, normalized_tyvars::NormalizedTyvars,
    seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId,
  },
};
struct RcGuard {
  count: *mut i32,
}

impl RcGuard {
  fn new(count: *mut i32) -> Self {
    // SAFETY: count 指向 shared_state 内的计数器，存活期覆盖 guard。
    unsafe {
      *count += 1;
    }
    RcGuard { count }
  }
}

impl Drop for RcGuard {
  fn drop(&mut self) {
    // SAFETY: 同上。
    unsafe {
      *self.count -= 1;
    }
  }
}

fn fresh_normalized_type(builtin_types: *mut BuiltinTypes) -> NormalizedType {
  // SAFETY: builtin_types 指向全局 BuiltinTypes。
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
  pub fn intersect_normal_with_ty(
    &mut self,
    here: &mut NormalizedType,
    there: TypeId,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set_types: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    // SAFETY: shared_state 在 Normalizer 存活期内有效。
    let _rc = RcGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });
    if !self.within_resource_limits() {
      return NormalizationResult::HitLimits;
    }

    self.consume_fuel();

    let there = follow_type_id(there);

    if get_type_id::<AnyType>(there).is_some() || get_type_id::<UnknownType>(there).is_some() {
      here.tops = self.intersection_of_tops(here.tops, there);
      return NormalizationResult::True;
    } else if get_type_id::<NeverType>(here.tops).is_none() {
      self.clear_normal(here);
      return self.union_normal_with_ty(here, there, seen_table_prop_pairs, seen_set_types, -1);
    } else if let Some(utv) = get_type_id::<UnionType>(there) {
      let mut norm = fresh_normalized_type(self.builtin_types);
      let options = utv.options.clone();
      for opt in options {
        let res =
          self.union_normal_with_ty(&mut norm, opt, seen_table_prop_pairs, seen_set_types, -1);
        if res != NormalizationResult::True {
          return res;
        }
      }
      return self.intersect_normals(here, &norm, -1);
    } else if let Some(itv) = get_type_id::<IntersectionType>(there) {
      let parts = itv.parts.clone();
      for part in parts {
        let res = self.intersect_normal_with_ty(here, part, seen_table_prop_pairs, seen_set_types);
        if res != NormalizationResult::True {
          return res;
        }
      }
      return NormalizationResult::True;
    } else if get_type_id::<GenericType>(there).is_some()
      || get_type_id::<FreeType>(there).is_some()
      || get_type_id::<BlockedType>(there).is_some()
      || get_type_id::<PendingExpansionType>(there).is_some()
      || get_type_id::<TypeFunctionInstanceType>(there).is_some()
    {
      let mut there_norm = fresh_normalized_type(self.builtin_types);
      let mut top_norm = fresh_normalized_type(self.builtin_types);
      // SAFETY: builtin_types 指向全局 BuiltinTypes。
      top_norm.tops = unsafe { (*self.builtin_types).unknown_type };
      there_norm.tyvars.insert(there, Box::new(top_norm));
      here.is_cacheable = false;
      return self.intersect_normals(here, &there_norm, -1);
    }

    let mut tyvars: NormalizedTyvars = mem::take(&mut here.tyvars);

    if get_type_id::<FunctionType>(there).is_some() {
      let mut functions = mem::replace(
        &mut here.functions,
        NormalizedFunctionType {
          is_top: false,
          parts: TypeIds::new(),
        },
      );
      self.clear_normal(here);
      self.intersect_functions_with_function(&mut functions, there);
      here.functions = functions;
    } else if get_type_id::<TableType>(there).is_some()
      || get_type_id::<MetatableType>(there).is_some()
    {
      if self.use_new_luau_solver() {
        let mut extern_types = mem::replace(
          &mut here.extern_types,
          NormalizedExternType {
            extern_types: BTreeMap::new(),
            shape_extensions: TypeIds::new(),
            ordering: Vec::new(),
          },
        );
        let mut tables = mem::take(&mut here.tables);
        self.clear_normal(here);

        if FFlag::LuauExternTypesNormalizeWithShapes.get() {
          if extern_types.is_never() {
            self.intersect_tables_with_table(
              &mut tables,
              there,
              seen_table_prop_pairs,
              seen_set_types,
            );
          } else {
            self.intersect_extern_types_with_shape(&mut extern_types, there);
          }
        } else {
          self.intersect_tables_with_table(
            &mut tables,
            there,
            seen_table_prop_pairs,
            seen_set_types,
          );
        }

        here.tables = tables;
        here.extern_types = extern_types;
      } else {
        let mut tables = mem::take(&mut here.tables);
        self.clear_normal(here);
        self.intersect_tables_with_table(&mut tables, there, seen_table_prop_pairs, seen_set_types);
        here.tables = tables;
      }
    } else if get_type_id::<ExternType>(there).is_some() {
      let mut nct = mem::replace(
        &mut here.extern_types,
        NormalizedExternType {
          extern_types: BTreeMap::new(),
          shape_extensions: TypeIds::new(),
          ordering: Vec::new(),
        },
      );
      self.clear_normal(here);
      self.intersect_extern_types_with_extern_type(&mut nct, there);
      here.extern_types = nct;
    } else if get_type_id::<ErrorType>(there).is_some() {
      let errors = here.errors;
      self.clear_normal(here);
      here.errors = if get_type_id::<ErrorType>(errors).is_some() {
        errors
      } else {
        there
      };
    } else if let Some(ptv) = get_type_id::<PrimitiveType>(there) {
      let booleans = here.booleans;
      let nils = here.nils;
      let numbers = here.numbers;
      let integers = here.integers;
      let strings = mem::replace(&mut here.strings, NormalizedStringType::NEVER);
      let functions = mem::replace(
        &mut here.functions,
        NormalizedFunctionType {
          is_top: false,
          parts: TypeIds::new(),
        },
      );
      let threads = here.threads;
      let buffers = here.buffers;
      let tables = mem::take(&mut here.tables);

      self.clear_normal(here);

      match ptv.r#type {
        PrimType::Boolean => here.booleans = booleans,
        PrimType::NilType => here.nils = nils,
        PrimType::Number => here.numbers = numbers,
        PrimType::Integer if FFlag::LuauIntegerType2.get() => here.integers = integers,
        PrimType::String => here.strings = strings,
        PrimType::Thread => here.threads = threads,
        PrimType::Buffer => here.buffers = buffers,
        PrimType::Function => here.functions = functions,
        PrimType::Table => here.tables = tables,
        _ => LUAU_ASSERT!(false),
      }
    } else if let Some(stv) = get_type_id::<SingletonType>(there) {
      let booleans = here.booleans;
      let strings = mem::replace(&mut here.strings, NormalizedStringType::NEVER);

      self.clear_normal(here);

      if get_singleton_type::<BooleanSingleton>(stv).is_some() {
        here.booleans = self.intersection_of_bools(booleans, there);
      } else if let Some(sstv) = get_singleton_type::<StringSingleton>(stv) {
        if strings.includes(&sstv.value) {
          here.strings.singletons.insert(sstv.value.clone(), there);
        }
      } else {
        LUAU_ASSERT!(false);
      }
    } else if let Some(ntv) = get_type_id::<NegationType>(there) {
      let ntv_ty = ntv.ty;
      let t = follow_type_id(ntv_ty);
      if get_type_id::<PrimitiveType>(t).is_some() {
        self.subtract_primitive(here, ntv_ty);
      } else if get_type_id::<SingletonType>(t).is_some() {
        self.subtract_singleton(here, follow_type_id(ntv_ty));
      } else if get_type_id::<ExternType>(t).is_some() {
        let res = self.intersect_normal_with_negation_ty(t, here);
        if should_early_exit(res) {
          here.tyvars = tyvars;
          return res;
        }
      } else if let Some(utv) = get_type_id::<UnionType>(t) {
        let options = utv.options.clone();
        for part in options {
          let res = self.intersect_normal_with_negation_ty(part, here);
          if should_early_exit(res) {
            here.tyvars = tyvars;
            return res;
          }
        }
      } else if get_type_id::<AnyType>(t).is_some() {
        // HACK: Refinements sometimes intersect with ~any under the
        // assumption that it is the same as any.
        here.tyvars = tyvars;
        return NormalizationResult::True;
      } else if get_type_id::<NoRefineType>(t).is_some() {
        // `*no-refine*` means we will never do anything to affect the intersection.
        here.tyvars = tyvars;
        return NormalizationResult::True;
      } else if get_type_id::<NeverType>(t).is_some() {
        // intersecting with `~never` is equivalent to intersecting with `unknown` (a noop).
        here.tyvars = tyvars;
        return NormalizationResult::True;
      } else if get_type_id::<UnknownType>(t).is_some() {
        // intersecting with `~unknown` is equivalent to intersecting with `never`.
        self.clear_normal(here);
        here.tyvars = tyvars;
        return NormalizationResult::True;
      } else if get_type_id::<ErrorType>(t).is_some() {
        // ~error is still an error.
        let errors = here.errors;
        self.clear_normal(here);
        here.errors = if get_type_id::<ErrorType>(errors).is_some() {
          errors
        } else {
          t
        };
      } else if let Some(ntv_t) = get_type_id::<NegationType>(t) {
        let nt_ty = ntv_t.ty;
        here.tyvars = tyvars;
        return self.intersect_normal_with_ty(here, nt_ty, seen_table_prop_pairs, seen_set_types);
      } else {
        // TODO negated unions, intersections, table, and function.
        // Report a TypeError for other types.
        LUAU_ASSERT!(false);
      }
    } else if get_type_id::<NeverType>(there).is_some() {
      here.extern_types.reset_to_never();
    } else if get_type_id::<NoRefineType>(there).is_some() {
      // `*no-refine*` means we will never do anything to affect the intersection.
      here.tyvars = tyvars;
      return NormalizationResult::True;
    } else {
      LUAU_ASSERT!(false);
    }

    let res =
      self.intersect_tyvars_with_ty(&mut tyvars, there, seen_table_prop_pairs, seen_set_types);
    if res != NormalizationResult::True {
      here.tyvars = tyvars;
      return res;
    }
    here.tyvars = tyvars;

    NormalizationResult::True
  }
}
