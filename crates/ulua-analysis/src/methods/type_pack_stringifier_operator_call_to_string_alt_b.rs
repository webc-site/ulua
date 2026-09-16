//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1274:type_pack_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1274-1286` (hand-ported)

use crate::{
  records::type_pack_stringifier::TypePackStringifier,
  type_aliases::{error_type_pack::ErrorTypePack, type_pack_id::TypePackId},
};

impl TypePackStringifier {
  /// C++ `void operator()(TypePackId, const ErrorTypePack& error)`.
  pub fn operator_call_5(&mut self, _id: TypePackId, error: &ErrorTypePack) {
    unsafe {
      (*(*self.state).result).error = true;

      if let Some(synthetic) = error.synthetic {
        (*self.state).emit("*");
        self.stringify_type_pack_id(synthetic);
        (*self.state).emit("*");
      } else {
        (*self.state).emit("*error-type*");
      }
    }
  }
}
