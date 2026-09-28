use crate::{
  enums::lua_type::LuaType, functions::lua_rawset::lua_rawset, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 1 为 table、index 2/3 存在任意值（`checktype`/`checkany` 校验，
/// 不符即抛错）；`lua_rawset` 弹出两槽并可 GC，须在受保护帧内调用。cpp `lbaselib.cpp:175`。
pub unsafe fn lua_b_rawset(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);
    (*l).check_any(2);
    (*l).check_any(3);
    (*l).set_top(3);
    lua_rawset(l, 1);
    1
  }
}

lua_lib_fn!(pub fn lua_b_rawset, lua_b_rawset_arm);
