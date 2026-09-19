extern crate alloc;

use ulua_ast::records::ast_name::AstName;
use ulua_common::enums::luau_bytecode_type::{
  LBC_TYPE_ANY, LBC_TYPE_BOOLEAN, LBC_TYPE_BUFFER, LBC_TYPE_INTEGER, LBC_TYPE_INVALID,
  LBC_TYPE_NIL, LBC_TYPE_NUMBER, LBC_TYPE_STRING, LBC_TYPE_THREAD, LBC_TYPE_VECTOR,
  LuauBytecodeType,
};

pub(crate) fn get_primitive_type(name: AstName) -> LuauBytecodeType {
  if name == "nil" {
    LBC_TYPE_NIL
  } else if name == "boolean" {
    LBC_TYPE_BOOLEAN
  } else if name == "number" {
    LBC_TYPE_NUMBER
  } else if name == "integer" {
    LBC_TYPE_INTEGER
  } else if name == "string" {
    LBC_TYPE_STRING
  } else if name == "thread" {
    LBC_TYPE_THREAD
  } else if name == "buffer" {
    LBC_TYPE_BUFFER
  } else if name == "vector" {
    LBC_TYPE_VECTOR
  } else if name == "any" || name == "unknown" {
    LBC_TYPE_ANY
  } else {
    LBC_TYPE_INVALID
  }
}
