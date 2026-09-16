use ulua_common::enums::luau_bytecode_type;
use ulua_vm::type_aliases::proto::Proto;
const LUA_TNIL: i32 = 0;
const LUA_TBOOLEAN: i32 = 1;
const LUA_TLIGHTUSERDATA: i32 = 2;
const LUA_TNUMBER: i32 = 3;
const LUA_TINTEGER: i32 = 4;
const LUA_TVECTOR: i32 = 5;
const LUA_TSTRING: i32 = 6;
const LUA_TTABLE: i32 = 7;
const LUA_TFUNCTION: i32 = 8;
const LUA_TUSERDATA: i32 = 9;
const LUA_TTHREAD: i32 = 10;
const LUA_TBUFFER: i32 = 11;

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
