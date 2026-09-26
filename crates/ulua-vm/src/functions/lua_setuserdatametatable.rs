//! `lua_setuserdatametatable` — pop a table from the stack and register it as
//! the metatable for userdata of type `tag`.
//! C++ source: `VM/src/lapi.cpp:1616`

use crate::{
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_utag_limit::LUA_UTAG_LIMIT,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setuserdatametatable(l: *mut LuaState, tag: i32) {
  unsafe {
    api_checknelems!(l, 1);
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    // reassignment not supported
    api_check!(l, (*(*l).global).udatamt[tag as usize].is_null());

    let t = (*l).top.offset(-1);
    let Some(h) = (*(*t).value.gc).as_table_mut() else {
      api_check!(l, false);
      return;
    };
    (*(*l).global).udatamt[tag as usize] = h as *mut LuaTable;

    // 弹栈：t 即上方已绑定的栈顶单槽，免二次 `(*l).top.offset(-1)` 裸重读
    (*l).top = t;
  }
}
