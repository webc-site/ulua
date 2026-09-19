/// Rust translation of Luau.Analysis::Analysis::TypeUtils.cpp:should_suppress_errors (TypePackId overload)
use core::ptr::null_mut;

use crate::{
  enums::value::Value,
  functions::{
    finite::finite, flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id,
    should_suppress_errors_type_utils::should_suppress_errors as should_suppress_errors_not_null_normalizer_type_id,
  },
  records::{
    any_type::AnyType, error_suppression::ErrorSuppression, normalizer::Normalizer,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};
pub(crate) fn should_suppress_errors_not_null_normalizer_type_pack_id(
  normalizer: *mut Normalizer,
  tp: TypePackId,
) -> ErrorSuppression {
  let tp = unsafe { follow_type_pack_id(tp) };

  if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp) {
    let ty = follow_type_id(vtp.ty);
    if get_type_id::<AnyType>(ty).is_some() {
      return ErrorSuppression::from_value(Value::Suppress);
    }
  }

  let (tys, tail) = flatten_type_pack_id(tp);

  for ty in tys {
    let result = unsafe { should_suppress_errors_not_null_normalizer_type_id(normalizer, ty) };
    if result != ErrorSuppression::from_value(Value::DoNotSuppress) {
      return result;
    }
  }

  if let Some(tail_tp) = tail
    && tp != tail_tp
    && unsafe { finite(tail_tp, null_mut()) }
  {
    return should_suppress_errors_not_null_normalizer_type_pack_id(normalizer, tail_tp);
  }

  ErrorSuppression::from_value(Value::DoNotSuppress)
}
