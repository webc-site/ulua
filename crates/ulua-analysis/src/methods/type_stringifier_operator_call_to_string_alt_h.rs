//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:643:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:643-710` (hand-ported)

use core::ffi::c_void;

use crate::{
  functions::{begin_type_pack::begin, end_type_pack::end, is_empty::is_empty},
  records::{function_type::FunctionType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId, const FunctionType& ftv)`.
  pub fn operator_call_12(&mut self, _ty: TypeId, ftv: &FunctionType) {
    unsafe {
      if (*self.state).has_seen(ftv as *const FunctionType as *const c_void) {
        (*(*self.state).result).cycle = true;
        (*self.state).emit("*CYCLE*");
        return;
      }

      // We should not be respecting opts.hideNamedFunctionTypeParameters here.
      if !ftv.generics.is_empty() || !ftv.generic_packs.is_empty() {
        (*self.state).emit("<");
        let mut comma = false;
        for &g in ftv.generics.iter() {
          if comma {
            (*self.state).emit(", ");
          }
          comma = true;
          self.stringify_type_id(g);
        }
        for &gp in ftv.generic_packs.iter() {
          if comma {
            (*self.state).emit(", ");
          }
          comma = true;
          self.stringify_type_pack_id(gp);
        }
        (*self.state).emit(">");
      }

      if ftv.is_checked_function {
        (*self.state).emit("@checked ");
      }

      (*self.state).emit("(");

      if is_empty(ftv.arg_types) {
        // if we've got an empty argument pack, we're done.
      } else if (*(*self.state).opts).function_type_arguments {
        self
          .stringify_type_pack_id_vector_optional_function_argument(ftv.arg_types, &ftv.arg_names);
      } else {
        self.stringify_type_pack_id(ftv.arg_types);
      }

      (*self.state).emit(") -> ");

      let mut plural = !is_empty(ftv.ret_types);

      let mut ret_begin = begin(ftv.ret_types);
      let ret_end = end(ftv.ret_types);
      if ret_begin.operator_ne(&ret_end) {
        ret_begin.operator_inc();
        if ret_begin.operator_eq(&ret_end) && ret_begin.tail().is_none() {
          plural = false;
        }
      }

      if plural {
        (*self.state).emit("(");
      }

      self.stringify_type_pack_id(ftv.ret_types);

      if plural {
        (*self.state).emit(")");
      }

      (*self.state).unsee(ftv as *const FunctionType as *const c_void);
    }
  }
}
