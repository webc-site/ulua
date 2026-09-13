use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::obj_2_gco::obj2gco,
  records::{closure::Closure, global_state::global_State},
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn validateclosure(g: *mut global_State, cl: *mut Closure) {
  unsafe {
    validateobjref(g, obj2gco!(cl), obj2gco!((*cl).env));

    let nups = (*cl).nupvalues as usize;

    if (*cl).is_c != 0 {
      let c = core::ptr::addr_of_mut!((*cl).inner.c);
      for upval in c_slice((*c).upvals.as_mut_ptr(), nups) {
        validateref(g, obj2gco!(cl), upval);
      }
    } else {
      let l = core::ptr::addr_of_mut!((*cl).inner.l);
      LUAU_ASSERT!((*cl).nupvalues as i32 == (*(*l).p).nups as i32);

      validateobjref(g, obj2gco!(cl), obj2gco!((*l).p));

      let uprefs = core::ptr::addr_of_mut!((*l).uprefs) as *mut TValue;
      for r in c_slice(uprefs, nups) {
        validateref(g, obj2gco!(cl), r);
      }
    }
  }
}
