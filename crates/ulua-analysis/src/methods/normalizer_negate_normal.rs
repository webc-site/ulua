use alloc::collections::BTreeMap;

use ulua_common::FFlag;

use crate::{
  functions::{
    get_singleton_type::get_singleton_type, get_type_alt_j::get_type_id, is_top::is_top,
    reset_to_top::reset_to_top,
  },
  records::{
    boolean_singleton::BooleanSingleton, never_type::NeverType,
    normalized_extern_type::NormalizedExternType, normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType, normalized_type::NormalizedType,
    normalizer::Normalizer, primitive_type::PrimitiveType, singleton_type::SingletonType,
    type_ids::TypeIds,
  },
};

impl Normalizer {
  pub fn negate_normal(&mut self, here: &NormalizedType) -> Option<NormalizedType> {
    self.consume_fuel();

    let never_type = unsafe { (*here.builtin_types).never_type };
    let mut result = NormalizedType {
      builtin_types: here.builtin_types,
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
    };
    result.is_cacheable = here.is_cacheable;

    let here_tops = get_type_id::<NeverType>(here.tops);
    if here_tops.is_none() {
      return Some(result);
    }

    let here_errors = get_type_id::<NeverType>(here.errors);
    if here_errors.is_none() {
      result.errors = here.errors;
      return Some(result);
    }

    let here_booleans = here.booleans;
    if get_type_id::<NeverType>(here_booleans).is_some() {
      result.booleans = unsafe { (*here.builtin_types).boolean_type };
    } else if get_type_id::<PrimitiveType>(here_booleans).is_some() {
      result.booleans = unsafe { (*here.builtin_types).never_type };
    } else if let Some(here_booleans_tv) = get_type_id::<SingletonType>(here_booleans)
      && let Some(boolean) = get_singleton_type::<BooleanSingleton>(here_booleans_tv)
    {
      if boolean.value {
        result.booleans = unsafe { (*here.builtin_types).false_type };
      } else {
        result.booleans = unsafe { (*here.builtin_types).true_type };
      }
    }

    let extern_types = &here.extern_types;
    if extern_types.is_never() {
      reset_to_top(unsafe { &*here.builtin_types }, &mut result.extern_types);
    } else if is_top(unsafe { &*here.builtin_types }, extern_types) {
      result.extern_types.reset_to_never();
    } else {
      let mut root_negations = TypeIds::new();

      for (here_parent, here_negations) in &extern_types.extern_types {
        if *here_parent != unsafe { (*here.builtin_types).extern_type } {
          root_negations.insert_type_id(*here_parent);
        }

        for &here_negation in &here_negations.order {
          self.union_extern_types_with_extern_type_normalized_extern_type_type_id(
            &mut result.extern_types,
            here_negation,
          );
        }
      }

      if !root_negations.empty() {
        result
          .extern_types
          .push_pair(unsafe { (*here.builtin_types).extern_type }, root_negations);
      }
    }

    if get_type_id::<NeverType>(here.nils).is_some() {
      result.nils = unsafe { (*here.builtin_types).nil_type };
    } else {
      result.nils = unsafe { (*here.builtin_types).never_type };
    }

    if get_type_id::<NeverType>(here.numbers).is_some() {
      result.numbers = unsafe { (*here.builtin_types).number_type };
    } else {
      result.numbers = unsafe { (*here.builtin_types).never_type };
    }

    if FFlag::LuauIntegerType2.get() {
      if get_type_id::<NeverType>(here.integers).is_some() {
        result.integers = unsafe { (*here.builtin_types).integer_type };
      } else {
        result.integers = unsafe { (*here.builtin_types).never_type };
      }
    }

    result.strings = here.strings.clone();
    result.strings.is_cofinite = !result.strings.is_cofinite;

    if get_type_id::<NeverType>(here.threads).is_some() {
      result.threads = unsafe { (*here.builtin_types).thread_type };
    } else {
      result.threads = unsafe { (*here.builtin_types).never_type };
    }

    if get_type_id::<NeverType>(here.buffers).is_some() {
      result.buffers = unsafe { (*here.builtin_types).buffer_type };
    } else {
      result.buffers = unsafe { (*here.builtin_types).never_type };
    }

    if here.functions.is_never() {
      result.functions.reset_to_top();
    } else if here.functions.is_top {
      result.functions.reset_to_never();
    } else {
      return None;
    }

    if here.tables.empty() {
      result
        .tables
        .insert_type_id(unsafe { (*here.builtin_types).table_type });
    } else if here.tables.size() == 1
      && here.tables.front() == unsafe { (*here.builtin_types).table_type }
    {
      result.tables.clear();
    } else {
      return None;
    }

    Some(result)
  }
}
