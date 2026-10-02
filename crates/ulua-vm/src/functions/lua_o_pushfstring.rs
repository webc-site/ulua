use core::fmt::Arguments;

use crate::{functions::lua_o_pushvfstring::lua_o_pushvfstring_ref, records::lua_state::LuaState};

/// cpp `luaO_pushfstring`（`VM/src/lobject.cpp:140-149`）的转发包（r12 T9 形）：
/// 真实逻辑全在 [lua_o_pushvfstring_ref]，本层仅保留 cpp 调用点具名的入口形
/// （变参由调用方 `format_args!` 直接封装为 [Arguments]，无 fmt 串与 va_list
/// 包装层）。
///
/// r12 T9 裁决保留转发包、删返回指针形：消费面实测三处消费方
/// （`pusherror`/`lua_l_where`/`ensure_stack`）均只取压栈副作用，返回值零消费
/// （指针折形唯一需求点已登记于 `lua_pushfstring_l` 裁决注记）；`l` 收 `&mut`
/// 接收者形直转核心，无裸构造。
///
/// # Safety
/// 同 [lua_o_pushvfstring_ref]：`l` 须为正在执行的 C 函数帧的存活 `LuaState` 且
/// 调用点处于可 GC 的受保护帧、栈顶留槽；变参已由调用方经 `format_args!` 与格式
/// 一一对应（不存在运行期 fmt 解析）。cpp lobject.cpp:140。
pub(crate) unsafe fn lua_o_pushfstring(l: &mut LuaState, args: Arguments<'_>) {
  // SAFETY: 契约整体转授同前提的切片 ref 核心
  unsafe { lua_o_pushvfstring_ref(l, args) };
}
