//! Source: `VM/src/lapi.cpp:272-280` (hand-ported)

use core::ptr::{copy, eq};

use crate::{
  functions::{index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_insert(l: *mut LuaState, idx: i32) {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    let p: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(p, LUA_O_NILOBJECT));

    let count = (*l).top.offset_from(p);
    if count > 1 {
      let val = *(*l).top.sub(1);
      copy(p, p.add(1), (count - 1) as usize);
      *p = val;
    }
  }
}
