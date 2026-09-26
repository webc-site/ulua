use core::{ffi::c_char, fmt::Arguments};

use crate::{
  functions::{lapi_barrier::lua_c_threadbarrier_lapi, lua_o_pushvfstring::lua_o_pushvfstring},
  macros::lua_c_check_gc::lua_c_check_gc,
  records::lua_state::LuaState,
};

/// cpp `lua_pushvfstring`（`VM/src/lapi.cpp`）的 Rust 版。
///
/// 格式化参数由 [Arguments] 携带（cpp 中为 va_list），无需再传 fmt 串。
///
/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`：先经 `lua_c_check_gc` 与线程屏障
/// （可进入 GC），再由 `lua_o_pushvfstring` 把 `argp` 格式化压入栈顶，返回指针指向该串
/// 内部、下一次操作 `l` 前有效；`argp` 须在调用点已与格式定型（无运行期 va_list 解析）。
/// cpp lapi.cpp:762 `lua_pushvfstring`。
pub(crate) unsafe fn lua_pushvfstring(l: *mut LuaState, argp: Arguments<'_>) -> *const c_char {
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    lua_o_pushvfstring(l, argp)
  }
}
