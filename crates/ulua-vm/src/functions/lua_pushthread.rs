use crate::{
  functions::{ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{api_incr_top::api_incr_top, setthvalue::setthvalue},
  records::lua_state::LuaState,
};

/// 把线程自身压栈（`lua_pushthread`）。`l` 以引用传入（存活由类型保证）；GC 线程屏障、
/// 扩容与 top 槽写入均为 push 族既定原语（`ensure_stack` 可触发 GC），返回是否主线程。
pub fn lua_pushthread(l: &mut LuaState) -> i32 {
  // SAFETY: `l` 存活（引用形保证）；`ensure_stack(l,1)` 保证 top 槽在分配栈界内可写，
  // `setthvalue!`/`api_incr_top!` 的其余前提由该扩容与栈不变量成立。
  unsafe {
    lua_c_threadbarrier_lapi(l.as_mut_ptr());

    ensure_stack(l.as_mut_ptr(), 1);
    setthvalue!(l, l.top, l.as_mut_ptr());
    api_incr_top!(l);
    ((*(*l).global).mainthread == l.as_mut_ptr()) as i32
  }
}
