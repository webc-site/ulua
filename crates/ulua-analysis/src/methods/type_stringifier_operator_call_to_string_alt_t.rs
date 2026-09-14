//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1118:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1118-1133` (hand-ported)

use crate::{
  functions::{follow_type::follow, get_type_alt_j::get},
  records::{
    intersection_type::IntersectionType, negation_type::NegationType,
    type_stringifier::TypeStringifier, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId, const NegationType& ntv)`.
  pub fn operator_call_14(&mut self, _ty: TypeId, ntv: &NegationType) {
    unsafe {
      (*self.state).emit("~");

      // The precedence of `~` should be less than `|` and `&`.
      let followed = follow(ntv.ty);
      let parens =
        !get::<UnionType>(followed).is_none() || !get::<IntersectionType>(followed).is_none();

      if parens {
        (*self.state).emit("(");
      }

      self.stringify_type_id(ntv.ty);

      if parens {
        (*self.state).emit(")");
      }
    }
  }
}
