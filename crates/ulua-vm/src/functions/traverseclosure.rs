//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:398:traverseclosure`
//! Source: `VM/src/lgc.cpp` (lgc.cpp:398-415, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::c_slice,
  macros::{markobject::markobject, markvalue::markvalue},
  records::{closure::Closure, global_state::global_State},
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn traverseclosure(g: *mut global_State, cl: *mut Closure) {
  unsafe {
    markobject!(g, (*cl).env);
    if (*cl).is_c != 0 {
      // ManuallyDrop is repr(transparent); upvals/uprefs are C flexible arrays
      let c = core::ptr::addr_of_mut!((*cl).inner.c);
      let upvals = core::ptr::addr_of_mut!((*c).upvals) as *mut TValue;
      // SAFETY：upvals 为 C 柔性数组，nupvalues 个元素随 Closure 一并分配。
      for upval in c_slice(upvals, (*cl).nupvalues as usize) {
        // mark its upvalues
        markvalue!(g, upval);
      }
    } else {
      let l = core::ptr::addr_of_mut!((*cl).inner.l);
      LUAU_ASSERT!((*cl).nupvalues as i32 == (*(*l).p).nups as i32);
      markobject!(g, (*l).p);
      let uprefs = core::ptr::addr_of_mut!((*l).uprefs) as *mut TValue;
      // SAFETY：uprefs 为 C 柔性数组，nupvalues 个元素随 Closure 一并分配。
      for upref in c_slice(uprefs, (*cl).nupvalues as usize) {
        // mark its upvalues
        markvalue!(g, upref);
      }
    }
  }
}
