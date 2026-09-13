use core::ptr::NonNull;

use crate::{
  enums::value::Value,
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    error_suppression::ErrorSuppression, normalizer::Normalizer,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

/// # Safety
/// 调用方须保证 `normalizer` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
/// Rust translation of Luau.Analysis::Analysis::TypeUtils.cpp:should_suppress_errors
pub unsafe fn should_suppress_errors(normalizer: *mut Normalizer, ty: TypeId) -> ErrorSuppression {
  // SAFETY: C++ NotNull<Normalizer> 契约；NonNull 封装解引用。
  let normalizer = unsafe { NonNull::new_unchecked(normalizer).as_mut() };

  let ty = follow_type_id(ty);

  if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty) {
    for &arg_ty in tfit.type_arguments.iter() {
      let Some(norm_type) = normalizer.try_normalize(arg_ty) else {
        return ErrorSuppression::from_value(Value::NormalizationFailed);
      };

      if norm_type.should_suppress_errors() {
        return ErrorSuppression::from_value(Value::Suppress);
      }
    }

    return ErrorSuppression::from_value(Value::DoNotSuppress);
  }

  let Some(norm_type) = normalizer.try_normalize(ty) else {
    return ErrorSuppression::from_value(Value::NormalizationFailed);
  };

  if norm_type.should_suppress_errors() {
    ErrorSuppression::from_value(Value::Suppress)
  } else {
    ErrorSuppression::from_value(Value::DoNotSuppress)
  }
}
