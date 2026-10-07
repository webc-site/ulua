//! 注册表缓存表的字节键栈操作门面：调用点一律只交出 `&[u8]` 键与
//! `&mut LuaState` 独占借用，表字段读写以「压键 + `gettable`/`settable`」的
//! 纯字节形态完成；子表定位走 ulua-vm 的字节口 `lua_l_findtable_bytes`
//! （无 NUL 约定）。裸指针只在向 vm C-API 借出 `l.as_mut_ptr()` 的一行内出现，
//! 由 `&mut LuaState` 的存活引用承载合法性，故本模块对调用方全部安全。

use ulua_vm::{
  functions::lua_l_findtable::lua_l_findtable_bytes, macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::LuaState,
};

use crate::functions::{
  push_str::{KeyForm, push_key},
  stack_index::to_absolute,
};

/// 子表预分配槽数提示（cpp `luaL_findtable(…, 1)` 的 szhint）。
const SUB_TABLE_HINT: i32 = 1;

/// 压入注册表下的命名子表（缺失则以 1 项预容量创建），对应 cpp
/// `lua_l_findtable(L, LUA_REGISTRYINDEX, tableKey, 1)`。
pub(crate) fn push_registry_table(l: &mut LuaState, table_key: &[u8]) {
  // Safety: `as_mut_ptr` 由 `&mut LuaState` 借出、调用期内独占存活；lua_l_findtable_bytes
  // 在本次调用内读完键内容（findtable 的失败返回指针不被消费，键串为本帧借用、
  // 调用期内可读），不外泄任何指针；净压一个表值。
  lua_l_findtable_bytes(l, LUA_REGISTRYINDEX, table_key, SUB_TABLE_HINT);
}

/// 取 `table_idx` 处表的字节键字段并压栈，对应 cpp `lua_getfield(L, idx, key.c_str())`。
///
/// 键经首 NUL 截断后走「压键 + `gettable`」：与 `lua_getfield` 内部的
/// strlen + 入 intern 表 + `gettable` 路径逐字节等价，且免掉临时 NUL 缓冲。
pub(crate) fn get_table_field(l: &mut LuaState, table_idx: i32, key: &[u8]) {
  let table_idx = to_absolute(table_idx, l.get_top());
  push_key(l, key, KeyForm::Raw);
  l.get_table(table_idx);
}

/// 用栈顶值写入 `table_idx` 处表的字节键字段（并消费该值），对应 cpp
/// `lua_setfield(L, idx, key.c_str())`。
pub(crate) fn set_table_field(l: &mut LuaState, table_idx: i32, key: &[u8]) {
  let table_idx = to_absolute(table_idx, l.get_top());
  push_key(l, key, KeyForm::Raw);
  l.insert(-2);
  l.set_table(table_idx);
}

/// 注册表缓存表的「字节键 → 命中查询」整段收口（cpp `isCached` /
/// `checkRegisteredModules` 共有的四步样板：findtable 压缓存表 → 压键 →
/// gettable → 收尾栈纪律），is_cached 与 check_registered_modules 两路并入此一处。
///
/// 命中（值非 nil）时返回 true 且命中值留在栈顶（缓存表已移出，供调用方直接
/// 消费）；未命中返回 false 且两槽弹空。
pub(crate) fn cache_hit(
  l: &mut LuaState,
  table_key: &[u8],
  lookup_key: &[u8],
  form: KeyForm,
) -> bool {
  push_registry_table(l, table_key);
  push_key(l, lookup_key, form);
  l.get_table(-2);

  // 收尾栈纪律：命中时把缓存表移出栈、命中值留在栈顶供调用方消费；
  // 未命中时弹空两槽（cpp 两路同一 lua_isnil/remove/pop 形态）。
  if l.is_nil(-1) {
    l.pop(2);
    false
  } else {
    l.remove(-2);
    true
  }
}

/// 写注册表子表标记 `_TABLE[key] = mark`（true → boolean 真、false → nil），
/// 压表→压值→写字段→弹表整段收口，栈自配平。cpp
/// `markPlaceholderProvided` / `luaRequireCont` 清标记 / `luarequire_clearcacheentry`
/// 置 nil 三处共用此形态。
pub(crate) fn set_registry_mark(l: &mut LuaState, table_key: &[u8], key: &[u8], mark: bool) {
  push_registry_table(l, table_key);
  if mark {
    l.push_boolean(true);
  } else {
    l.push_nil();
  }
  set_table_field(l, -2, key);
  l.pop(1);
}

/// 读取并清除注册表子表标记（cpp `consumeCyclicPlaceholderProvided` 的
/// 读值→清 nil 两步合一）：返回字段按 Lua 真值语义是否置位，栈自配平。
pub(crate) fn take_registry_mark(l: &mut LuaState, table_key: &[u8], key: &[u8]) -> bool {
  push_registry_table(l, table_key);
  get_table_field(l, -1, key);
  let marked = l.to_boolean(-1);
  l.pop(1);
  l.push_nil();
  set_table_field(l, -2, key);
  l.pop(1);
  marked
}

/// 把栈上既有值（`value_idx` 处，按调用方压表前的栈布局）复制写入注册表
/// 子表字段 `_TABLE[key]`，压表→复制→写字段→弹表整段收口，栈自配平。cpp
/// `createPlaceholder` 与 `luaRequireCont` 常规缓存路径共用。
pub(crate) fn cache_stack_value(l: &mut LuaState, table_key: &[u8], key: &[u8], value_idx: i32) {
  // 压表前先把值索引归一（cpp 同一 lua_absindex 时机），其后 pushvalue 复制
  // 既有栈值、set_table_field 即时消费键，pop 弹掉缓存表，净变化 0。
  let value_idx = to_absolute(value_idx, l.get_top());
  push_registry_table(l, table_key);
  l.push_value(value_idx);
  set_table_field(l, -2, key);
  l.pop(1);
}
