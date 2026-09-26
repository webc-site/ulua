use ulua_vm::records::lua_state::LuaState;

use crate::functions::{
  c_str_prefix::push_lowered_c_str, cache_table_keys::REGISTERED_CACHE_TABLE_KEY,
  registry_table::cache_hit,
};

/// # Safety
/// `l` 必须指向存活的 `LuaState`；函数在 Lua 栈上压入/弹出临时值。
/// 命中缓存时返回 true，且命中值留在栈顶（cpp 返回 1 时同样留在栈顶）。
pub(crate) unsafe fn check_registered_modules(l: *mut LuaState, path: &[u8]) -> bool {
  // Safety: l 由 lua_requireinternal 的 require 链路传入，是该调用栈帧内存活的
  // LuaState；push_lowered_c_str 按其契约把小写归一后的键压栈（净增一槽），
  // 整段查表收口在 registry_table::cache_hit 门面（其内部只有 findtable 一步补
  // NUL、且仅闭包调用期内存活）；本帧不解引用任何裸指针。
  // 无大写时零分配直推；有则转小写后推入（cpp 同为 ASCII 字节小写归一查缓存）
  unsafe { cache_hit(l, REGISTERED_CACHE_TABLE_KEY, path, push_lowered_c_str) }
}
