use crate::{
  functions::{ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{api_incr_top::api_incr_top, setthvalue::setthvalue},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pushthread(l: *mut LuaState) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);

    ensure_stack(l, 1);
    setthvalue!(l, (*l).top, l);
    api_incr_top!(l);
    ((*(*l).global).mainthread == l) as i32
  }
}
