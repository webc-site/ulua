use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

/// cpp `ReplRequirer.h:17` 的 `using Coverage = void (*)(LuaState*, int)`：
/// 记录模块函数用于覆盖率 / 计数器采集。
pub type Coverage = fn(*mut LuaState, c_int);
