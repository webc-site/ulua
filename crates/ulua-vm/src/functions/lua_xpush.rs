//! Source: `VM/src/lapi.cpp:224-231` (hand-ported)

use crate::{
  functions::{
    ensure_stack::ensure_stack_impl, index_2_addr::index_2_addr,
    lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{api_check::api_check, api_incr_top::api_incr_top, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// `lua_xpush` 核心。调用序契约（正确性，非内存安全）：`from`/`to` 为同属一个
/// `global` 的两个存活线程（debug 断言；引用形天然排除 `from`/`to` 同一对象——cpp
/// 亦无该用法）；`idx` 为 `from` 的合法（伪）索引；`ensure_stack_impl(to,from,1)`
/// 扩 `to` 栈、扩不出来时在 `from` 上抛 "stack overflow"，故 `from` 必须是独占借用
/// （cpp `VM/include/lua.h:152` 的 `from` 本就非 const）；`setobj2s` 写入并可能触发
/// GC/屏障，须在受保护帧内调用。
pub(crate) fn lua_xpush(from: &mut LuaState, to: &mut LuaState, idx: i32) {
  // SAFETY: `from`/`to` 为两个不同存活线程的独占借用（引用形保证）；
  // `ensure_stack_impl` 只写 `to`，扩不出来时才在 `error_l`（=`from`）上抛错且随即
  // 发散；`index_2_addr` 形参为 `&LuaState`，此处由独占借用再借出只读窗；
  // `setobj_2_s`/`api_incr_top` 的指针前提由调用序契约与 VM 栈不变式成立。
  unsafe {
    api_check!(from, from.global == to.global);
    lua_c_threadbarrier_lapi(to.as_mut_ptr());
    // cpp `ensure_stack_impl(to, from, 1)`
    ensure_stack_impl(to, from, 1);
    let o = index_2_addr(from, idx);
    setobj_2_s!(to.as_mut_ptr(), to.top, o);
    api_incr_top!(to.as_mut_ptr());
  }
}
