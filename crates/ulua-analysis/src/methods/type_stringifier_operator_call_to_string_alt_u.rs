//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1135:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1135-1164` (hand-ported)

use crate::{
  records::{
    type_function_instance_type::TypeFunctionInstanceType, type_stringifier::TypeStringifier,
  },
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId, const TypeFunctionInstanceType& tfitv)`.
  pub fn operator_call_19(&mut self, _ty: TypeId, tfitv: &TypeFunctionInstanceType) {
    unsafe {
      if let Some(user_func_name) = &tfitv.user_func_name {
        // Special stringification for user-defined type functions
        (*self.state).emit(user_func_name.as_str_or_empty());
      } else {
        (*self.state).emit(tfitv.function.as_ref().name.as_str());
      }

      (*self.state).emit("<");

      let mut comma = false;
      for &ty in tfitv.type_arguments.iter() {
        if comma {
          (*self.state).emit(", ");
        }

        comma = true;
        self.stringify_type_id(ty);
      }

      for &tp in tfitv.pack_arguments.iter() {
        if comma {
          (*self.state).emit(", ");
        }

        comma = true;
        self.stringify_type_pack_id(tp);
      }

      (*self.state).emit(">");
    }
  }
}
