//! crate 内部共用的小工具函数。

use alloc::borrow::Cow;
use core::{
  cell::RefCell,
  ffi::{CStr, c_char},
  ptr::null,
};
use std::ffi::CString;

/// C 字符串 → `Cow<'static, str>`（UTF-8 宽容解码），null 视为空串。
///
/// 合法 UTF-8 时零拷贝借用，仅在非法 UTF-8 时分配。
///
/// # Safety
/// `p` 非 null 时必须指向 NUL 结尾的有效内存。
pub(crate) unsafe fn cstr_cow(p: *const c_char) -> Cow<'static, str> {
  if p.is_null() {
    return Cow::Borrowed("");
  }
  unsafe { CStr::from_ptr(p) }.to_string_lossy()
}

/// 把 `result` 存入 C 调用方的结果缓存并返回其指针，保证指针在调用返回后仍
/// 有效（镜像 C++ 函数内 `static std::string`）；空结果清空缓存并返回 null。
pub(crate) fn cache_result(cache: &RefCell<Option<CString>>, result: String) -> *const c_char {
  if result.is_empty() {
    *cache.borrow_mut() = None;
    return null();
  }
  let cstring = CString::new(result).unwrap_or_default();
  let ptr = cstring.as_ptr();
  *cache.borrow_mut() = Some(cstring);
  ptr
}
