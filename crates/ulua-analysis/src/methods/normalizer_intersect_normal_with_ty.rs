//! Source: `Analysis/src/Normalize.cpp:3319-3564` (hand-ported)

use alloc::{boxed::Box, collections::BTreeMap};
use core::mem;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`.
use crate::methods::fresh_normalized_type::fresh_normalized_type;
use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type,
    get_singleton_type::get_singleton_type,
    get_type,
    should_early_exit::should_early_exit,
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
    recursion_counter::RecursionCounter,
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

impl Normalizer {
  pub fn intersect_normal_with_ty(
    &mut self,
    here: &mut NormalizedType,
    there: TypeId,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set_types: &mut DenseHashSet<TypeId>,
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

    let there = follow_type::follow(there);

    if get_type::get::<AnyType>(there).is_some() || get_type::get::<UnknownType>(there).is_some() {
      here.tops = self.intersection_of_tops(here.tops, there);
      return NormalizationResult::True;
    } else if get_type::get::<NeverType>(here.tops).is_none() {
      self.clear_normal(here);
      return self.union_normal_with_ty(here, there, seen_table_prop_pairs, seen_set_types, -1);
    } else if let Some(utv) = get_type::get::<UnionType>(there) {
      let mut norm = fresh_normalized_type(self.builtin_types);
      // C++ `for (UnionTypeIterator it = begin(utv); ...)`——迭代器展平嵌套
      // union 并 follow;克隆后再递归。
      let options: Vec<_> = begin_union_type(utv).collect();
      for opt in options {
        let res =
          self.union_normal_with_ty(&mut norm, opt, seen_table_prop_pairs, seen_set_types, -1);
        if res != NormalizationResult::True {
          return res;
        }
      }
      return self.intersect_normals(here, &norm, -1);
    } else if let Some(itv) = get_type::get::<IntersectionType>(there) {
      // C++ `for (IntersectionTypeIterator it = begin(itv); ...)`——同理展平。
      let parts: Vec<_> = begin_intersection_type(itv).collect();
      for part in parts {
        let res = self.intersect_normal_with_ty(here, part, seen_table_prop_pairs, seen_set_types);
        if res != NormalizationResult::True {
          return res;
        }
      }
      return NormalizationResult::True;
    } else if get_type::get::<GenericType>(there).is_some()
      || get_type::get::<FreeType>(there).is_some()
      || get_type::get::<BlockedType>(there).is_some()
      || get_type::get::<PendingExpansionType>(there).is_some()
      || get_type::get::<TypeFunctionInstanceType>(there).is_some()
    {
      let mut there_norm = fresh_normalized_type(self.builtin_types);
      let mut top_norm = fresh_normalized_type(self.builtin_types);
      // SAFETY: builtin_types 指向全局 BuiltinTypes。
      top_norm.tops = self.builtin_types.get().unknown_type;
      there_norm.tyvars.insert(there, Box::new(top_norm));
      here.is_cacheable = false;
      return self.intersect_normals(here, &there_norm, -1);
    }

    let mut tyvars: NormalizedTyvars = mem::take(&mut here.tyvars);

    if get_type::get::<FunctionType>(there).is_some() {
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
    } else if get_type::get::<TableType>(there).is_some()
      || get_type::get::<MetatableType>(there).is_some()
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

        if fflag::LuauExternTypesNormalizeWithShapes.get() {
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
    } else if get_type::get::<ExternType>(there).is_some() {
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
    } else if get_type::get::<ErrorType>(there).is_some() {
      let errors = here.errors;
      self.clear_normal(here);
      here.errors = if get_type::get::<ErrorType>(errors).is_some() {
        errors
      } else {
        there
      };
    } else if let Some(ptv) = get_type::get::<PrimitiveType>(there) {
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
        PrimType::Integer if fflag::LuauIntegerType2.get() => here.integers = integers,
        PrimType::String => here.strings = strings,
        PrimType::Thread => here.threads = threads,
        PrimType::Buffer => here.buffers = buffers,
        PrimType::Function => here.functions = functions,
        PrimType::Table => here.tables = tables,
        _ => LUAU_ASSERT!(false),
      }
    } else if let Some(stv) = get_type::get::<SingletonType>(there) {
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
    } else if let Some(ntv) = get_type::get::<NegationType>(there) {
      let ntv_ty = ntv.ty;
      let t = follow_type::follow(ntv_ty);
      if get_type::get::<PrimitiveType>(t).is_some() {
        self.subtract_primitive(here, ntv_ty);
      } else if get_type::get::<SingletonType>(t).is_some() {
        self.subtract_singleton(here, follow_type::follow(ntv_ty));
      } else if get_type::get::<ExternType>(t).is_some() {
        let res = self.intersect_normal_with_negation_ty(t, here);
        if should_early_exit(res) {
          here.tyvars = tyvars;
          return res;
        }
      } else if let Some(utv) = get_type::get::<UnionType>(t) {
        let options = utv.options.clone();
        for part in options {
          let res = self.intersect_normal_with_negation_ty(part, here);
          if should_early_exit(res) {
            here.tyvars = tyvars;
            return res;
          }
        }
      } else if get_type::get::<AnyType>(t).is_some() {
        // HACK: Refinements sometimes intersect with ~any under the
        // assumption that it is the same as any.
        here.tyvars = tyvars;
        return NormalizationResult::True;
      } else if get_type::get::<NoRefineType>(t).is_some() {
        // `*no-refine*` means we will never do anything to affect the intersection.
        here.tyvars = tyvars;
        return NormalizationResult::True;
      } else if get_type::get::<NeverType>(t).is_some() {
        // intersecting with `~never` is equivalent to intersecting with `unknown` (a noop).
        here.tyvars = tyvars;
        return NormalizationResult::True;
      } else if get_type::get::<UnknownType>(t).is_some() {
        // intersecting with `~unknown` is equivalent to intersecting with `never`.
        self.clear_normal(here);
        here.tyvars = tyvars;
        return NormalizationResult::True;
      } else if get_type::get::<ErrorType>(t).is_some() {
        // ~error is still an error.
        let errors = here.errors;
        self.clear_normal(here);
        here.errors = if get_type::get::<ErrorType>(errors).is_some() {
          errors
        } else {
          t
        };
      } else if let Some(ntv_t) = get_type::get::<NegationType>(t) {
        let nt_ty = ntv_t.ty;
        here.tyvars = tyvars;
        return self.intersect_normal_with_ty(here, nt_ty, seen_table_prop_pairs, seen_set_types);
      } else {
        // TODO negated unions, intersections, table, and function.
        // Report a TypeError for other types.
        LUAU_ASSERT!(false);
      }
    } else if get_type::get::<NeverType>(there).is_some() {
      here.extern_types.reset_to_never();
    } else if get_type::get::<NoRefineType>(there).is_some() {
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
