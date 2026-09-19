use ulua_common::enums::luau_bytecode_type;
use ulua_vm::{enums::lua_type::LuaType, type_aliases::proto::Proto};
const LUA_TNIL: i32 = LuaType::Nil as i32;
const LUA_TBOOLEAN: i32 = LuaType::Boolean as i32;
const LUA_TLIGHTUSERDATA: i32 = LuaType::LightUserData as i32;
const LUA_TNUMBER: i32 = LuaType::Number as i32;
const LUA_TINTEGER: i32 = LuaType::Integer as i32;
const LUA_TVECTOR: i32 = LuaType::Vector as i32;
const LUA_TSTRING: i32 = LuaType::String as i32;
const LUA_TTABLE: i32 = LuaType::Table as i32;
const LUA_TFUNCTION: i32 = LuaType::Function as i32;
const LUA_TUSERDATA: i32 = LuaType::UserData as i32;
const LUA_TTHREAD: i32 = LuaType::Thread as i32;
const LUA_TBUFFER: i32 = LuaType::Buffer as i32;

const LBC_TYPE_NIL: u8 = luau_bytecode_type::LBC_TYPE_NIL.0 as u8;
const LBC_TYPE_BOOLEAN: u8 = luau_bytecode_type::LBC_TYPE_BOOLEAN.0 as u8;
const LBC_TYPE_NUMBER: u8 = luau_bytecode_type::LBC_TYPE_NUMBER.0 as u8;
const LBC_TYPE_STRING: u8 = luau_bytecode_type::LBC_TYPE_STRING.0 as u8;
const LBC_TYPE_TABLE: u8 = luau_bytecode_type::LBC_TYPE_TABLE.0 as u8;
const LBC_TYPE_FUNCTION: u8 = luau_bytecode_type::LBC_TYPE_FUNCTION.0 as u8;
const LBC_TYPE_USERDATA: u8 = luau_bytecode_type::LBC_TYPE_USERDATA.0 as u8;
const LBC_TYPE_THREAD: u8 = luau_bytecode_type::LBC_TYPE_THREAD.0 as u8;
const LBC_TYPE_VECTOR: u8 = luau_bytecode_type::LBC_TYPE_VECTOR.0 as u8;
const LBC_TYPE_BUFFER: u8 = luau_bytecode_type::LBC_TYPE_BUFFER.0 as u8;
const LBC_TYPE_INTEGER: u8 = luau_bytecode_type::LBC_TYPE_INTEGER.0 as u8;
const LBC_TYPE_ANY: u8 = luau_bytecode_type::LBC_TYPE_ANY.0 as u8;

pub(crate) fn get_bytecode_constant_tag(proto: *mut Proto, ki: u32) -> u8 {
  unsafe {
    let protok = (*proto).k.add(ki as usize).read();
    match protok.tt {
      LUA_TNIL => LBC_TYPE_NIL,
      LUA_TBOOLEAN => LBC_TYPE_BOOLEAN,
      LUA_TLIGHTUSERDATA => LBC_TYPE_USERDATA,
      LUA_TNUMBER => LBC_TYPE_NUMBER,
      LUA_TINTEGER => LBC_TYPE_INTEGER,
      LUA_TVECTOR => LBC_TYPE_VECTOR,
      LUA_TSTRING => LBC_TYPE_STRING,
      LUA_TTABLE => LBC_TYPE_TABLE,
      LUA_TFUNCTION => LBC_TYPE_FUNCTION,
      LUA_TUSERDATA => LBC_TYPE_USERDATA,
      LUA_TTHREAD => LBC_TYPE_THREAD,
      LUA_TBUFFER => LBC_TYPE_BUFFER,
      _ => LBC_TYPE_ANY,
    }
  }
}
