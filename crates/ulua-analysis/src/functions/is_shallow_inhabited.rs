use ulua_common::FFlag;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

pub fn is_shallow_inhabited(norm: &NormalizedType) -> bool {
  let luau_integer_type_2 = FFlag::LuauIntegerType2.get();

  let tops_is_never = !get_type_id::<NeverType>(norm.tops).is_none();
  let booleans_is_never = !get_type_id::<NeverType>(norm.booleans).is_none();
  let extern_types_is_never = !norm.extern_types.is_never();
  let errors_is_never = !get_type_id::<NeverType>(norm.errors).is_none();
  let nils_is_never = !get_type_id::<NeverType>(norm.nils).is_none();
  let numbers_is_never = !get_type_id::<NeverType>(norm.numbers).is_none();
  let strings_is_never = !norm.strings.is_never();
  let threads_is_never = !get_type_id::<NeverType>(norm.threads).is_none();
  let buffers_is_never = get_type_id::<NeverType>(norm.buffers).is_none();
  let functions_is_never = !norm.functions.is_never();
  let tables_not_empty = norm.tables.size() != 0;
  let tyvars_not_empty = !norm.tyvars.is_empty();

  if luau_integer_type_2 {
    let integers_is_never = get_type_id::<NeverType>(norm.integers).is_none();
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
      || !get_type_id::<NeverType>(norm.buffers).is_none()
      || functions_is_never
      || tables_not_empty
      || tyvars_not_empty
  }
}
