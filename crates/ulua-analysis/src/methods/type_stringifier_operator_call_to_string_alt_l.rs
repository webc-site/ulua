//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:877:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:877-880` (hand-ported)

use crate::{
  records::{any_type::AnyType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId, const AnyType&)`.
  pub fn operator_call_8(&mut self, _ty: TypeId, _atv: &AnyType) {
    unsafe { (*self.state).emit("any") }
  }
}
