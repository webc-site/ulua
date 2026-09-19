use core::{ffi::c_char, slice::from_raw_parts};

use ulua_ast::records::ast_array::AstArray;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::constant::{Constant, ConstantStr};

impl Constant {
  pub fn get_string(&self) -> AstArray<c_char> {
    let ConstantStr { ptr, len } = self.as_str();
    // 指针借用自 AST/字符串表，沿用 C++ 的 AstArray 可变指针表示
    AstArray {
      data: ptr.cast_mut(),
      size: len as usize,
    }
  }

  /// Returns the string constant as a byte slice.
  ///
  /// The data pointer lives in the constant's storage, outliving `self`, so
  /// the slice is tied to `&self` rather than to the temporary `AstArray`.
  #[inline]
  pub fn get_string_bytes(&self) -> &[u8] {
    let ConstantStr { ptr, len } = self.as_str();
    if ptr.is_null() || len == 0 {
      &[]
    } else {
      unsafe { from_raw_parts(ptr.cast::<u8>(), len as usize) }
    }
  }

  /// 取出字符串载荷；非字符串常量属契约违例，断言拦截
  #[inline]
  pub(crate) fn as_str(&self) -> ConstantStr {
    LUAU_ASSERT!(matches!(self, Constant::Str(_)));
    match self {
      Constant::Str(s) => *s,
      _ => ConstantStr {
        ptr: core::ptr::null(),
        len: 0,
      },
    }
  }
}
