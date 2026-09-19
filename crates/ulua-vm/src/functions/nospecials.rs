use core::ffi::{CStr, c_char, c_int};

const SPECIALS: &[u8] = b"^$*+?.([%-";

/// cpp `VM/src/lstrlib.cpp:636`：逐 NUL 段扫描，任一特殊字符即返回 0。
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn nospecials(p: *const c_char, l: usize) -> c_int {
  let mut upto: usize = 0;
  loop {
    // cpp 的 strpbrk × strlen 换成对 CStr 字节切片的单次短路扫描
    let bytes = unsafe { CStr::from_ptr(p.add(upto)) }.to_bytes();
    if bytes.iter().any(|b| SPECIALS.contains(b)) {
      return 0;
    }

    upto += bytes.len() + 1; // 跳过段与其 NUL 终止符
    if upto > l {
      return 1;
    }
  }
}
