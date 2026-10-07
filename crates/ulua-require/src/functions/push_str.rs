//! Lua 串压栈门面：按 C 串语义（首个 NUL 截断）把字节串推入 VM 栈，
//! 对应 cpp `lua_pushstring(s.c_str())` / `lua_getfield(…, key.c_str())`。
//! 全部为安全函数：字节经 ptr+len 口拷入 VM 字符串存储后即与入参解耦。

use alloc::vec::Vec;

use ulua_vm::{functions::lua_pushlstring::lua_pushlstring_bytes, records::lua_state::LuaState};

/// 对应 cpp `std::string::c_str()` 的字节视图：按首个 NUL 截断。
/// 返回切片保证不含 NUL（Lua 字符串是字节串，非 UTF-8）。
fn c_str_prefix(s: &[u8]) -> &[u8] {
  let end = memchr::memchr(0, s).unwrap_or(s.len());
  &s[..end]
}

/// 键压栈归一形态：原样推送 / ASCII 小写归一后推送（注册与查表两侧共用，
/// 保证键一致）。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum KeyForm {
  /// 原样推送（按首个 NUL 截断）
  Raw,
  /// ASCII 小写归一后推送
  Lowered,
}

/// 按 C 串语义（首个 NUL 截断）把 VM 串拷贝为本地字节串（cpp `std::string`
/// 拷贝形态：读出即与 VM 栈解耦，可在任意后续栈操作中安全使用）。
pub(crate) fn c_str_prefix_owned(s: &[u8]) -> Vec<u8> {
  c_str_prefix(s).to_vec()
}

/// 对应 cpp `lua_pushstring(L, s.c_str())` 的零拷贝等价实现：按首个 NUL 截断后
/// 经 `lua_pushlstring_bytes`（切片 ref 核心，r12-w6d 已降为安全 `fn`）直推（cpp
/// `lua_pushstring` 内部同为 strlen + pushlstring），免去 NUL 补齐与堆分配。
///
/// `l` 为存活 `LuaState` 的独占借用（引用即存活证明），故本函数对调用方安全。
pub(crate) fn push_c_str(l: &mut LuaState, s: &[u8]) {
  // 切片经 c_str_prefix 收敛为无 NUL 字节视图，lua_pushlstring_bytes 把字节拷入
  // VM 字符串存储后不再引用该切片。
  lua_pushlstring_bytes(l, c_str_prefix(s));
}

/// cpp 的模块注册键归一：`std::tolower` 逐字节小写后 `lua_pushstring(c_str())`。
/// 无大写时零分配直推，有则转小写；与 `check_registered_modules` 共用同一形态，
/// 保证注册与查表两侧的键一致。
pub(crate) fn push_lowered_c_str(l: &mut LuaState, s: &[u8]) {
  let s = c_str_prefix(s);
  // 仅存在大写字母时才物化小写副本，否则直推原切片（两分支字节数一致）
  let lowered = s
    .iter()
    .any(u8::is_ascii_uppercase)
    .then(|| s.to_ascii_lowercase());
  push_c_str(l, lowered.as_deref().unwrap_or(s));
}

/// 按归一形态压键（`registry_table::cache_hit` 的两条查表路共用）。
pub(crate) fn push_key(l: &mut LuaState, key: &[u8], form: KeyForm) {
  match form {
    KeyForm::Raw => push_c_str(l, key),
    KeyForm::Lowered => push_lowered_c_str(l, key),
  }
}
