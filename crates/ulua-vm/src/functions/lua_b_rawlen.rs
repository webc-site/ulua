use crate::{
  enums::lua_type::LuaType, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位须为 table 或 string（`lua_type`+`arg_check` 校验，否则抛错），
/// `lua_objlen` 读该槽、`lua_pushinteger` 写回结果并可分配/GC。cpp/VM/src/lbaselib.cpp:185 luaB_rawlen。
pub unsafe fn lua_b_rawlen(l: *mut LuaState) -> i32 {
  unsafe {
    let tt = (*l).type_of(1);

    (*l).arg_check(
      matches!(tt, LuaType::Table | LuaType::String),
      1,
      "table or string expected",
    );

    let len = (*l).obj_len(1) as i32;
    (*l).push_integer(len);

    1
  }
}

lua_lib_fn!(pub fn lua_b_rawlen, lua_b_rawlen_arm);
