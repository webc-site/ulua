use core::ffi::c_int;

use ulua_vm::type_aliases::lua_state::lua_State;

/// cpp `ReplRequirer.h:17` 的 `using Coverage = void (*)(lua_State*, int)`：
/// 记录模块函数用于覆盖率 / 计数器采集。
pub type Coverage = fn(*mut lua_State, c_int);
