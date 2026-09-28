use ulua_common::functions::c_str::cstr_bytes;

use crate::records::ast_name::AstName;

impl Default for AstName {
  fn default() -> Self {
    Self::new()
  }
}

impl AstName {
  /// 从存活指针直接包名（cpp `AstName(const char* value)`，Ast.h:30）：
  /// 通过 cstr_bytes 计算长度并记录在 len 字段，使得后续 as_bytes 均为 O(1)。
  pub fn ast_name_u8(value: *const u8) -> Self {
    if value.is_null() {
      Self::new()
    } else {
      let len = unsafe { cstr_bytes(value.cast()) }.len() as u32;
      Self { value, len }
    }
  }
}
