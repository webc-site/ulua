//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:855:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:855-870` (hand-ported)

use crate::{
  records::{metatable_type::MetatableType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId ty, const MetatableType& mtv)`.
  pub fn operator_call_5(&mut self, ty: TypeId, mtv: &MetatableType) {
    unsafe {
      (*(*self.state).result).invalid = true;
      if !(*self.state).exhaustive
        && let Some(synthetic_name) = &mtv.synthetic_name
      {
        (*self.state).emit_and_record_span(synthetic_name, ty);
        return;
      }

      (*self.state).emit("{ @metatable ");
      self.stringify_type_id(mtv.metatable);
      (*self.state).emit(",");
      (*self.state).newline();
      self.stringify_type_id(mtv.table);
      (*self.state).emit(" }");
    }
  }
}
