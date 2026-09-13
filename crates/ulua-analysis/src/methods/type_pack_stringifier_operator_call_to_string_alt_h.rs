//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1357:type_pack_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1357-1380` (hand-ported)

use crate::{
  records::{
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_pack_stringifier::TypePackStringifier,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl TypePackStringifier {
  /// C++ `void operator()(TypePackId, const TypeFunctionInstanceTypePack& tfitp)`.
  pub fn operator_call_6(&mut self, _id: TypePackId, tfitp: &TypeFunctionInstanceTypePack) {
    unsafe {
      (*self.state).emit((*tfitp.function).name.as_str());
      (*self.state).emit("<");

      let mut comma = false;
      for &p in tfitp.type_arguments.iter() {
        if comma {
          (*self.state).emit(", ");
        }

        comma = true;
        self.stringify_type_id(p);
      }

      for &p in tfitp.pack_arguments.iter() {
        if comma {
          (*self.state).emit(", ");
        }

        comma = true;
        self.stringify_type_pack_id(p);
      }

      (*self.state).emit(">");
    }
  }
}
