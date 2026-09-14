//! `lua_setuserdatametatable` — pop a table from the stack and register it as
//! the metatable for userdata of type `tag`.
//! C++ source: `VM/src/lapi.cpp:1616`

use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_utag_limit::LUA_UTAG_LIMIT,
  },
  records::{lua_state::lua_State, lua_table::LuaTable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_setuserdatametatable")]
pub unsafe fn lua_setuserdatametatable(l: *mut lua_State, tag: c_int) {
  unsafe {
    api_checknelems!(l, 1);
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    // reassignment not supported
    api_check!(l, (*(*l).global).udatamt[tag as usize].is_null());

    let t = (*l).top.offset(-1);
    api_check!(l, (*t).tt == LuaType::Table as c_int);

    // hvalue(top-1): the gc pointer refers to a GcObject; its `h` union arm is a LuaTable.
    let gco = (*t).value.gc;
    let h: *mut LuaTable = core::ptr::addr_of_mut!((*gco).h) as *mut LuaTable;
    (*(*l).global).udatamt[tag as usize] = h;

    (*l).top = (*l).top.offset(-1);
  }
}
