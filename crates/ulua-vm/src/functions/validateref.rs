use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::validateobjref::validateobjref,
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, ttype::ttype},
  records::{gc_object::GCObject, global_state::global_State},
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn validateref(g: *mut global_State, f: *mut GCObject, v: &TValue) {
  unsafe {
    if iscollectable!(v) {
      LUAU_ASSERT!(ttype!(v) == (*gcvalue!(v)).gch.tt as i32);
      validateobjref(g, f, gcvalue!(v));
    }
  }
}
