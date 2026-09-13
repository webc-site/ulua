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
  unsafe {
    // 处理完后把表弹出栈
    let _table_popper = ThreadPopper { l };
    let mut table = ConfigTable::new();

    lua_pushnil(l);
    while lua_next(l, -2) != 0 {
      let _value_popper = ThreadPopper { l };

      let key = match lua_type(l, -2) {
        T_NUMBER => ConfigTableKey::from(lua_tonumberx(l, -2, null_mut())),
        T_STRING => ConfigTableKey::from(lua_string(l, -2)),
        _ => {
          *error = String::from("configuration table keys must be strings or numbers");
          return None;
        }
      };

      match lua_type(l, -1) {
        T_NUMBER => {
          table.insert(key, ConfigValue::from(lua_tonumberx(l, -1, null_mut())));
        }
        T_STRING => {
          table.insert(key, ConfigValue::from(lua_string(l, -1)));
        }
        T_BOOLEAN => {
          table.insert(key, ConfigValue::from(lua_toboolean(l, -1) != 0));
        }
        T_TABLE => {
          // 复制表供递归调用
          lua_pushvalue(l, -1);
          let nested = serialize_table(l, error)?;
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
}
