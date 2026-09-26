use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettop::lua_gettop, lua_l_checktype::lua_l_checktype,
    lua_pcallyieldable::lua_pcallyieldable, lua_pushvalue::lua_pushvalue, lua_replace::lua_replace,
  },
  macros::{lua_lib_fn::lua_lib_fn, lua_multret::LUA_MULTRET},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checktype(l,2,FUNCTION)` 要求索引 2 为错误处理函数否则抛错回退，
/// 索引 1 为被调函数、其后为实参；`lua_pushvalue`/`lua_replace` 交换 1、2 槽（各槽须存活）；随后
/// `lua_pcall`（errfunc=1、LUA_MULTRET 变长返回）可再入 Lua、抛错、扩栈与触发 GC，调用侧须承接展开。
/// cpp VM/src/lbaselib.cpp:344
pub(crate) unsafe fn lua_b_xpcally(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 2, LuaType::Function as i32);

    // swap function & error function
    lua_pushvalue(l, 1);
    lua_pushvalue(l, 2);
    lua_replace(l, 1);
    lua_replace(l, 2);
    // at this point the stack looks like err, f, args

    lua_pcallyieldable(l, lua_gettop(l) - 2, LUA_MULTRET, 1)
  }
}

lua_lib_fn!(pub(crate) fn lua_b_xpcally, lua_b_xpcally_arm);
