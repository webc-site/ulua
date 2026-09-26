use crate::{
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_e_newthread::lua_e_newthread,
  },
  macros::{api_incr_top::api_incr_top, lua_c_check_gc::lua_c_check_gc, setthvalue::setthvalue},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可分配/GC 的受保护帧，`ensure_stack(l, 1)` 保证 `(*l).top` 之后留 1 空槽
/// （`setthvalue` 写入并 `api_incr_top`）；`lua_e_newthread` 建新线程并由本线程 GC 接住，`userthread` 回调收创建通知。
/// 返回的新线程仅在 `l` 的 GC 保活期间有效。cpp/VM/src/lapi.cpp:233 lua_newthread。
pub unsafe fn lua_newthread(l: *mut LuaState) -> *mut LuaState {
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    // cpp `ensure_stack(L, 1)`：随后 setthvalue 直接写 L->top
    ensure_stack(l, 1);
    let l1 = lua_e_newthread(l);
    setthvalue!(l, (*l).top, l1);
    api_incr_top!(l);
    let g = (*l).global;
    if let Some(userthread) = (*g).cb.userthread {
      userthread(l, l1);
    }
    l1
  }
}
