use crate::{
  functions::lua_d_reallocstack::lua_d_reallocstack, macros::getgrownstacksize::getgrownstacksize,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须存活且栈已初始化（`stacksize` 有效）；`n` 为需要新增的槽数，且已由调用方按 LUA_MAXSTACK
/// 上界过滤（否则 `getgrownstacksize` 加倍/求和会整型回绕）。`lua_d_reallocstack` 可抛 ERR_MEM unwind，
/// 调用后旧栈指针全部失效。cpp ldo.cpp:222。
pub unsafe fn lua_d_growstack(l: *mut LuaState, n: i32) {
  // Safety: 契约保证 `l` 存活且 n 已被调用方以 MAXSTACK 上界过滤，重排栈按 grownstacksize 分配并重建全部栈指针
  unsafe {
    lua_d_reallocstack(l, getgrownstacksize(l, n), 0);
  }
}
