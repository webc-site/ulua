//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：对应 C++ `isRequireAllowed`，
//! 仅允许 `=stdin` 或以 `@` 开头的 chunkname 触发 require。两侧判定条件语义
//! 相同（`== "=stdin"` 或 首字节为 `@`）。

use core::ffi::{CStr, c_char, c_void};

/// # Safety
///
/// `requirer_chunkname` 必须是有效、NUL 结尾的 C 字符串指针（或 null）。
pub unsafe extern "C-unwind" fn is_require_allowed(
  _l: *mut c_void,
  _ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> bool {
  unsafe {
    if requirer_chunkname.is_null() {
      return false;
    }
    let chunkname = CStr::from_ptr(requirer_chunkname).to_bytes();
    chunkname == b"=stdin" || (!chunkname.is_empty() && chunkname[0] == b'@')
  }
}
