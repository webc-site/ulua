//! Source: `VM/src/lfunc.cpp` (lfunc.cpp:156-169, hand-ported)

use core::ptr::addr_of_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_c_upvalclosed::lua_c_upvalclosed,
  macros::setobj::setobj,
  records::{lua_state::LuaState, up_val::UpVal},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`uv` 须为处于 open 双向链表中的存活 `UpVal`——`(*uv).u.open.next/prev`
/// 非空且互为回指（`LUAU_ASSERT` 校验），本函数先摘链再关闭；`dead==false` 时把值搬进 `(*uv).u.value`
/// 并 `luaC_upvalclosed`（可触发屏障），须保证 `(*uv).v` 指向的栈槽仍存活。cpp `lfunc.cpp:156`。
pub unsafe fn lua_f_closeupval(l: *mut LuaState, uv: *mut UpVal, dead: bool) {
  unsafe {
    // unlink value from all lists *before* closing it since value storage overlaps
    let next = (*uv).u.open.next;
    let prev = (*uv).u.open.prev;
    LUAU_ASSERT!((*next).u.open.prev == uv && (*prev).u.open.next == uv);
    (*next).u.open.prev = prev;
    (*prev).u.open.next = next;

    if dead {
      return;
    }

    let value = addr_of_mut!((*uv).u.value);
    setobj!(l, value, (*uv).v);
    (*uv).v = value;
    lua_c_upvalclosed(l, uv);
  }
}
