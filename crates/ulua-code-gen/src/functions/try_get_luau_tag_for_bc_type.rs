use ulua_common::enums::luau_bytecode_type;
const LUA_TNIL: u8 = 0;
const LUA_TBOOLEAN: u8 = 1;
const LUA_TNUMBER: u8 = 3;
const LUA_TINTEGER: u8 = 4;
const LUA_TVECTOR: u8 = 5;
const LUA_TSTRING: u8 = 6;
const LUA_TTABLE: u8 = 7;
const LUA_TFUNCTION: u8 = 8;
const LUA_TUSERDATA: u8 = 9;
const LUA_TTHREAD: u8 = 10;
const LUA_TBUFFER: u8 = 11;

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

const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = 64;
const LBC_TYPE_TAGGED_USERDATA_END: u8 = luau_bytecode_type::LBC_TYPE_TAGGED_USERDATA_END.0 as u8;
const LBC_TYPE_OPTIONAL_BIT: u8 = 128;

pub fn try_get_luau_tag_for_bc_type(mut bc_type: u8, ignore_optional_part: bool) -> Option<u8> {
  if ignore_optional_part {
    bc_type &= !LBC_TYPE_OPTIONAL_BIT;
  }

  match bc_type {
    LBC_TYPE_NIL => Some(LUA_TNIL),
    LBC_TYPE_BOOLEAN => Some(LUA_TBOOLEAN),
    LBC_TYPE_NUMBER => Some(LUA_TNUMBER),
    LBC_TYPE_INTEGER => Some(LUA_TINTEGER),
    LBC_TYPE_STRING => Some(LUA_TSTRING),
    LBC_TYPE_TABLE => Some(LUA_TTABLE),
    LBC_TYPE_FUNCTION => Some(LUA_TFUNCTION),
    LBC_TYPE_THREAD => Some(LUA_TTHREAD),
    LBC_TYPE_USERDATA => Some(LUA_TUSERDATA),
    LBC_TYPE_VECTOR => Some(LUA_TVECTOR),
    LBC_TYPE_BUFFER => Some(LUA_TBUFFER),
    _ => {
      if (LBC_TYPE_TAGGED_USERDATA_BASE..LBC_TYPE_TAGGED_USERDATA_END).contains(&bc_type) {
        return Some(LUA_TUSERDATA);
      }
      None
    }
  }
}
