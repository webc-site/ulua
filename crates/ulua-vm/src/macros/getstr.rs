use core::ffi::c_char;

use crate::records::t_string::tstring;

/// cpp `lobject.h:306` `#define getstr(ts) (ts)->data` 对应。
///
/// # Safety
///
/// `ts` 必须指向存活的 `tstring`（取其 `data` 柔性数组首地址，返回值寿命随该字符串对象，
/// 可读界为 `len + 1` 字节——空串亦有 NUL 终止符）。
#[inline]
pub unsafe fn getstr(ts: *const tstring) -> *const c_char {
  // Safety: 契约保证 `ts` 指向存活 tstring，as_ptr 仅取柔性数组成员地址不解引用
  unsafe { (*ts).data.as_ptr() }
}
