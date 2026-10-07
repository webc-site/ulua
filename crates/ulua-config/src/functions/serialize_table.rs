use ulua_vm::{enums::lua_type::LuaType, records::lua_state::LuaState};

use crate::{
  error::ConfigError,
  functions::lua_string::lua_string,
  records::{
    config_table::ConfigTable, config_table_key::ConfigTableKey, config_value::ConfigValue,
  },
};

/// 本轮遍历的键读取（`lua_next` 压在 -2 的键）：Number 已判型后直接取值
/// （对应 cpp `lua_tonumber`），String 经 `lua_string`，
/// 其余按 cpp 报 `TableKeyNotStringOrNumber`。
fn serialize_key(l: &mut LuaState) -> Result<ConfigTableKey, ConfigError> {
  match l.type_of(-2) {
    LuaType::Number => Ok(ConfigTableKey::from(l.to_number(-2).unwrap_or(0.0))),
    LuaType::String => Ok(ConfigTableKey::from(lua_string(l, -2))),
    _ => Err(ConfigError::TableKeyNotStringOrNumber),
  }
}

/// 本轮遍历的值读取（`lua_next` 压在 -1 的值）：标量直取；Table 复制一份压栈
/// 后递归 `serialize_table`（副本由递归帧收尾弹栈，错误路径由调用层 settop 弹出，
/// 净效果同）；其余按 cpp 以键名报 `BadTableValue`。不触碰栈高度（递归分支自我配平）。
///
/// `key` 仅用于拼装错误文案。
fn serialize_value(l: &mut LuaState, key: &ConfigTableKey) -> Result<ConfigValue, ConfigError> {
  match l.type_of(-1) {
    LuaType::Number => Ok(ConfigValue::from(l.to_number(-1).unwrap_or(0.0))),
    LuaType::String => Ok(ConfigValue::from(lua_string(l, -1))),
    LuaType::Boolean => Ok(ConfigValue::from(l.to_boolean(-1))),
    LuaType::Table => {
      // 复制表供递归调用，副本由递归帧收尾弹出
      l.push_value(-1);
      Ok(ConfigValue::from(serialize_table(l)?))
    }
    _ => Err(ConfigError::BadTableValue {
      key: key.to_string(),
    }),
  }
}

/// 本轮遍历的键值对读取（栈 -2 键 / -1 值）：出错时由调用方 [`serialize_entry`]
/// 统一做 settop 配平，本函数只负责取值、不触碰栈。
fn read_entry(l: &mut LuaState) -> Result<(ConfigTableKey, ConfigValue), ConfigError> {
  let key = serialize_key(l)?;
  let value = serialize_value(l, &key)?;
  Ok((key, value))
}

/// `serialize_table` 遍历循环的一轮处理：读取栈顶两项（-2 键 / -1 值）插入
/// `table`，返回时按 cpp 内层 `ThreadPopper` 弹掉本轮值；键留在栈上，由下一轮
/// `lua_next` 消费（迭代器语义）或遍历结束时的收尾弹栈。错误路径：settop 只弹
/// 栈顶一元素（值或子层滞留副本）；遗留键随后由外层帧的收尾弹栈弹出、表副本
/// 回归栈顶弹出——逐层收敛，否则表副本滞留（+1 槽/层）。cpp 靠异常 unwind
/// 天然免此问题，Rust 栈机须显式配平。
fn serialize_entry(l: &mut LuaState, table: &mut ConfigTable) -> Result<(), ConfigError> {
  match read_entry(l) {
    Ok((key, value)) => {
      // 本轮值处理完即弹栈（对应 cpp 内层 ThreadPopper）
      l.pop(1);
      table.insert(key, value);
      Ok(())
    }
    Err(err) => {
      l.set_top(-2);
      Err(err)
    }
  }
}

/// 遍历循环体：nil 键起始 + `lua_next` 游标，每轮把键（-2）/值（-1）压回栈，
/// `serialize_entry` 按栈位约定消费本轮键值并弹值留键。
fn serialize_entries(l: &mut LuaState) -> Result<ConfigTable, ConfigError> {
  let mut table = ConfigTable::new();

  l.push_nil();
  while l.next(-2) {
    serialize_entry(l, &mut table)?;
  }

  Ok(table)
}

/// 对应 C++ `serializeTable`：递归把栈顶 Lua 表序列化为 `ConfigTable`，
/// 错误经 `Result` 传播（`?`）。
///
/// 全部栈操作经 `LuaState` 的安全栈 API（`type_of`/`to_number`/`next`/`pop`…），
/// 无裸指针与 `unsafe`；帧出入的弹栈配对（cpp 的 `ThreadPopper`）由本函数
/// 尾部的显式 `pop(1)` 表达，成功与错误路径同帧收口。
///
/// 调用契约：栈顶（-1）为待序列化的表。
pub(crate) fn serialize_table(l: &mut LuaState) -> Result<ConfigTable, ConfigError> {
  let outcome = serialize_entries(l);
  // 处理完后把表弹出栈（对应 cpp 帧级 ThreadPopper，错误路径同样收口）
  l.pop(1);
  outcome
}
