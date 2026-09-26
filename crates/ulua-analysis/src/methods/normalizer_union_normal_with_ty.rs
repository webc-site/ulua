//! Source: `Analysis/src/Normalize.cpp:1796-1967` (hand-ported)

use alloc::{boxed::Box, vec::Vec};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    assert_invariant::assert_invariant,
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type,
    get_singleton_type::get_singleton_type,
    get_type,
    is_cacheable_normalize::is_cacheable_type_id,
    tyvar_index::tyvar_index,
  },
  methods::{
    fresh_normalized_type::fresh_normalized_type,
    normalized_string_type_reset_to_string::normalized_string_type_reset_to_string,
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
    normalized_type::NormalizedType,
    normalizer::Normalizer,
    pending_expansion_type::PendingExpansionType,
    primitive_type::{PrimitiveType, Type as PrimType},
    recursion_counter::RecursionCounter,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{
    error_type::ErrorType, seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId,
  },
};

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
      let mut tops = self.union_of_tops(here.tops, there);
      if get_type::get::<UnknownType>(tops).is_some()
        && get_type::get::<ErrorType>(here.errors).is_some()
      {
        // SAFETY: self.builtin_types.as_ptr() 指向常驻的 BuiltinTypes
        tops = self.builtin_types.get().any_type;
      }
      self.clear_normal(here);
      here.tops = tops;
      return NormalizationResult::True;
    } else if get_type::get::<NeverType>(there).is_some()
      || get_type::get::<AnyType>(here.tops).is_some()
    {
      return NormalizationResult::True;
    } else if get_type::get::<ErrorType>(there).is_some()
      && get_type::get::<UnknownType>(here.tops).is_some()
    {
      // SAFETY: self.builtin_types.as_ptr() 指向常驻的 BuiltinTypes
      here.tops = self.builtin_types.get().any_type;
      return NormalizationResult::True;
    } else if let Some(utv) = get_type::get::<UnionType>(there) {
      if seen_set_types.contains(&there) {
        return NormalizationResult::True;
      }
      seen_set_types.insert(there);

      // C++ `for (UnionTypeIterator it = begin(utv); ...)`——迭代器展平嵌套
      // union 并 follow;克隆后再递归：递归中 visit 可能改写该 union。
      let options: Vec<_> = begin_union_type(utv).collect();
      for opt in options {
        let res = self.union_normal_with_ty(here, opt, seen_table_prop_pairs, seen_set_types, -1);
        if res != NormalizationResult::True {
          seen_set_types.erase(&there);
          return res;
        }
      }

      seen_set_types.erase(&there);
      return NormalizationResult::True;
    } else if let Some(itv) = get_type::get::<IntersectionType>(there) {
      if seen_set_types.contains(&there) {
        return NormalizationResult::True;
      }
      seen_set_types.insert(there);

      let mut norm = fresh_normalized_type(self.builtin_types);
      // SAFETY: self.builtin_types.as_ptr() 指向常驻的 BuiltinTypes
      norm.tops = self.builtin_types.get().unknown_type;
      // C++ `for (IntersectionTypeIterator it = begin(itv); ...)`——同理展平。
      let parts: Vec<_> = begin_intersection_type(itv).collect();
      for part in parts {
        let res =
          self.intersect_normal_with_ty(&mut norm, part, seen_table_prop_pairs, seen_set_types);
        if res != NormalizationResult::True {
          seen_set_types.erase(&there);
          return res;
        }
      }

      seen_set_types.erase(&there);

      return self.union_normals(here, &norm, -1);
    } else if get_type::get::<UnknownType>(here.tops).is_some() {
      return NormalizationResult::True;
    } else if get_type::get::<GenericType>(there).is_some()
      || get_type::get::<FreeType>(there).is_some()
      || get_type::get::<BlockedType>(there).is_some()
      || get_type::get::<PendingExpansionType>(there).is_some()
      || get_type::get::<TypeFunctionInstanceType>(there).is_some()
    {
      if tyvar_index(there) <= ignore_smaller_tyvars {
        return NormalizationResult::True;
      }
      let mut inter = fresh_normalized_type(self.builtin_types);
      // SAFETY: self.builtin_types.as_ptr() 指向常驻的 BuiltinTypes
      inter.tops = self.builtin_types.get().unknown_type;
      here.tyvars.insert(there, Box::new(inter));

      if !is_cacheable_type_id(there) {
        here.is_cacheable = false;
      }
    } else if get_type::get::<FunctionType>(there).is_some() {
      self.union_functions_with_function(&mut here.functions, there);
    } else if get_type::get::<TableType>(there).is_some()
      || get_type::get::<MetatableType>(there).is_some()
    {
      self.union_tables_with_table(&mut here.tables, there);
    } else if get_type::get::<ExternType>(there).is_some() {
      self.union_extern_types_with_extern_type_normalized_extern_type_type_id(
        &mut here.extern_types,
        there,
      );
    } else if get_type::get::<ErrorType>(there).is_some() {
      here.errors = there;
    } else if let Some(ptv) = get_type::get::<PrimitiveType>(there) {
      match ptv.r#type {
        PrimType::Boolean => here.booleans = there,
        PrimType::NilType => here.nils = there,
        PrimType::Number => here.numbers = there,
        PrimType::Integer if fflag::LuauIntegerType2.get() => here.integers = there,
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
    } else if let Some(stv) = get_type::get::<SingletonType>(there) {
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
    } else if let Some(ntv) = get_type::get::<NegationType>(there) {
      let ntv_ty = ntv.ty;

      // C++ 中归一化失败返回空指针：直接返回 False
      let Some(there_normal) = self.try_normalize(ntv_ty) else {
        return NormalizationResult::False;
      };
      let tn = self.negate_normal(&there_normal);

      let tn = match tn {
        Some(t) => t,
        None => return NormalizationResult::False,
      };

      let res = self.union_normals(here, &tn, -1);
      if res != NormalizationResult::True {
        return res;
      }
    } else if get_type::get::<PendingExpansionType>(there).is_some()
      || get_type::get::<TypeFunctionInstanceType>(there).is_some()
      || get_type::get::<NoRefineType>(there).is_some()
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
