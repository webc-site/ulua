use core::{ffi::c_char, fmt::Arguments};

use crate::{
  functions::{lapi_barrier::lua_c_threadbarrier_lapi, lua_o_pushvfstring::lua_o_pushvfstring},
  macros::lua_c_check_gc::lua_c_check_gc,
  records::lua_state::LuaState,
};

/// cpp `lua_pushfstringL` 形态的内部封装：GC 检查 + 线程屏障后格式化并压栈。
///
/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`：先经 `lua_c_check_gc` 与线程屏障
/// （可进入 GC 挪动可达对象），再由 `lua_o_pushvfstring` 把格式化结果压入栈顶，返回
/// 指针指向该串内部、下一次操作 `l` 前有效；`args` 须与调用点格式一一对应。
/// cpp lapi.cpp:770 `lua_pushfstringL`。
pub(crate) unsafe fn lua_pushfstring_l(l: *mut LuaState, args: Arguments<'_>) -> *const c_char {
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    lua_o_pushvfstring(l, args)
  }
}
