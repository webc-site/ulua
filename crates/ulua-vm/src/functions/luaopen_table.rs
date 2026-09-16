//! Node: `cxx:Function:Luau.VM:VM/src/ltablib.cpp:619:luaopen_table`
//! Source: `VM/src/ltablib.cpp:596-628` (hand-ported)

use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{
    foreach::foreach, foreachi::foreachi, getn::getn, lua_l_register::lua_l_register, maxn::maxn,
    tclear::tclear, tclone::tclone, tconcat::tconcat, tcreate::tcreate, tfind::tfind,
    tfreeze::tfreeze, tinsert::tinsert, tisfrozen::tisfrozen, tmove::tmove, tpack::tpack,
    tremove::tremove, tsort::tsort, tunpack::tunpack,
  },
  macros::{lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal},
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

struct TabFuncs([LuaLReg; 18]);
unsafe impl Sync for TabFuncs {}

static TAB_FUNCS: TabFuncs = TabFuncs([
  LuaLReg {
    name: c"concat".as_ptr(),
    func: Some(tconcat),
  },
  LuaLReg {
    name: c"foreach".as_ptr(),
    func: Some(foreach),
  },
  LuaLReg {
    name: c"foreachi".as_ptr(),
    func: Some(foreachi),
  },
  LuaLReg {
    name: c"getn".as_ptr(),
    func: Some(getn),
  },
  LuaLReg {
    name: c"maxn".as_ptr(),
    func: Some(maxn),
  },
  LuaLReg {
    name: c"insert".as_ptr(),
    func: Some(tinsert),
  },
  LuaLReg {
    name: c"remove".as_ptr(),
    func: Some(tremove),
  },
  LuaLReg {
    name: c"sort".as_ptr(),
    func: Some(tsort),
  },
  LuaLReg {
    name: c"pack".as_ptr(),
    func: Some(tpack),
  },
  LuaLReg {
    name: c"unpack".as_ptr(),
    func: Some(tunpack),
  },
  LuaLReg {
    name: c"move".as_ptr(),
    func: Some(tmove),
  },
  LuaLReg {
    name: c"create".as_ptr(),
    func: Some(tcreate),
  },
  LuaLReg {
    name: c"find".as_ptr(),
    func: Some(tfind),
  },
  LuaLReg {
    name: c"clear".as_ptr(),
    func: Some(tclear),
  },
  LuaLReg {
    name: c"freeze".as_ptr(),
    func: Some(tfreeze),
  },
  LuaLReg {
    name: c"isfrozen".as_ptr(),
    func: Some(tisfrozen),
  },
  LuaLReg {
    name: c"clone".as_ptr(),
    func: Some(tclone),
  },
  LuaLReg {
    name: null(),
    func: None,
  },
]);

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_table(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_register(l, c"table".as_ptr(), TAB_FUNCS.0.as_ptr());

    LUA_PUSHCFUNCTION(l, Some(tunpack), c"unpack".as_ptr());
    lua_setglobal(l, c"unpack".as_ptr());

    1
  }
}
