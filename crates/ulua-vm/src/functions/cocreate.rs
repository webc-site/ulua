use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_newthread::lua_newthread, lua_xpush::lua_xpush,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checktype(l,1,FUNCTION)` 要求索引 1 为函数否则抛错回退；
/// `lua_newthread(l)` 新建并压入协程（需 `(*l).top` 后 ≥1 空槽、写入 `(*l).top`、分配可触发 GC）；
/// `lua_xpush(l,nl,1)` 要求 `l` 索引 1 的值存活，将其移入新线程 `nl` 栈。
/// cpp VM/src/lcorolib.cpp:332
pub unsafe fn cocreate(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Function as i32);

    let nl = lua_newthread(l);
    lua_xpush(l, nl, 1);

    1
  }
}

lua_lib_fn!(pub fn cocreate, cocreate_arm);
