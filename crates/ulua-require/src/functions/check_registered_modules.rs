use ulua_vm::records::lua_state::LuaState;

use crate::functions::{
  cache_table_keys::REGISTERED_CACHE_TABLE_KEY, push_str::KeyForm, registry_table::cache_hit,
};

/// cpp `checkRegisteredModules`：查显式注册模块缓存表（键 ASCII 小写归一）。
/// 命中缓存时返回 true，且命中值留在栈顶（cpp 返回 1 时同样留在栈顶）。
pub(crate) fn check_registered_modules(l: &mut LuaState, path: &[u8]) -> bool {
  // 无大写时零分配直推；有则转小写后推入（cpp 同为 ASCII 字节小写归一查缓存）
  cache_hit(l, REGISTERED_CACHE_TABLE_KEY, path, KeyForm::Lowered)
}
