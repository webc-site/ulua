//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1350:type_pack_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1350-1355` (hand-ported)

use crate::{
  records::{blocked_type_pack::BlockedTypePack, type_pack_stringifier::TypePackStringifier},
  type_aliases::type_pack_id::TypePackId,
};

impl TypePackStringifier {
  /// C++ `void operator()(TypePackId, const BlockedTypePack& btp)`.
  pub fn operator_call_3(&mut self, _id: TypePackId, btp: &BlockedTypePack) {
    unsafe {
      (*self.state).emit("*blocked-tp-");
      (*self.state).emit(&btp.index);
      (*self.state).emit("*");
    }
  }
}
