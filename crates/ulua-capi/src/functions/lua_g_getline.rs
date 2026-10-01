//! 本文件对应 `ulua_luaG_getline` 导出符号（源：ulua-vm/src/functions/lua_g_getline.rs）。
//! 导出壳与 ulua-vm 对应函数签名一致，除把 `p` 裸指针重建为 `&Proto` 后透传外，零业务逻辑。
use core::ffi::c_int;

use ulua_vm::{functions::lua_g_getline, records::proto::Proto};

/// # Safety
/// C ABI 导出壳（符号 `ulua_luaG_getline`），透传至 `lua_g_getline(p, pc)`。调用方须保证：
/// - `p`（`*mut Proto`）：非空、对齐，指向存活的 `Proto` 原型对象，非空、对齐，调用期间被所属状态/GC 持有不移动；
/// - `pc`：值型参数（指令下标），前置条件依被调函数 `# Safety` 契约（`0 <= pc < sizecode`）。
#[unsafe(export_name = "ulua_luaG_getline")]
pub unsafe extern "C-unwind" fn lua_g_getline(p: *mut Proto, pc: c_int) -> c_int {
  // Safety: 契约保证 p 非空对齐且调用期间存活，重建共享引用仅按 pc 只读取行号，不写 Proto
  lua_g_getline::lua_g_getline(unsafe { &*p }, pc)
}
