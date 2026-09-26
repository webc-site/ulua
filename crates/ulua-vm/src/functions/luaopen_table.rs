//! Source: `VM/src/ltablib.cpp:596-628` (hand-ported)

use crate::{
  functions::{
    foreach::foreach, foreachi::foreachi, getn::getn, lua_l_register::lua_l_register, maxn::maxn,
    tclear::tclear, tclone::tclone, tconcat::tconcat, tcreate::tcreate, tfind::tfind,
    tfreeze::tfreeze, tinsert::tinsert, tisfrozen::tisfrozen, tmove::tmove, tpack::tpack,
    tremove::tremove, tsort::tsort, tunpack::tunpack,
  },
  macros::{lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal},
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static TAB_FUNCS: [LuaLReg; 17] = [
  LuaLReg::new(b"concat", tconcat),
  LuaLReg::new(b"foreach", foreach),
  LuaLReg::new(b"foreachi", foreachi),
  LuaLReg::new(b"getn", getn),
  LuaLReg::new(b"maxn", maxn),
  LuaLReg::new(b"insert", tinsert),
  LuaLReg::new(b"remove", tremove),
  LuaLReg::new(b"sort", tsort),
  LuaLReg::new(b"pack", tpack),
  LuaLReg::new(b"unpack", tunpack),
  LuaLReg::new(b"move", tmove),
  LuaLReg::new(b"create", tcreate),
  LuaLReg::new(b"find", tfind),
  LuaLReg::new(b"clear", tclear),
  LuaLReg::new(b"freeze", tfreeze),
  LuaLReg::new(b"isfrozen", tisfrozen),
  LuaLReg::new(b"clone", tclone),
];

/// # Safety
/// `l` 须为存活 LuaState 且栈顶之上留足空槽（`lua_l_register` push 库表；`LUA_PUSHCFUNCTION` push cfunction 后
/// `lua_setglobal` 消费之），须在可分配/GC 的受保护帧内调用。
/// cpp/VM/src/ltablib.cpp:694 luaopen_table。
pub unsafe extern "C-unwind" fn luaopen_table(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register(l, c"table".as_ptr(), &TAB_FUNCS);

    LUA_PUSHCFUNCTION(l, Some(tunpack), c"unpack".as_ptr());
    lua_setglobal(l, c"unpack".as_ptr());

    1
  }
}
