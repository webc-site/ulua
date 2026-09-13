use core::{ffi::c_char, slice::from_raw_parts};

use ulua_ast::records::ast_array::AstArray;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{enums::type_constant_folding::Type, records::constant::Constant};

impl Constant {
  pub fn get_string(&self) -> AstArray<c_char> {
    LUAU_ASSERT!(self.r#type == Type::String);
    let data = unsafe { self.data.value_string as *mut c_char };
    let size = self.string_length as usize;
    AstArray { data, size }
  }

  /// Returns the string constant as a byte slice.
  ///
  /// The data pointer lives in the constant's storage, outliving `self`, so
  /// the slice is tied to `&self` rather than to the temporary `AstArray`.
  #[inline]
  pub fn get_string_bytes(&self) -> &[u8] {
    LUAU_ASSERT!(self.r#type == Type::String);
    let data = unsafe { self.data.value_string as *const u8 };
    let size = self.string_length as usize;
    if data.is_null() || size == 0 {
      &[]
    } else {
      unsafe { from_raw_parts(data, size) }
    }
  }
}
