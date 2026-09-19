//! Source: `VM/src/lgc.cpp` (lgc.cpp:417-430, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{
    markobject::markobject, markvalue::markvalue, stringmark::stringmark, upisopen::upisopen,
  },
  records::global_state::global_State,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn traversestack(g: *mut global_State, l: *mut lua_State) {
  unsafe {
    markobject!(g, (*l).gt);
    if !(*l).namecall.is_null() {
      stringmark!((*l).namecall);
    }
    let mut o = (*l).stack;
    while o < (*l).top {
      markvalue!(g, o);
      o = o.add(1);
    }
    let mut uv = (*l).openupval;
    while !uv.is_null() {
      LUAU_ASSERT!(upisopen!(uv));
      (*uv).markedopen = 1;
      markobject!(g, uv);
      uv = (*uv).u.open.threadnext;
    }
  }
}
