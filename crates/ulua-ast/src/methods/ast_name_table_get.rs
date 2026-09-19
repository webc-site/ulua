//! `AstName AstNameTable::get(const char* name) const` — Ast/src/Lexer.cpp:264.

use core::ffi::{CStr, c_char};

use crate::records::{ast_name::AstName, ast_name_table::AstNameTable};

impl AstNameTable {
  /// Looks up a name in the table.
  ///
  /// C 边界最小收口：唯一保留的 `char*` 入口，仅供持有
  /// `*const *const c_char` 数组的 FFI 场景调用（如 `CompileOptions` 的
  /// `libraries_with_known_members` / `userdata_types`）。Rust 侧一律走
  /// [`Self::get_str`] / [`Self::get_slice`]。
  ///
  /// # Safety
  /// `name` must point to a valid null-terminated C string.
  pub unsafe fn get(&self, name: *const c_char) -> AstName {
    let len = unsafe { CStr::from_ptr(name).to_bytes().len() };
    self.get_with_type(name, len).0
  }

  /// Looks up a name by byte slice.
  #[inline]
  pub fn get_slice(&self, name: &[u8]) -> AstName {
    self
      .get_with_type(name.as_ptr() as *const c_char, name.len())
      .0
  }

  /// Looks up a name by string slice.
  #[inline]
  pub fn get_str(&self, name: &str) -> AstName {
    self.get_slice(name.as_bytes())
  }
}
