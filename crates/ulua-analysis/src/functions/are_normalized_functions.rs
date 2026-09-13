use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{function_type::FunctionType, normalized_function_type::NormalizedFunctionType},
  type_aliases::error_type::ErrorType,
};

/// C++ `static bool areNormalizedFunctions(const NormalizedFunctionType& tys)`.
pub fn are_normalized_functions(tys: &NormalizedFunctionType) -> bool {
  for &ty in &tys.parts.order {
    if get_type_id::<FunctionType>(ty).is_none() && get_type_id::<ErrorType>(ty).is_none() {
      return false;
    }
  }

  true
}
