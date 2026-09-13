//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:571:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:571-576` (hand-ported)

use crate::{
  records::{blocked_type::BlockedType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId, const BlockedType& btv)`.
  pub fn operator_call_9(&mut self, _ty: TypeId, btv: &BlockedType) {
    unsafe {
      (*self.state).emit("*blocked-");
      (*self.state).emit(&btv.index);
      (*self.state).emit("*");
    }
  }
}
