//! 本文件对应 `ulua_lua_l_argerror_l` 导出符号（源：ulua-vm/src/functions/lua_l_argerror_l.rs）。
//! 导出壳把 `extramsg` 由 `*const c_char` 经 `crate::cstr` 辅助适配为 `&str` 后逐参数透传，零业务逻辑。
use core::ffi::{c_char, c_int};

use ulua_vm::{functions::lua_l_argerror_l, records::lua_state::LuaState};

use crate::cstr::to_str_or_empty;

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_l_argerror_l`），除把 `extramsg` 经 `crate::cstr` 辅助
/// 重建为 `&str` 外，仅逐参数透传至 `lua_l_argerror_l::lua_l_argerror_l(l, narg, extramsg)`，
/// 除该重建外本帧不解引用其它指针。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；
/// - `extramsg`（`*const c_char`）：以 NUL 结尾的 C 字符串，整个调用期间存活（oracle `lualib.h:46` 的 `const char*`）——`crate::cstr` 重建前提；
/// - 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；
/// - 其余安全前置条件与被调函数的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_l_argerror_l")]
pub unsafe extern "C-unwind" fn lua_l_argerror_l(
  l: *mut LuaState,
  narg: c_int,
  extramsg: *const c_char,
) -> ! {
  // Safety: 契约声明 `extramsg` 为 NUL 结尾的存活 C 字符串（C API 约定），
  // `crate::cstr::to_str_or_empty` 的 `# Safety` 前提成立；非 UTF-8 字节兜底为空串，
  // 错误消息退化、不改变报错路径。
  let extramsg = unsafe { to_str_or_empty(extramsg) };
  // Safety: `l` 的前提见上方契约；`extramsg` 已在本帧重建为合法 `&str`（本次调用期内存活），
  // 本行仅按声明顺序透传、不再解引用任何原始指针，与被调函数 `# Safety` 契约一致。
  unsafe { lua_l_argerror_l::lua_l_argerror_l(l, narg, extramsg) }
}
