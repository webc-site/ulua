use crate::{
  macros::lua_m_freearray::luaM_freearray,
  records::{call_info::CallInfo, lua_state::LuaState},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l`、`l1` 须均为存活 `LuaState` 且共享同一 `global_State`：`l1` 为待释放栈的线程（通常正在关闭/回收），
/// 其 `base_ci`/`size_ci`、`stack`/`stacksize` 须自洽，`(*l1).hdr.memcat` 为分配时记账类别；
/// `luaM_freearray!` 以 `l` 分配器归还 `l1` 的这两段数组内存，释放后不得再访问 `l1` 的 ci/stack。非 GC 原子里并发调用。
/// cpp VM/src/lstate.cpp:52
pub(crate) unsafe fn freestack(l: *mut LuaState, l1: *mut LuaState) {
  unsafe {
    // cpp lstate.cpp:54-55：按线程自身 GC 头的 memcat 记账，而非 activememcat
    let memcat = (*l1).hdr.memcat;
    luaM_freearray!(l, (*l1).base_ci, (*l1).size_ci as usize, CallInfo, memcat);
    luaM_freearray!(l, (*l1).stack, (*l1).stacksize as usize, TValue, memcat);
  }
}
