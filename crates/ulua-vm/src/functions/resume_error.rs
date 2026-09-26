use core::{ffi::c_char, mem::size_of};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{cstr_bytes, lua_d_growstack::lua_d_growstack},
  macros::{lua_s_new::lua_s_new, setsvalue::setsvalue},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// 恢复失败时统一压出错消息并回 `ERRRUN`（cpp `resume_error`）。
///
/// # Safety
///
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `l` 须为正在恢复的协程状态；`msg` 须为有效终止 C 串指针；`narg` 不得超过
/// 当前栈内实参个数（与 `resume_start` 的 `api_check!` 同一前提）。
pub(crate) unsafe fn resume_error(l: *mut LuaState, msg: *const c_char, narg: i32) -> i32 {
  // Safety: 契约保证 `l` 为正在 resume 的协程存活状态、narg 不超过当前栈内实参数，top 回退不越过 base
  unsafe {
    // l->top -= narg;
    (*l).top = (*l).top.sub(narg as usize);

    // setsvalue(l, l->top, lua_s_new(l, msg));
    // setsvalue! 宏接收 TValue 指针
    // (*l).top is a StkId (which is a *mut TValue).
    // msg 为 NUL 结尾 C 串，经 cstr_bytes 扫首个 NUL 得字节切片（保持原 lua_s_new 的 strlen 语义）
    setsvalue!(l, (*l).top, lua_s_new(l, cstr_bytes(msg)));

    // incr_top(l) expands to: { luaD_checkstack(l, 1); l->top++; }
    // We manually perform the incr_top logic here to match the C++ source.

    // stacklimitreached check (simplified for the error-prone macro environment)
    let stack_last = (*l).stack_last as *mut u8;
    let top = (*l).top as *mut u8;
    let limit_reached = (stack_last as usize).wrapping_sub(top as usize) <= size_of::<TValue>();

    if limit_reached {
      lua_d_growstack(l, 1);
    }

    (*l).top = (*l).top.add(1);

    LuaStatus::ErrRun as i32
  }
}
