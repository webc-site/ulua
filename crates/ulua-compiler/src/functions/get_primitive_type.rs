extern crate alloc;

use ulua_ast::records::ast_name::AstName;
use ulua_common::enums::luau_bytecode_type::{
  LBC_TYPE_ANY, LBC_TYPE_BOOLEAN, LBC_TYPE_BUFFER, LBC_TYPE_INTEGER, LBC_TYPE_INVALID,
  LBC_TYPE_NIL, LBC_TYPE_NUMBER, LBC_TYPE_STRING, LBC_TYPE_THREAD, LBC_TYPE_VECTOR,
  LuauBytecodeType,
};

pub(crate) fn get_primitive_type(name: AstName) -> LuauBytecodeType {
  if name.operator_eq_c_char(c"nil") {
    LBC_TYPE_NIL
  } else if name.operator_eq_c_char(c"boolean") {
    LBC_TYPE_BOOLEAN
  } else if name.operator_eq_c_char(c"number") {
    LBC_TYPE_NUMBER
  } else if name.operator_eq_c_char(c"integer") {
    LBC_TYPE_INTEGER
  } else if name.operator_eq_c_char(c"string") {
    LBC_TYPE_STRING
  } else if name.operator_eq_c_char(c"thread") {
    LBC_TYPE_THREAD
  } else if name.operator_eq_c_char(c"buffer") || name.operator_eq_c_char(c"Buffer") {
    LBC_TYPE_BUFFER
  } else if name.operator_eq_c_char(c"vector") {
    LBC_TYPE_VECTOR
  } else if name.operator_eq_c_char(c"any") || name.operator_eq_c_char(c"unknown") {
    LBC_TYPE_ANY
  } else {
    LBC_TYPE_INVALID
  }
}
