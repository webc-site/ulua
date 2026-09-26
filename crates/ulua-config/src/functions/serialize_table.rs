use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_next::lua_next, lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
    lua_settop::lua_settop, lua_toboolean::lua_toboolean, lua_type::lua_type,
  },
  macros::lua_tonumber::lua_tonumber,
  records::lua_state::LuaState,
};

use crate::{
  error::ConfigError,
  functions::lua_string::lua_string,
  records::{
    config_table::ConfigTable, config_table_key::ConfigTableKey, config_value::ConfigValue,
    thread_popper::ThreadPopper,
  },
};

// VM 类型码，避免逐处 `as i32`
const T_NUMBER: i32 = LuaType::Number as i32;
const T_STRING: i32 = LuaType::String as i32;
const T_BOOLEAN: i32 = LuaType::Boolean as i32;
const T_TABLE: i32 = LuaType::Table as i32;

/// 本轮遍历的键读取（`lua_next` 压在 -2 的键）：Number 已判型后直接取值
/// （`lua_tonumber!` 即 cpp `lua_tonumber`），String 经 `lua_string`，
/// 其余按 cpp 报 `TableKeyNotStringOrNumber`。
///
/// # Safety
/// `l` 必须是有效 VM 状态，且栈 -2 处为本轮遍历的键。
unsafe fn serialize_key(l: *mut LuaState) -> Result<ConfigTableKey, ConfigError> {
  let kind = unsafe { lua_type(l, -2) };
  match kind {
    T_NUMBER => Ok(ConfigTableKey::from(unsafe { lua_tonumber!(l, -2) })),
    T_STRING => Ok(ConfigTableKey::from(unsafe { lua_string(l, -2) })),
    _ => Err(ConfigError::TableKeyNotStringOrNumber),
  }
}

/// 本轮遍历的值读取（`lua_next` 压在 -1 的值）：标量直取；Table 复制一份压栈
/// 后递归 `serialize_table`（副本由递归帧的 ThreadPopper 弹出（错误路径由调用层 settop 弹出，净效果同）；其余按 cpp 以
/// 键名报 `BadTableValue`。不触碰栈高度（递归分支自我配平）。
///
/// # Safety
/// `l` 必须是有效 VM 状态，且栈 -1 处为本轮遍历的值；`key` 仅用于拼装错误文案。
unsafe fn serialize_value(
  l: *mut LuaState,
  key: &ConfigTableKey,
) -> Result<ConfigValue, ConfigError> {
  let kind = unsafe { lua_type(l, -1) };
  match kind {
    T_NUMBER => Ok(ConfigValue::from(unsafe { lua_tonumber!(l, -1) })),
    T_STRING => Ok(ConfigValue::from(unsafe { lua_string(l, -1) })),
    T_BOOLEAN => Ok(ConfigValue::from(unsafe { lua_toboolean(l, -1) } != 0)),
    T_TABLE => {
      // 复制表供递归调用，副本由递归帧的 ThreadPopper 弹出
      unsafe { lua_pushvalue(l, -1) };
      Ok(ConfigValue::from(unsafe { serialize_table(l) }?))
    }
    _ => Err(ConfigError::BadTableValue {
      key: key.to_string(),
    }),
  }
}

/// `serialize_table` 遍历循环的一轮处理：读取栈顶两项（-2 键 / -1 值）插入
/// `table`，返回时按 cpp 内层 `ThreadPopper` 弹掉本轮值；键留在栈上，由下一轮
/// `lua_next` 消费（迭代器语义）或遍历结束时的收尾弹栈。
///
/// # Safety
/// `l` 必须是有效 VM 状态，且栈顶两项（-2/-1）为本轮 `lua_next` 压回的键值对。
unsafe fn serialize_entry(l: *mut LuaState, table: &mut ConfigTable) -> Result<(), ConfigError> {
  // 本轮值处理完即弹栈（对应 cpp 内层 ThreadPopper）
  let _value_popper = ThreadPopper { l };

  let result = (|| {
    let key = unsafe { serialize_key(l) }?;
    let value = unsafe { serialize_value(l, &key) }?;
    table.insert(key, value);
    Ok(())
  })();

  if result.is_err() {
    // 错误路径：settop 只弹栈顶一元素（值或子层滞留副本）；遗留键随后由外层
    // _table_popper 弹出、表副本回归栈顶弹出——逐层收敛，否则表副本滞留
    // （+1 槽/层）。cpp 靠异常 unwind 天然免此问题，Rust 栈机须显式配平
    unsafe { lua_settop(l, -2) };
  }
  result
}

/// 对应 C++ `serializeTable`：递归把栈顶 Lua 表序列化为 `ConfigTable`，
/// 错误经 `Result` 传播（`?`）。
///
/// 本函数是 ulua-config 对 ulua-vm C-API 的绑定边界，属「(b) 表遍历栈机 +
/// RAII 弹栈」的最小形态：栈机骨架（nil 键起始 + `lua_next` 游标 + 双层
/// `ThreadPopper` 配平）必须同帧持有，无法再外提；逐键/值语义已拆到
/// `serialize_entry`/`serialize_key`/`serialize_value` 三个带栈位契约的小函数，
/// 本函数体内只剩遍历控制流。裸指针与 `unsafe` 仅限栈操作调用处，出参
/// （`isnum`）经 `lua_tonumber!` 宏以 null 传回契约（对应 cpp 直接调
/// `lua_tonumber`），不外泄到返回值。
///
/// # Safety
/// `l` 必须是有效 VM 状态，且栈顶（-2）为待序列化的表。
pub(crate) unsafe fn serialize_table(l: *mut LuaState) -> Result<ConfigTable, ConfigError> {
  // 处理完后把表弹出栈
  let _table_popper = ThreadPopper { l };
  let mut table = ConfigTable::new();

  // Safety: l 为有效 VM 状态，-2 为待遍历表（本函数 /// # Safety 契约）；
  // lua_next 自 nil 键起始迭代、返回 0 时不压栈（收尾即弹掉最后键的语义由
  // VM 保证），每轮把键（-2）/值（-1）压回栈；serialize_entry 按自身契约
  // 消费本轮键值并弹值留键，整轮净栈效果与拆块前逐字一致。
  unsafe {
    lua_pushnil(l);
    while lua_next(l, -2) != 0 {
      serialize_entry(l, &mut table)?;
    }
  }

  Ok(table)
}
