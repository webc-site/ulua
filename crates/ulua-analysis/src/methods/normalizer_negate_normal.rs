use ulua_common::fflag;

use crate::{
  functions::{
    get_singleton_type::get_singleton_type, get_type_alt_j::get_type_id, is_top::is_top,
    reset_to_top::reset_to_top,
  },
  methods::fresh_normalized_type::fresh_normalized_type,
  records::{
    boolean_singleton::BooleanSingleton, never_type::NeverType, normalized_type::NormalizedType,
    normalizer::Normalizer, primitive_type::PrimitiveType, singleton_type::SingletonType,
    type_ids::TypeIds,
  },
};

impl Normalizer {
  pub fn negate_normal(&mut self, here: &NormalizedType) -> Option<NormalizedType> {
    self.consume_fuel();

    let mut result = fresh_normalized_type(here.builtin_types);
    result.is_cacheable = here.is_cacheable;

    // SAFETY: here.builtin_types 由构造方保证有效（C++ 同契约）；BuiltinTypes 构造后
    // 不可变，字段读取提前到函数头与按需读取等价，均为 Copy。
    let bt = here.builtin_types;
    let boolean_type = unsafe { (*bt).boolean_type };
    let never_type = unsafe { (*bt).never_type };
    let false_type = unsafe { (*bt).false_type };
    let true_type = unsafe { (*bt).true_type };
    let extern_type = unsafe { (*bt).extern_type };
    let nil_type = unsafe { (*bt).nil_type };
    let number_type = unsafe { (*bt).number_type };
    let integer_type = unsafe { (*bt).integer_type };
    let thread_type = unsafe { (*bt).thread_type };
    let buffer_type = unsafe { (*bt).buffer_type };
    let table_type = unsafe { (*bt).table_type };

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
      result.booleans = boolean_type;
    } else if get_type_id::<PrimitiveType>(here_booleans).is_some() {
      result.booleans = never_type;
    } else if let Some(here_booleans_tv) = get_type_id::<SingletonType>(here_booleans)
      && let Some(boolean) = get_singleton_type::<BooleanSingleton>(here_booleans_tv)
    {
      if boolean.value {
        result.booleans = false_type;
      } else {
        result.booleans = true_type;
      }
    }

    let extern_types = &here.extern_types;
    if extern_types.is_never() {
      reset_to_top(unsafe { &*bt }, &mut result.extern_types);
    } else if is_top(unsafe { &*bt }, extern_types) {
      result.extern_types.reset_to_never();
    } else {
      let mut root_negations = TypeIds::new();

      for (here_parent, here_negations) in &extern_types.extern_types {
        if *here_parent != extern_type {
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
        result.extern_types.push_pair(extern_type, root_negations);
      }
    }

    if get_type_id::<NeverType>(here.nils).is_some() {
      result.nils = nil_type;
    } else {
      result.nils = never_type;
    }

    if get_type_id::<NeverType>(here.numbers).is_some() {
      result.numbers = number_type;
    } else {
      result.numbers = never_type;
    }

    if fflag::LuauIntegerType2.get() {
      if get_type_id::<NeverType>(here.integers).is_some() {
        result.integers = integer_type;
      } else {
        result.integers = never_type;
      }
    }

    result.strings = here.strings.clone();
    result.strings.is_cofinite = !result.strings.is_cofinite;

    if get_type_id::<NeverType>(here.threads).is_some() {
      result.threads = thread_type;
    } else {
      result.threads = never_type;
    }

    if get_type_id::<NeverType>(here.buffers).is_some() {
      result.buffers = buffer_type;
    } else {
      result.buffers = never_type;
    }

    if here.functions.is_never() {
      result.functions.reset_to_top();
    } else if here.functions.is_top {
      result.functions.reset_to_never();
    } else {
      return None;
    }

    if here.tables.empty() {
      result.tables.insert_type_id(table_type);
    } else if here.tables.size() == 1 && here.tables.front() == table_type {
      result.tables.clear();
    } else {
      return None;
    }

    Some(result)
  }
}
