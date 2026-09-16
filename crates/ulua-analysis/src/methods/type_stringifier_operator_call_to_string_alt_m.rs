//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:882:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:882-885` (hand-ported)

use crate::{
  records::{no_refine_type::NoRefineType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId, const NoRefineType&)`.
  pub fn operator_call_16(&mut self, _ty: TypeId, _nrtv: &NoRefineType) {
    unsafe { (*self.state).emit("*no-refine*") }
  }
}
