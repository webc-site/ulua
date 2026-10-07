use alloc::string::String;

use ulua_vm::records::lua_state::LuaState;

/// 读取栈上字符串（对应 C++ `lua_tostring`），共享辅助。
///
/// 经 `LuaState::to_bytes` 安全视图读取，无 null 出参：C 接口
/// `lua_tolstring(L, idx, size_t*)` 的 size 出参传 null 只为取 NUL 结尾视图，
/// 这里等价地按首个 NUL 截断（cpp oracle `std::string{lua_tostring(...)}` 的
/// strlen 语义）；不可转换类型的 null 结果译成空串。
///
/// 已知偏差：非 UTF-8 别名字节经 lossy 折叠为 U+FFFD，理论上可与不同原始字节
/// 碰撞为同一键；根治方案是键类型改为 `Vec<u8>`（字节域），划入后续轮次。
pub(crate) fn lua_string(l: &mut LuaState, index: i32) -> String {
  let bytes = l.to_bytes(index).unwrap_or_default();
  let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
  String::from_utf8_lossy(&bytes[..len]).into_owned()
}
