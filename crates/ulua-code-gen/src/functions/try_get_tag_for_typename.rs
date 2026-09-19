use ulua_vm::enums::lua_type::LuaType;

use crate::records::ir_data::K_UNKNOWN_TAG;

const LUA_TNIL: u8 = LuaType::Nil as u8;
const LUA_TBOOLEAN: u8 = LuaType::Boolean as u8;
const LUA_TNUMBER: u8 = LuaType::Number as u8;
const LUA_TINTEGER: u8 = LuaType::Integer as u8;
const LUA_TVECTOR: u8 = LuaType::Vector as u8;
const LUA_TSTRING: u8 = LuaType::String as u8;
const LUA_TTABLE: u8 = LuaType::Table as u8;
const LUA_TFUNCTION: u8 = LuaType::Function as u8;
const LUA_TTHREAD: u8 = LuaType::Thread as u8;
const LUA_TBUFFER: u8 = LuaType::Buffer as u8;

pub(crate) fn try_get_tag_for_typename(name: &str, for_typeof: bool) -> u8 {
  if name == "nil" {
    return LUA_TNIL;
  }

  if name == "boolean" {
    return LUA_TBOOLEAN;
  }

  if name == "number" {
    return LUA_TNUMBER;
  }

  if name == "integer" {
    return LUA_TINTEGER;
  }

  // typeof(vector) can be changed by environment
  // TODO: support the environment option
  if name == "vector" && !for_typeof {
    return LUA_TVECTOR;
  }

  if name == "string" {
    return LUA_TSTRING;
  }

  if name == "table" {
    return LUA_TTABLE;
  }

  if name == "function" {
    return LUA_TFUNCTION;
  }

  if name == "thread" {
    return LUA_TTHREAD;
  }

  if name == "buffer" {
    return LUA_TBUFFER;
  }

  K_UNKNOWN_TAG
}
