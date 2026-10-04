use core::fmt::Arguments;

use crate::{
  functions::{lapi_barrier::lua_c_threadbarrier_lapi, lua_o_pushvfstring::lua_o_pushvfstring_ref},
  macros::lua_c_check_gc::lua_c_check_gc,
  records::lua_state::LuaState,
};

/// cpp `lua_pushvfstring`（`VM/src/lapi.cpp:762`）的转发包（r12 T9 形）：GC 检查 +
/// 线程屏障前置序保留，真实逻辑全在 [lua_o_pushvfstring_ref]。格式化参数由
/// [Arguments] 携带（cpp 中为 va_list），无需再传 fmt 串。
///
/// r12 T9 裁决保留转发包、删返回指针形：消费面实测唯一消费方 `lua_l_error_l`
/// 只取压栈副作用（随后与 `lua_l_where` 结果 `concat(2)` 合并抛错、不返回），
/// 返回值零消费（指针折形唯一需求点已登记于 `lua_pushfstring_l` 裁决注记）。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 接收者类型
/// 承载，wave-6d 降为安全 `pub(crate) fn`）：`l` 须为正在执行的 C 函数帧的存活
/// `LuaState` 且调用点处于可 GC 帧：`lua_c_check_gc` 可进入 GC step（挪动可达对象），
/// 核心净压一层（分配、可触发 GC），结果串借出窗口由栈槽存活钉住；`argp` 须在调用点
/// 已与格式定型（无运行期 va_list 解析）。窄 `unsafe` 块内剩余裸操作契约：`luaC_needsGC`
/// 对 `(*l).global` 的裸读由 LuaState 不变量保护；`lua_c_step`/`lua_c_threadbarrier_lapi`
/// 收 `*mut` 形参，指针 `as` 重建自独占借用、借用窗止于当句（r16-v21 判例）；
/// `lua_o_pushvfstring_ref` 同前提转授。cpp lapi.cpp:762。
pub(crate) fn lua_pushvfstring(l: &mut LuaState, argp: Arguments<'_>) {
  // SAFETY: 契约保证 `l` 为存活调用帧且处于可 GC 帧，前置序（check_gc→barrier→压栈）
  // 与收口前逐位一致；三处裸操作/转调核心的前提均在上方契约内整体成立。
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    lua_o_pushvfstring_ref(l, argp);
  }
}
