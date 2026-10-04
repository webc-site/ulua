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
/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 接收者类型
/// 承载，wave-6d 降为安全 `pub(crate) fn`）：调用点须处于可 GC 的受保护帧、栈顶留槽；
/// 变参已由调用方经 `format_args!` 与格式一一对应（不存在运行期 fmt 解析）。窄
/// `unsafe` 块只因剩余被调 [lua_o_pushvfstring_ref] 仍收 `unsafe fn` 形（其自身契约
/// 即上文，直传无转手裸解引用）。cpp lobject.cpp:140。
pub(crate) fn lua_o_pushfstring(l: &mut LuaState, args: Arguments<'_>) {
  // SAFETY: 契约整体转授同前提的切片 ref 核心：`l` 存活独占由接收者引用承载，
  // 压栈点留槽且可 GC。
  unsafe { lua_o_pushvfstring_ref(l, args) };
}
