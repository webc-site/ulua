//! Source: `VM/src/lapi.cpp:1942-1957`

use crate::{
  functions::{ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_utag_limit::LUA_UTAG_LIMIT,
    sethvalue::sethvalue, setnilvalue::setnilvalue,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_getuserdatametatable(l: *mut LuaState, tag: i32) {
  // Safety: 契约保证 `l` 的 global 存活且 tag 落在 udatametatable 注册数组界内，取回的表指针为登记原值
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let h = (*(*l).global).udatamt[tag as usize];
    if !h.is_null() {
      sethvalue!(l, (*l).top, h);
    } else {
      setnilvalue!((*l).top);
    }

    api_incr_top!(l);
  }
}
