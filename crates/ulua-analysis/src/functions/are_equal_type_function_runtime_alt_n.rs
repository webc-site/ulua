use crate::{
  functions::are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type,
  records::{are_equal_state::AreEqualState, type_function_type_pack::TypeFunctionTypePack},
};

pub fn are_equal_are_equal_state_type_function_type_pack_type_function_type_pack(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTypePack,
  rhs: &TypeFunctionTypePack,
) -> bool {
  if lhs.head.len() != rhs.head.len() {
    return false;
  }

  // head 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.head.iter().zip(&rhs.head) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      unsafe { &*l },
      unsafe { &*r },
    ) {
      return false;
    }
  }

  true
}
