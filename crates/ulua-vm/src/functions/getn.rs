use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_objlen::lua_objlen, lua_pushinteger::lua_pushinteger,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于本 C 函数受保护帧：栈 1 号位为 table（`lua_l_checktype` 校验、非表即抛错），
/// `lua_objlen`/`lua_pushinteger` 读取该栈槽并可触发 GC/分配。cpp/VM/src/ltablib.cpp:83 getn。
pub unsafe extern "C-unwind" fn getn(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);

    lua_pushinteger(l, lua_objlen(l, 1));

    1
  }
}
