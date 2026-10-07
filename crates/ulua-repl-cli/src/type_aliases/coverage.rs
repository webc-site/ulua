use ulua_vm::records::lua_state::LuaState;

/// cpp `ReplRequirer.h:17` 的 `using Coverage = void (*)(LuaState*, int)`：
/// 记录模块函数用于覆盖率 / 计数器采集。Rust 侧只在内部链路调用（非 FFI 边界），
/// 形参用原生 `i32` 而非 `c_int`（review.md §7），状态句柄按 §2 收编为借用形。
pub type Coverage = fn(&mut LuaState, i32);
