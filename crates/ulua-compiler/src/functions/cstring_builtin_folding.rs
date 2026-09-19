use core::ffi::c_char;

use crate::{
  enums::type_constant_folding::Type,
  records::constant::{Constant, ConstantData},
};

/// 由字节切片构造字符串常量：仅借用源数据，不拷贝（与 C++ `Constant` 语义一致）。
#[inline]
pub fn cstring_slice(s: &[u8]) -> Constant {
  Constant {
    r#type: Type::String,
    string_length: s.len() as u32,
    data: ConstantData {
      value_string: s.as_ptr() as *const c_char,
    },
  }
}

/// 由字符串切片构造字符串常量。
#[inline]
pub fn cstring_str(s: &str) -> Constant {
  cstring_slice(s.as_bytes())
}
