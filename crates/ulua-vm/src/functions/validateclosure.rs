use core::ptr::{addr_of, from_ref};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::obj_2_gco::obj2gco,
  records::{closure::Closure, gc_object::GCObject, global_state::global_State},
};

/// # Safety
/// `g` 须指向存活 global_State；`cl` 须为存活 Closure，其 `inner.c.upvals`/`inner.l.uprefs`
/// 覆盖 `nupvalues` 项且 `inner.l.p` 可读（校验仅读取引用字段与着色位）。
pub(crate) unsafe fn validateclosure(g: *mut global_State, cl: &Closure) {
  unsafe {
    // 引用源身份：只读校验，从共享借用降 `*const` 即够用
    let obj: *const GCObject = from_ref(cl).cast();
    validateobjref(g, obj, obj2gco!(cl.env));

    let nups = cl.nupvalues as usize;

    if cl.is_c != 0 {
      let c = addr_of!(cl.inner.c);
      for upval in c_slice((*c).upvals.as_ptr(), nups) {
        validateref(g, obj, upval);
      }
    } else {
      let l = addr_of!(cl.inner.l);
      LUAU_ASSERT!(cl.nupvalues as i32 == (*(*l).p).nups as i32);

      validateobjref(g, obj, obj2gco!((*l).p));

      for r in c_slice((*l).uprefs.as_ptr(), nups) {
        validateref(g, obj, r);
      }
    }
  }
}
