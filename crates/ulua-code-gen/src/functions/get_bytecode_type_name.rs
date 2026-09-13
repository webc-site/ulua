use core::ffi::{CStr, c_char};

use ulua_common::enums::luau_bytecode_type;
const LBC_TYPE_NIL: u8 = luau_bytecode_type::LBC_TYPE_NIL.0 as u8;
const LBC_TYPE_BOOLEAN: u8 = luau_bytecode_type::LBC_TYPE_BOOLEAN.0 as u8;
const LBC_TYPE_NUMBER: u8 = luau_bytecode_type::LBC_TYPE_NUMBER.0 as u8;
const LBC_TYPE_STRING: u8 = luau_bytecode_type::LBC_TYPE_STRING.0 as u8;
const LBC_TYPE_TABLE: u8 = luau_bytecode_type::LBC_TYPE_TABLE.0 as u8;
const LBC_TYPE_FUNCTION: u8 = luau_bytecode_type::LBC_TYPE_FUNCTION.0 as u8;
const LBC_TYPE_THREAD: u8 = luau_bytecode_type::LBC_TYPE_THREAD.0 as u8;
const LBC_TYPE_USERDATA: u8 = luau_bytecode_type::LBC_TYPE_USERDATA.0 as u8;
const LBC_TYPE_VECTOR: u8 = luau_bytecode_type::LBC_TYPE_VECTOR.0 as u8;
const LBC_TYPE_BUFFER: u8 = luau_bytecode_type::LBC_TYPE_BUFFER.0 as u8;
const LBC_TYPE_ANY: u8 = luau_bytecode_type::LBC_TYPE_ANY.0 as u8;
const LBC_TYPE_INTEGER: u8 = luau_bytecode_type::LBC_TYPE_INTEGER.0 as u8;

const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = 64;
const LBC_TYPE_TAGGED_USERDATA_END: u8 = luau_bytecode_type::LBC_TYPE_TAGGED_USERDATA_END.0 as u8;
const LBC_TYPE_OPTIONAL_BIT: u8 = 128;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_bytecode_type_name<'a>(
  mut r#type: u8,
  userdata_types: *const *const c_char,
) -> &'a str {
  // Optional bit should be handled externally
  r#type &= !LBC_TYPE_OPTIONAL_BIT;

  if (LBC_TYPE_TAGGED_USERDATA_BASE..LBC_TYPE_TAGGED_USERDATA_END).contains(&r#type) {
    if !userdata_types.is_null() {
      let ptr = unsafe { *userdata_types.add((r#type - LBC_TYPE_TAGGED_USERDATA_BASE) as usize) };
      if !ptr.is_null() {
        return unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("userdata") };
      }
    }

    return "userdata";
  }

  match r#type {
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
    LBC_TYPE_BUFFER => "buffer",
    LBC_TYPE_ANY => "any",
    _ => {
      ulua_common::LUAU_ASSERT!(false);
      "unknown"
    }
  }
}
