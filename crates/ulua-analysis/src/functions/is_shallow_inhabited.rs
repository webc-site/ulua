use ulua_common::fflag;

use crate::{
  functions::get_type,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

pub fn is_shallow_inhabited(norm: &NormalizedType) -> bool {
  let luau_integer_type_2 = fflag::LuauIntegerType2.get();

  let tops_is_never = get_type::get::<NeverType>(norm.tops).is_some();
  let booleans_is_never = get_type::get::<NeverType>(norm.booleans).is_some();
  let extern_types_is_never = !norm.extern_types.is_never();
  let errors_is_never = get_type::get::<NeverType>(norm.errors).is_some();
  let nils_is_never = get_type::get::<NeverType>(norm.nils).is_some();
  let numbers_is_never = get_type::get::<NeverType>(norm.numbers).is_some();
  let strings_is_never = !norm.strings.is_never();
  let threads_is_never = get_type::get::<NeverType>(norm.threads).is_some();
  let buffers_is_never = get_type::get::<NeverType>(norm.buffers).is_none();
  let functions_is_never = !norm.functions.is_never();
  let tables_not_empty = norm.tables.size() != 0;
  let tyvars_not_empty = !norm.tyvars.is_empty();

  if luau_integer_type_2 {
    let integers_is_never = get_type::get::<NeverType>(norm.integers).is_none();
    tops_is_never
      || booleans_is_never
      || extern_types_is_never
      || errors_is_never
      || nils_is_never
      || numbers_is_never
      || strings_is_never
      || threads_is_never
      || buffers_is_never
      || functions_is_never
      || tables_not_empty
      || tyvars_not_empty
      || integers_is_never
  } else {
    tops_is_never
      || booleans_is_never
      || extern_types_is_never
      || errors_is_never
      || nils_is_never
      || numbers_is_never
      || strings_is_never
      || threads_is_never
      || get_type::get::<NeverType>(norm.buffers).is_some()
      || functions_is_never
      || tables_not_empty
      || tyvars_not_empty
  }
}
