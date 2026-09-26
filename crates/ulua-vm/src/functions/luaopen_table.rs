//! Source: `VM/src/ltablib.cpp:596-628` (hand-ported)

use crate::{
  functions::{
    foreach::foreach_arm, foreachi::foreachi_arm, getn::getn_arm, lua_l_register::lua_l_register,
    maxn::maxn_arm, tclear::tclear_arm, tclone::tclone_arm, tconcat::tconcat_arm,
    tcreate::tcreate_arm, tfind::tfind_arm, tfreeze::tfreeze_arm, tinsert::tinsert_arm,
    tisfrozen::tisfrozen_arm, tmove::tmove_arm, tpack::tpack_arm, tremove::tremove_arm,
    tsort::tsort_arm, tunpack::tunpack_arm,
  },
  macros::{lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal},
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static TAB_FUNCS: [LuaLReg; 17] = [
  LuaLReg::new(b"concat", tconcat_arm),
  LuaLReg::new(b"foreach", foreach_arm),
  LuaLReg::new(b"foreachi", foreachi_arm),
  LuaLReg::new(b"getn", getn_arm),
  LuaLReg::new(b"maxn", maxn_arm),
  LuaLReg::new(b"insert", tinsert_arm),
  LuaLReg::new(b"remove", tremove_arm),
  LuaLReg::new(b"sort", tsort_arm),
  LuaLReg::new(b"pack", tpack_arm),
  LuaLReg::new(b"unpack", tunpack_arm),
  LuaLReg::new(b"move", tmove_arm),
  LuaLReg::new(b"create", tcreate_arm),
  LuaLReg::new(b"find", tfind_arm),
  LuaLReg::new(b"clear", tclear_arm),
  LuaLReg::new(b"freeze", tfreeze_arm),
  LuaLReg::new(b"isfrozen", tisfrozen_arm),
  LuaLReg::new(b"clone", tclone_arm),
];

/// # Safety
/// `l` 须为存活 LuaState 且栈顶之上留足空槽（`lua_l_register` push 库表；`LUA_PUSHCFUNCTION` push cfunction 后
/// `lua_setglobal` 消费之），须在可分配/GC 的受保护帧内调用。
/// cpp/VM/src/ltablib.cpp:694 luaopen_table。
pub unsafe extern "C-unwind" fn luaopen_table(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register(l, c"table".as_ptr(), &TAB_FUNCS);

    LUA_PUSHCFUNCTION(l, Some(tunpack_arm), c"unpack".as_ptr());
    lua_setglobal(l, c"unpack".as_ptr());

    1
  }
}
