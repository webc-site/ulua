use core::{ffi::c_char, fmt::Arguments};

use crate::{functions::lua_o_pushvfstring::lua_o_pushvfstring, records::lua_state::LuaState};

/// cpp `luaO_pushfstring`（`VM/src/lobject.cpp:140-149`）的 Rust 版。
///
/// 变参在 Rust 侧由调用方的 `format_args!` 直接封装为 [Arguments]，
/// 不再需要 fmt 串与 va_list 的包装层。
///
/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`：变参已由调用方经 `format_args!` 与
/// 格式一一对应（不存在运行期 fmt 解析），结果经 `lua_o_pushvfstring` 串化后压入 `l` 栈顶
/// （分配、可触发 GC），返回指针指向该串内部、下一次操作 `l` 前有效。cpp lobject.cpp:140。
pub unsafe fn lua_o_pushfstring(l: *mut LuaState, args: Arguments<'_>) -> *const c_char {
  // Safety: 契约保证 l 存活且 args 已定型，透传给同契约的 lua_o_pushvfstring
  unsafe { lua_o_pushvfstring(l, args) }
}
