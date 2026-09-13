//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:578:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:578-583` (hand-ported)

use crate::{
  records::{pending_expansion_type::PendingExpansionType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId ty, const PendingExpansionType& petv)`.
  pub fn operator_call_6(&mut self, _ty: TypeId, petv: &PendingExpansionType) {
    unsafe {
      (*self.state).emit("*pending-expansion-");
      (*self.state).emit(&petv.index);
      (*self.state).emit("*");
    }
  }
}
