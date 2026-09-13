use core::ffi::c_char;

use crate::{
  enums::type_constant_folding::Type,
  records::constant::{Constant, ConstantData},
};

pub fn cstring_c_char_usize(v: *const c_char, len: usize) -> Constant {
  Constant {
    r#type: Type::String,
    string_length: len as u32,
    data: ConstantData { value_string: v },
  }
}
