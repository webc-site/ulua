//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1175:type_pack_stringifier_type_pack_stringifier`
//! Source: `Analysis/src/ToString.cpp:1175-1179` (hand-ported)

use crate::records::{
  function_argument::FunctionArgument, stringifier_state::StringifierState,
  type_pack_stringifier::TypePackStringifier,
};

impl TypePackStringifier {
  /// C++ `explicit TypePackStringifier(StringifierState& state, const std::vector<std::optional<FunctionArgument>>& elemNames)`.
  pub fn type_pack_stringifier_stringifier_state_vector_optional_function_argument(
    state: *mut StringifierState,
    elem_names: &[Option<FunctionArgument>],
  ) -> Self {
    Self {
      state,
      elem_names: elem_names.to_vec(),
      elem_index: 0,
    }
  }
}
