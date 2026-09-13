use core::ffi::c_void;

use crate::type_aliases::lua_state::lua_State;

/// C++ `Pfunc` 是普通函数指针，异常（lua_exception 模拟 longjmp）需要穿过它
/// unwind 到 `luaD_rawrunprotected` 的 catch 边界；`extern "C"` 会让 panic 直接
/// abort，故用 `extern "C-unwind"`（与 lua_callbacks.rs 中回调 ABI 先例一致）。
pub type Pfunc = Option<unsafe extern "C-unwind" fn(l: *mut lua_State, ud: *mut c_void)>;
