const LUA_TNIL: u8 = 0;
const LUA_TBOOLEAN: u8 = 1;
const LUA_TNUMBER: u8 = 3;
const LUA_TINTEGER: u8 = 4;
const LUA_TVECTOR: u8 = 5;
const LUA_TSTRING: u8 = 6;
const LUA_TTABLE: u8 = 7;
const LUA_TFUNCTION: u8 = 8;
const LUA_TTHREAD: u8 = 10;
const LUA_TBUFFER: u8 = 11;

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

  0xff
}
