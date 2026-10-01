use crate::{
  enums::lua_type::LuaType, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于本 C 函数受保护帧：栈 1 号位为 table（`lua_l_checktype` 校验、非表即抛错），
/// `lua_objlen`/`lua_pushinteger` 读取该栈槽并可触发 GC/分配。cpp/VM/src/ltablib.cpp:83 getn。
pub unsafe fn getn(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);

    (*l).push_integer((*l).obj_len(1) as i32);

    1
  }
}

lua_lib_fn!(pub fn getn, getn_arm);
