use ulua_common::macros::luau_assert::LUAU_ASSERT;

// Values must match the LuauBytecodeType enum (ulua_common / C++ Bytecode.h):
// NIL=0, BOOLEAN=1, NUMBER=2, STRING=3, TABLE=4, FUNCTION=5, THREAD=6,
// USERDATA=7, VECTOR=8, BUFFER=9, INTEGER=10, ANY=15. The port had a spurious
// INTEGER=3 that shifted STRING..BUFFER down by one (USERDATA dumped as 'thread').
const LBC_TYPE_NIL: u8 = 0;
const LBC_TYPE_BOOLEAN: u8 = 1;
const LBC_TYPE_NUMBER: u8 = 2;
const LBC_TYPE_STRING: u8 = 3;
const LBC_TYPE_TABLE: u8 = 4;
const LBC_TYPE_FUNCTION: u8 = 5;
const LBC_TYPE_THREAD: u8 = 6;
const LBC_TYPE_USERDATA: u8 = 7;
const LBC_TYPE_VECTOR: u8 = 8;
const LBC_TYPE_BUFFER: u8 = 9;
const LBC_TYPE_INTEGER: u8 = 10;
const LBC_TYPE_ANY: u8 = 15;

const LBC_TYPE_OPTIONAL_BIT: u8 = 128;

pub(crate) fn get_base_type_string(r#type: u8) -> &'static str {
  let tag = r#type & !LBC_TYPE_OPTIONAL_BIT;

  match tag {
    LBC_TYPE_NIL => "nil",
    LBC_TYPE_BOOLEAN => "boolean",
    LBC_TYPE_NUMBER => "number",
    LBC_TYPE_INTEGER => "integer",
    LBC_TYPE_STRING => "string",
    LBC_TYPE_TABLE => "table",
    LBC_TYPE_FUNCTION => "function",
    LBC_TYPE_THREAD => "thread",
    LBC_TYPE_USERDATA => "userdata",
    LBC_TYPE_VECTOR => "vector",
    LBC_TYPE_BUFFER => "Buffer",
    LBC_TYPE_ANY => "any",
    _ => {
      LUAU_ASSERT!(false);
      ""
    }
  }
}
