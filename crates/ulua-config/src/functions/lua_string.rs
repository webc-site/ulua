use alloc::string::String;
use core::ptr::null_mut;

use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{functions::lua_tolstring::lua_tolstring, records::lua_state::LuaState};

/// 读取栈上字符串（对应 C++ `lua_tostring`），共享辅助。
///
/// # Safety
/// `l` 必须是有效 VM 状态，`index` 处需为字符串或可转换的数字。
pub(crate) unsafe fn lua_string(l: *mut LuaState, index: i32) -> String {
  // Safety: 调用方保证 l 有效且 index 处可转换为字符串。C 接口
  // `lua_tolstring(L, idx, size_t*)` 的 size 出参传 null 以取 NUL 结尾视图，与 cpp
  // oracle `std::string{lua_tostring(...)}`（strlen 语义）逐字对齐；返回的 C 串指针交
  // `cstr_cow` 收口判空与 lossy 解码（null 译成空串，非空时其内容仅在本次读取内有效，
  // 随即 `into_owned` 落入 String 不外泄）。已知偏差：非 UTF-8 别名字节经 lossy 折叠为
  // U+FFFD，理论上可与不同原始字节碰撞为同一键；根治方案是键类型改为 Vec<u8>（字节域），
  // 划入后续轮次。
  unsafe { cstr_cow(lua_tolstring(l, index, null_mut())) }.into_owned()
}
