//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1113:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1113-1116` (hand-ported)

use crate::{
  records::{never_type::NeverType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId, const NeverType&)`.
  pub fn operator_call_15(&mut self, _ty: TypeId, _ttv: &NeverType) {
    unsafe { (*self.state).emit("never") }
  }
}
