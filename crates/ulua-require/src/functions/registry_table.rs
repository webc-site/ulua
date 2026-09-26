//! 注册表缓存表的字节键栈操作门面：调用点一律只交出 `&[u8]` 键，
//! 表字段读写以「`push_c_str` 压键 + `lua_gettable`/`lua_settable`」的
//! 纯字节形态完成（与 `cache_hit` 同一手法），不再补 NUL；
//! 只有 `lua_l_findtable` 那类按 C 路径指针逐段推进的 ulua-vm C ABI 仍经
//! `with_c_str` 收口，`*const c_char` 只存在于该闭包调用期内。
//!
//! 本模块不新增任何裸指针：`LuaState` 句柄与 `LUA_REGISTRYINDEX` 伪索引仍是
//! ulua-vm C-API 的真边界形态（与 cyclic_placeholder 批同一裁定）。

use core::ffi::c_int;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettable::lua_gettable, lua_gettop::lua_gettop, lua_insert::lua_insert,
    lua_l_findtable::lua_l_findtable, lua_remove::lua_remove, lua_settable::lua_settable,
    lua_type::lua_type,
  },
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::LuaState,
};

use crate::functions::{
  c_str_prefix::{push_c_str, with_c_str},
  stack_index::to_absolute,
};

/// 压入注册表下的命名子表（缺失则以 1 项预容量创建），对应 cpp
/// `lua_l_findtable(L, LUA_REGISTRYINDEX, tableKey, 1)`。
///
/// `with_c_str` 在此保留：`lua_l_findtable` 的契约是按 C 路径指针逐段
/// （`fname.add(i + 1)`）推进并在段间查表，只接受 NUL 结尾串，无字节切片形态；
/// 表键都是本 crate 的静态常量，短名走栈内联缓冲、不分配。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；调用后栈净增一个表值。
pub(crate) unsafe fn push_registry_table(l: *mut LuaState, table_key: &[u8]) {
  // Safety: l 由契约保证有效；with_c_str 交出的键指针补有 NUL 且仅闭包调用期内存活，
  // lua_l_findtable 在本次调用内读完键内容，不外泄。
  with_c_str(table_key, |key| unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, key, 1);
  });
}

/// 取 `table_idx` 处表的字节键字段并压栈，对应 cpp `lua_getfield(L, idx, key.c_str())`。
///
/// 键经 `push_c_str`（`lua_pushlstring` 的 ptr+len 形态）入栈后走 `lua_gettable`：
/// `lua_getfield` 内部即 `lua_s_new`（strlen + `lua_s_newlstr`）+ `lua_v_gettable`，
/// 两路的键串与栈净效果逐字节一致，且免掉「补 NUL 交给 VM、VM 再 strlen 一遍」的临时缓冲。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，`table_idx` 处必须是表；调用后栈净增一个值。
pub(crate) unsafe fn get_table_field(l: *mut LuaState, table_idx: c_int, key: &[u8]) {
  // Safety: l 由契约保证存活，lua_gettop 只做同一栈区内的指针差值；压键前先把
  // 表索引归一为绝对索引，故其后 lua_gettable 读到的仍是同一表槽。
  let table_idx = to_absolute(table_idx, unsafe { lua_gettop(l) });
  // Safety: l/table_idx 由契约保证；push_c_str 按自身契约把键（按首个 NUL 截断，
  // 与 cpp `key.c_str()` 的 strlen 语义一致）净压一槽，lua_gettable 消费该键、压回字段值。
  unsafe {
    push_c_str(l, key);
    lua_gettable(l, table_idx);
  }
}

/// 用栈顶值写入 `table_idx` 处表的字节键字段（并消费该值），对应 cpp
/// `lua_setfield(L, idx, key.c_str())`。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，`table_idx` 处必须是表且栈顶为待写入的值；
/// 调用后栈净减一个值。
pub(crate) unsafe fn set_table_field(l: *mut LuaState, table_idx: c_int, key: &[u8]) {
  // Safety: 同 get_table_field 的索引归一与 lua_gettop 契约。
  let table_idx = to_absolute(table_idx, unsafe { lua_gettop(l) });
  // Safety: 键先压栈再 `lua_insert(-2)` 挪到值之下，凑成 `lua_settable` 要求的
  // [key, value] 布局；两串都是本帧字节串的即时消费，insert/settable 均为纯栈操作，
  // 净效果与原 `lua_setfield`（消费栈顶值）一致。
  unsafe {
    push_c_str(l, key);
    lua_insert(l, -2);
    lua_settable(l, table_idx);
  }
}

/// 注册表缓存表的「字节键 → 命中查询」整段收口（cpp `isCached` /
/// `checkRegisteredModules` 共有的四步样板：findtable 压缓存表 → 压键 →
/// gettable → 收尾栈纪律），is_cached 与 check_registered_modules 两路并入此一处。
///
/// `push_key` 是两条路唯一的差异点：查表键的压栈形态（原样推入 / ASCII 小写
/// 归一后推入），由调用方以函数指针注入，二者签名同为
/// `unsafe fn(*mut LuaState, &[u8])`（c_str_prefix 门面的 push_c_str /
/// push_lowered_c_str）。
///
/// 命中（值非 nil）时返回 true 且命中值留在栈顶（缓存表已移出，供调用方直接
/// 消费）；未命中返回 false 且两槽弹空。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`push_key(l, lookup_key)` 必须把键压上栈顶
/// （净增一槽、不触碰其下的缓存表）。
pub(crate) unsafe fn cache_hit(
  l: *mut LuaState,
  table_key: &[u8],
  lookup_key: &[u8],
  push_key: unsafe fn(*mut LuaState, &[u8]),
) -> bool {
  // Safety: l 由契约保证有效；push_registry_table 键指针仅闭包期内存活；
  // push_key 按契约净压键值一槽；lua_gettable 消费键、压回字段值；
  // 收尾栈纪律由 take_cache_hit 承担（其自身带契约）。
  unsafe {
    push_registry_table(l, table_key);
    push_key(l, lookup_key);
    lua_gettable(l, -2);
    take_cache_hit(l)
  }
}

/// 缓存表查询的收尾栈纪律（[`cache_hit`] 的收尾步）：
/// 进入时栈顶为刚 `lua_gettable` 得到的值、其下是缓存表。命中（非 nil）时把缓存表
/// 移出栈、命中值留在栈顶供调用方直接消费；未命中时弹空两槽。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，且栈顶两个槽依次为缓存表、其字段值；
/// 命中时返回值 `true` 且栈净减一，未命中返回 `false` 且栈净减二。
pub(crate) unsafe fn take_cache_hit(l: *mut LuaState) -> bool {
  // Safety: l 由契约保证有效，三处操作均为 VM 栈 API，无指针解引用。
  unsafe {
    if lua_type(l, -1) == LuaType::Nil as i32 {
      lua_pop(l, 2);
      false
    } else {
      lua_remove(l, -2);
      true
    }
  }
}
