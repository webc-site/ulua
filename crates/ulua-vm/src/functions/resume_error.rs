use core::mem::size_of;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_growstack::lua_d_growstack,
  macros::{lua_s_new::lua_s_new, setsvalue::setsvalue},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// 恢复失败时统一压出错消息并回 `ERRRUN`（cpp `resume_error`）。
///
/// # Safety
///
/// `l` 为调用方传入的裸协程状态指针且函数体直接解引用它：`rewind_top`/`top` 回退、
/// `lua_s_new` 建串、`stack_last` 顶界现读，末尾 `lua_d_growstack` 可触发 GC 与栈重分配
/// 搬移 `top`/栈基——故本函数保留 `unsafe`。`l` 须为正在恢复的存活协程状态；`narg` 不得
/// 超过当前栈内实参个数（与 `resume_start` 的 `api_check!` 同一前提）。
/// `msg` 为安全 `&[u8]` 错误消息字节切片（§10 C 串消灭：原 `*const c_char`+`cstr_bytes`
/// 收口为字节切片形，须覆盖到 `lua_s_new` 建串调用为止）。
pub(crate) unsafe fn resume_error(l: *mut LuaState, msg: &[u8], narg: i32) -> i32 {
  // SAFETY: 契约保证 `l` 为正在 resume 的协程存活状态、narg 不超过当前栈内实参数，top 回退不越过 base
  unsafe {
    // l->top -= narg;
    // r12-w7a2 收编（同形单点·resume 错误族）：裸场域回落经 rewind_top 原语，
    // 与被替代式同址同宽；cpp 同形 `L->top -= narg;`
    (*l).rewind_top(narg as usize);

    // setsvalue(l, l->top, lua_s_new(l, msg));
    // setsvalue! 宏接收 TValue 指针
    // (*l).top is a StkId (which is a *mut TValue).
    // msg 已是错误消息字节切片，直接交 lua_s_new 建串（原 cstr_bytes 的 strlen 收口下沉到调用方的 `&[u8]` 切片形）
    setsvalue!(l, (*l).top, lua_s_new(l, msg));

    // incr_top(l) expands to: { luaD_checkstack(l, 1); l->top++; }
    // We manually perform the incr_top logic here to match the C++ source.

    // stacklimitreached check (simplified for the error-prone macro environment)
    let stack_last = (*l).stack_last as *mut u8;
    let top = (*l).top as *mut u8;
    let limit_reached = (stack_last as usize).wrapping_sub(top as usize) <= size_of::<TValue>();

    if limit_reached {
      lua_d_growstack(l, 1);
    }

    // 收编：growstack 恢复点后的裸场域抬顶经 advance_top 原语（现读场，同址同宽；
    // 手动 limit 检查时序保持原样，非本票面）
    (*l).advance_top(1);

    LuaStatus::ErrRun as i32
  }
}
