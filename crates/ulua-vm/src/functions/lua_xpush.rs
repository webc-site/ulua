//! Source: `VM/src/lapi.cpp:206-212` (hand-ported)

use crate::{
  functions::{
    ensure_stack::ensure_stack_impl, index_2_addr::index_2_addr,
    lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{api_check::api_check, api_incr_top::api_incr_top, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// `lua_xpush` 核心（cpp `VM/src/lapi.cpp:224`）。调用序契约（正确性，非内存安
/// 全）：`from`/`to` 为同属一个 `global` 的两个存活线程（debug 断言；引用形天然
/// 排除 `from`/`to` 同一对象——cpp 亦无该用法）；`idx` 为 `from` 的合法（伪）索
/// 引；`ensure_stack_impl(to,from,1)` 扩 `to` 栈、`setobj2s` 写入并可能触发
/// GC/屏障，须在受保护帧内调用。
pub(crate) fn lua_xpush(from: &LuaState, to: &mut LuaState, idx: i32) {
  // SAFETY: `from`/`to` 存活（引用形保证）；index_2_addr 已对任意 idx 硬化；
  // read_ptr 只读转发（from 仅被读）契约成立；ensure_stack_impl/setobj_2_s 的
  // 指针前提由调用序契约与 VM 栈不变式成立。
  unsafe {
    api_check!(from.read_ptr(), from.global == to.global);
    lua_c_threadbarrier_lapi(to.as_mut_ptr());
    // cpp `ensure_stack_impl(to, from, 1)`
    ensure_stack_impl(to.as_mut_ptr(), from.read_ptr(), 1);
    let o = index_2_addr(from.read_ptr(), idx);
    setobj_2_s!(to.as_mut_ptr(), to.top, o);
    api_incr_top!(to.as_mut_ptr());
  }
}
