use alloc::string::String;

use ulua_vm::records::lua_state::LuaState;

/// 读取全局 `capturedoutput`（对齐 cpp `getCapturedOutput`，
/// ReplFixture/ReplWithPathFixture 公共实现）。
///
/// 全程经 `LuaState` 安全栈 API：读全局、拷出字节、弹栈，无裸指针与 unsafe。
pub(crate) fn captured_output(l: &mut LuaState) -> String {
  l.get_global_bytes(b"capturedoutput");
  // 借用栈槽字节在被弹出前拷贝为 String
  let result = l
    .to_bytes(-1)
    .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
    .unwrap_or_default();
  l.pop(1);
  result
}
