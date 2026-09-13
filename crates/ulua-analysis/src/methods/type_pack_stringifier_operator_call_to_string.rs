//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1220:type_pack_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1220-1272` (hand-ported)

use core::ffi::c_void;

use ulua_common::FInt;

use crate::{
  functions::{follow_type_pack::follow_type_pack_id, get_type_pack::get, is_empty::is_empty},
  records::{
    type_pack::TypePack, type_pack_stringifier::TypePackStringifier,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl TypePackStringifier {
  /// C++ `void operator()(TypePackId, const TypePack& tp)`.
  pub fn operator_call_7(&mut self, _id: TypePackId, tp: &TypePack) {
    unsafe {
      if (*self.state).has_seen(tp as *const TypePack as *const c_void) {
        (*(*self.state).result).cycle = true;
        (*self.state).emit("*CYCLETP*");
        return;
      }

      if tp.head.is_empty() && (tp.tail.is_none() || is_empty(tp.tail.unwrap())) {
        (*self.state).emit("()");
        (*self.state).unsee(tp as *const TypePack as *const c_void);
        return;
      }

      let mut first = true;

      for &type_id in tp.head.iter() {
        if first {
          first = false;
        } else {
          (*self.state).emit(", ");
        }

        // Do not respect opts.namedFunctionOverrideArgNames here
        let idx = self.elem_index as usize;
        if idx < self.elem_names.len()
          && let Some(elem_name) = &self.elem_names[idx]
        {
          (*self.state).emit(elem_name.name.as_str());
          (*self.state).emit(": ");
        }

        self.elem_index += 1;

        self.stringify_type_id(type_id);
      }

      if let Some(tp_tail) = tp.tail
        && !is_empty(tp_tail)
      {
        let tail = follow_type_pack_id(tp_tail);
        let vtp = get::<VariadicTypePack>(tail);
        let should_emit = match vtp {
          None => true,
          Some(v) => FInt::DebugLuauVerboseTypeNames.get() < 1 && !v.hidden,
        };
        if should_emit {
          if first {
            // first = false; (C++ writes it; nothing reads it after)
          } else {
            (*self.state).emit(", ");
          }

          self.stringify_type_pack_id(tail);
        }
      }

      (*self.state).unsee(tp as *const TypePack as *const c_void);
    }
  }
}
