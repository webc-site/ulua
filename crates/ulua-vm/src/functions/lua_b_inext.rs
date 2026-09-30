use crate::{
  enums::lua_type::LuaType, functions::lua_rawgeti::lua_rawgeti, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 2 为可转整数（`lua_l_checkinteger`，作当前下标）、index 1 为
/// table（`luaL_checktype`），不符即抛错；`lua_rawgeti`/`push` 会读写栈，须在受保护帧内调用。
/// cpp `lbaselib.cpp:238`。
pub unsafe fn lua_b_inext(l: *mut LuaState) -> i32 {
  unsafe {
    let mut i = (*l).check_integer(2);
    (*l).check_type(1, LuaType::Table);
    i += 1; // next value
    (*l).push_integer(i);
    lua_rawgeti(l, 1, i);

    if (*l).is_nil(-1) { 0 } else { 2 }
  }
}

lua_lib_fn!(pub fn lua_b_inext, lua_b_inext_arm);
