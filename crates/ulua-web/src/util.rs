//! crate 内部共用的小工具函数与常量。

use alloc::borrow::Cow;
use core::{
  cell::RefCell,
  ffi::{CStr, c_char, c_int},
  ptr::null,
};
use std::ffi::CString;

/// VM 全局 `print` 的 C 名称（wasm 捕获版与 `run_code` 的结果打印共用）。
pub(crate) const PRINT_NAME: &CStr = c"print";

/// C++ `useNewSolver` 的 int 语义：0 = 旧求解器（`check` 系入口共用）。
pub(crate) const OLD_SOLVER_FLAG: c_int = 0;

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
  // C++ `std::string::c_str()` 读取到首个 NUL 为止：结果含内嵌 NUL 时保留首
  // 个 NUL 前的内容，与 C++ 观察行为一致（不能静默丢弃整段结果）。
  let cstring = CString::new(result).unwrap_or_else(|e| {
    let bytes = e.into_vec();
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    // 100% 安全：end 是首个 NUL 的下标，[..end] 不含 NUL，构造必然成功。
    CString::new(&bytes[..end]).unwrap_or_default()
  });
  let ptr = cstring.as_ptr();
  *cache.borrow_mut() = Some(cstring);
  ptr
}
