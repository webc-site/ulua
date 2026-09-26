//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_l_argerror_l.rs）。
//! 导出壳与 ulua-vm 对应函数签名一致，除把 `extramsg` 由 `*const c_char` 适配为 `&str` 后逐参数透传外，零业务逻辑。
use core::ffi::{CStr, c_char, c_int};

use ulua_vm::{functions::lua_l_argerror_l, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_l_argerror_l`），仅逐参数透传至 `lua_l_argerror_l::lua_l_argerror_l(l, narg, extramsg)`，零逻辑，本帧不解引用任何指针。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；
/// - `extramsg`（`*const c_char`）：以 NUL 结尾的 C 字符串，整个调用期间存活（oracle `lualib.h:46` 的 `const char*`）；
/// - 其余参数均为值类型（栈索引/标量），其合法性按 Lua/C API 约定由调用方给出，不引入额外内存前提；
/// - 其余安全前置条件与被调函数的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_l_argerror_l")]
pub unsafe extern "C-unwind" fn lua_l_argerror_l(
  l: *mut LuaState,
  narg: c_int,
  extramsg: *const c_char,
) -> ! {
  // C 字符串转 &str：extramsg 仅拼接进错误消息，非 UTF-8 字节兜底为空串（错误消息退化，不改变报错路径）。
  // Safety: 契约声明 `extramsg` 为 NUL 结尾的存活 C 字符串（C API 约定），`from_ptr` 前提成立。
  let extramsg = unsafe { CStr::from_ptr(extramsg) };
  let extramsg = extramsg.to_str().unwrap_or("");
  // Safety: `l` 的前提见上方契约；`extramsg` 已在本帧由 `from_ptr` 构造为合法 `&str`（本次调用期内存活），
  // 本行仅按声明顺序透传、不再解引用任何原始指针，与被调函数 `# Safety` 契约一致。
  unsafe { lua_l_argerror_l::lua_l_argerror_l(l, narg, extramsg) }
}
