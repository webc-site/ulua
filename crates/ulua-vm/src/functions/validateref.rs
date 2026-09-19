use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::validateobjref::validateobjref,
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, ttype::ttype},
  records::{gc_object::GCObject, global_state::global_State},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn validateref(g: *mut global_State, f: *mut GCObject, v: &TValue) {
  unsafe {
    if iscollectable!(v) {
      LUAU_ASSERT!(ttype!(v) == (*gcvalue!(v)).gch.tt as i32);
      validateobjref(g, f, gcvalue!(v));
    }
  }
}
