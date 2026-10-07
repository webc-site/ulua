//! Source: `VM/src/lgc.cpp` (lgc.cpp:398-415, hand-ported)

use core::ptr::addr_of_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::c_slice,
  macros::{markobject::markobject, markvalue::markvalue, stringmark::stringmark},
  records::{closure::Closure, global_state::global_State},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn traverseclosure(g: *mut global_State, cl: *mut Closure) {
  unsafe {
    markobject!(g, (*cl).env);
    if (*cl).is_c != 0 {
      // ManuallyDrop is repr(transparent); upvals/uprefs are C flexible arrays
      let c = addr_of_mut!((*cl).inner.c);
      // debugname 正确性不变式（cpp lgc.cpp:430 `if (TString* str = cl->c.debugname)
      // stringmark(str)`）：CClosure.debugname 锚定的是串表（intern 弱表）里的 TString，
      // 闭包自身是它唯一的强引用边。串表在 GC atomic 阶段按可达性清理桶链，漏标则
      // sweep 视该串不可达而回收，闭包内指针随即悬垂（currfuncname/getinfo/dump 复发
      // use-after-free）。与 traverseproto 对 f.debugname 的处理同形态。
      let debugname = (*c).debugname;
      if !debugname.is_null() {
        stringmark!(debugname);
      }
      let upvals = addr_of_mut!((*c).upvals) as *mut TValue;
      // SAFETY:upvals 为 C 柔性数组，nupvalues 个元素随 Closure 一并分配。
      for upval in c_slice(upvals, (*cl).nupvalues as usize) {
        // mark its upvalues
        markvalue!(g, upval);
      }
    } else {
      let l = addr_of_mut!((*cl).inner.l);
      LUAU_ASSERT!((*cl).nupvalues as i32 == (*(*l).p).nups as i32);
      markobject!(g, (*l).p);
      let uprefs = addr_of_mut!((*l).uprefs) as *mut TValue;
      // SAFETY:uprefs 为 C 柔性数组，nupvalues 个元素随 Closure 一并分配。
      for upref in c_slice(uprefs, (*cl).nupvalues as usize) {
        // mark its upvalues
        markvalue!(g, upref);
      }
    }
  }
}
