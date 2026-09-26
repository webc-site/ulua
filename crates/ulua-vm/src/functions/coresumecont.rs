use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    auxresumecont::auxresumecont, coresumefinish::coresumefinish,
    interrupt_thread::interrupt_thread, lua_tothread::lua_tothread,
  },
  macros::lua_l_argexpected::luaL_argexpected,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于 resume 续体的受保护帧，栈 1 号位为 coroutine：`lua_tothread` 解析后
/// `luaL_argexpected` 先判 `co` 非空再解引用 `(*co).status`。转调 `interrupt_thread`/`auxresumecont`/
/// `coresumefinish` 可 GC/可抛错。cpp/VM/src/lcorolib.cpp:239 coresumecont。
pub(crate) unsafe extern "C-unwind" fn coresumecont(l: *mut LuaState, _status: i32) -> i32 {
  unsafe {
    let co = lua_tothread(l, 1);
    luaL_argexpected!(l, co.is_some(), 1, "thread");
    let co = co.expect("luaL_argexpected 已证 co 非空");

    // if coroutine still hasn't yielded after the break, break current thread again
    if (*co).status == LuaStatus::Break as u8 {
      return interrupt_thread(l, co);
    }

    let r = auxresumecont(l, co);
    coresumefinish(l, r)
  }
}
