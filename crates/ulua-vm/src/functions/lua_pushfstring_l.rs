use core::{ffi::c_char, fmt::Arguments};

use crate::{
  functions::{lapi_barrier::lua_c_threadbarrier_lapi, lua_o_pushvfstring::lua_o_pushvfstring_ref},
  macros::{lua_c_check_gc::lua_c_check_gc, svalue::svalue},
  records::lua_state::LuaState,
};

/// cpp `lua_pushfstringL` 形态的内部封装（`VM/src/lapi.cpp:770`，r12 T9 形）：
/// GC 检查 + 线程屏障前置序后，真实逻辑全在 [lua_o_pushvfstring_ref]，末尾做
/// 一次性折形——把核心压入栈顶的串字节折回 cpp `return svalue(L->top - 1)` 的
/// `const char*` 返回形。
///
/// r12 T9 裁决保留返回指针折形：消费面实测——`lua_l_checkoption` 需要把刚压栈的
/// `invalid option '%s'` 消息串直接以 `cstr_cow` 喂 `lua_l_argerror_l`（cpp
/// laux.cpp:34 同形链）；其余消费方（`lua_l_tolstring`/`auxresume`）只取压栈
/// 副作用。本折形是 fstring 族返回指针形的唯一登记点，其余转发包与核心的返回值
/// 已随零消费删净。
///
/// # Safety
/// `l` 须为正在执行的 C 函数帧的存活 `LuaState`（存活由接收者引用承载）且调用点
/// 处于可 GC 帧：`lua_c_check_gc` 可进入 GC step（挪动可达对象）；核心净压一层后
/// `top - 1` 为合法已写槽，返回指针指向该栈顶 TString 内部字节（TString 恒有
/// 终止 NUL，C 串语义保持），由该栈槽钉住存活、下一次操作 `l` 前有效（借出窗口
/// 止于该点）；`args` 须与调用点格式一一对应。cpp lapi.cpp:770。
pub(crate) unsafe fn lua_pushfstring_l(l: &mut LuaState, args: Arguments<'_>) -> *const c_char {
  unsafe {
    lua_c_check_gc!(l);
    // SAFETY: 前置序与收口前逐位一致；核心压栈后 top-1 槽即新串值，折形只做返回
    // 形状转换（存活帧 + 刚净压一层的槽合法性见上契约）
    lua_c_threadbarrier_lapi(l);
    lua_o_pushvfstring_ref(l, args);
    // 对应 cpp lapi.cpp:770 回返的 `luaO_pushvfstring` 结果（lobject.cpp:136
    // `return svalue(L->top - 1)`）；栈顶槽地址经 `top_slot(-1)` 读数原语取得
    svalue!(l.top_slot(-1))
  }
}
