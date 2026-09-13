use alloc::string::String;
use core::ffi::c_char;

use crate::{
  functions::get_bytecode_type_name::get_bytecode_type_name,
  records::bytecode_types::{BytecodeTypes, LBC_TYPE_ANY},
};

const LBC_TYPE_OPTIONAL_BIT: u8 = 0x80;

pub(crate) fn to_string_string_bytecode_types_c_char(
  result: &mut String,
  bc_types: &BytecodeTypes,
  userdata_types: *const *const c_char,
) {
  let optional_suffix = |t: u8| {
    if (t & LBC_TYPE_OPTIONAL_BIT) != 0 {
      "?"
    } else {
      ""
    }
  };

  let result_ty_str = unsafe { get_bytecode_type_name(bc_types.result, userdata_types) };
  let a_ty_str = unsafe { get_bytecode_type_name(bc_types.a, userdata_types) };
  let b_ty_str = unsafe { get_bytecode_type_name(bc_types.b, userdata_types) };

  result.push_str(result_ty_str);
  result.push_str(optional_suffix(bc_types.result));

  result.push_str(" <- ");

  result.push_str(a_ty_str);
  result.push_str(optional_suffix(bc_types.a));

  result.push_str(", ");

  result.push_str(b_ty_str);
  result.push_str(optional_suffix(bc_types.b));

  if bc_types.c != LBC_TYPE_ANY {
    let c_ty_str = unsafe { get_bytecode_type_name(bc_types.c, userdata_types) };
    result.push_str(", ");
    result.push_str(c_ty_str);
    result.push_str(optional_suffix(bc_types.c));
  }
}
