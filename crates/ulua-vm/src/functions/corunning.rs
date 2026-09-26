use crate::{
  functions::{lua_pushnil::lua_pushnil, lua_pushthread::lua_pushthread},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState，栈顶之上至少留 1 个空槽（`lua_pushthread`/`lua_pushnil` 各占一槽作返回值），
/// 且处于可分配/GC 的受保护帧。cpp/VM/src/lcorolib.cpp:354 corunning。
pub(crate) unsafe extern "C-unwind" fn corunning(l: *mut LuaState) -> i32 {
  unsafe {
    if lua_pushthread(l) != 0 {
      lua_pushnil(l); // main thread is not a coroutine
    }
    1
  }
}
