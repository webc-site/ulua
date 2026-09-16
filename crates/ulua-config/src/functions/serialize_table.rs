use alloc::{format, string::String};
use core::ptr::null_mut;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_next::lua_next, lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
    lua_toboolean::lua_toboolean, lua_tonumberx::lua_tonumberx, lua_type::lua_type,
  },
  type_aliases::lua_state::lua_State,
};

use crate::{
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

/// 对应 C++ `serializeTable`：递归把栈顶 Lua 表序列化为 `ConfigTable`。
///
/// # Safety
/// `l` 必须是有效 VM 状态，且栈顶（-2）为待序列化的表。
pub(crate) unsafe fn serialize_table(l: *mut lua_State, error: &mut String) -> Option<ConfigTable> {
  // 处理完后把表弹出栈
  let _table_popper = ThreadPopper { l };
  let mut table = ConfigTable::new();

  // SAFETY: l 为有效 VM 状态，-2 为待遍历表
  unsafe { lua_pushnil(l) };
  // SAFETY: lua_next 按 nil 起始键迭代表
  while unsafe { lua_next(l, -2) } != 0 {
    let _value_popper = ThreadPopper { l };

    // SAFETY: l 有效，-2 为当前键
    let key = match unsafe { lua_type(l, -2) } {
      T_NUMBER => ConfigTableKey::from(unsafe { lua_tonumberx(l, -2, null_mut()) }),
      T_STRING => ConfigTableKey::from(unsafe { lua_string(l, -2) }),
      _ => {
        *error = String::from("configuration table keys must be strings or numbers");
        return None;
      }
    };

    // SAFETY: l 有效，-1 为当前值
    match unsafe { lua_type(l, -1) } {
      T_NUMBER => {
        table.insert(
          key,
          ConfigValue::from(unsafe { lua_tonumberx(l, -1, null_mut()) }),
        );
      }
      T_STRING => {
        table.insert(key, ConfigValue::from(unsafe { lua_string(l, -1) }));
      }
      T_BOOLEAN => {
        table.insert(key, ConfigValue::from(unsafe { lua_toboolean(l, -1) } != 0));
      }
      T_TABLE => {
        // 复制表供递归调用
        // SAFETY: l 有效，-1 为嵌套表
        unsafe { lua_pushvalue(l, -1) };
        let nested = unsafe { serialize_table(l, error)? };
        table.insert(key, ConfigValue::from(nested));
      }
      _ => {
        *error = format!(
          "configuration value for key \"{}\" must be a string, number, boolean, or nested table",
          key
        );
        return None;
      }
    }
  }

  Some(table)
}
