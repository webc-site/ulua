use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_checktype::lua_l_checktype, lua_rawset::lua_rawset,
    lua_settop::lua_settop,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 1 为 table、index 2/3 存在任意值（`checktype`/`checkany` 校验，
/// 不符即抛错）；`lua_rawset` 弹出两槽并可 GC，须在受保护帧内调用。cpp `lbaselib.cpp:175`。
pub unsafe extern "C-unwind" fn lua_b_rawset(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_l_checkany(l, 2);
    lua_l_checkany(l, 3);
    lua_settop(l, 3);
    lua_rawset(l, 1);
    1
  }
}
