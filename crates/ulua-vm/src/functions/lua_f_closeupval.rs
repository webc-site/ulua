//! Node: `cxx:Function:Luau.VM:VM/src/lfunc.cpp:156:lua_f_closeupval`
//! Source: `VM/src/lfunc.cpp` (lfunc.cpp:156-169, hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_c_upvalclosed::luaC_upvalclosed, macros::setobj::setobj, records::up_val::UpVal,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_f_closeupval(l: *mut lua_State, uv: *mut UpVal, dead: bool) {
  unsafe {
    // unlink value from all lists *before* closing it since value storage overlaps
    LUAU_ASSERT!((*(*uv).u.open.next).u.open.prev == uv && (*(*uv).u.open.prev).u.open.next == uv);
    (*(*uv).u.open.next).u.open.prev = (*uv).u.open.prev;
    (*(*uv).u.open.prev).u.open.next = (*uv).u.open.next;

    if dead {
      return;
    }

    let value = core::ptr::addr_of_mut!((*uv).u.value);
    setobj!(l, value, (*uv).v);
    (*uv).v = value;
    luaC_upvalclosed(l, uv);
  }
}

pub use lua_f_closeupval as luaF_closeupval;
