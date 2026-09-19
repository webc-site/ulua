//! crate 内部共用的小工具函数与常量。

use core::{
  cell::RefCell,
  ffi::{CStr, c_char, c_int},
  ptr::null,
  slice,
};
use std::{borrow::Cow, ffi::CString, string::String};

use ulua_vm::macros::lua_memerrmsg::LUA_MEMERRMSG;

/// VM 全局 `print` 的 C 名称（wasm 捕获版与 `run_code` 的结果打印共用）。
pub(crate) const PRINT_NAME: &CStr = c"print";

/// C ABI `checkScript(source, useNewSolver)` 的 int 语义：0 = 旧求解器。
/// 仅在 C 入口边界解释 `c_int`，内部一律用 `bool`。
pub(crate) const OLD_SOLVER_FLAG: c_int = 0;

/// `luaL_newstate` / `lua_newthread` 分配失败（内存耗尽）时回报宿主的错误文本；
/// VM 状态为 null 时不得继续解引用。直接复用 VM 的内存错误常量，避免同一字面量
/// 在多处漂移。
pub(crate) const NOT_ENOUGH_MEMORY: &CStr = LUA_MEMERRMSG;

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

/// `lua_tolstring` / `luaL_tolstring` 的（指针, 长度）出参 → `String`（UTF-8 宽容
/// 解码），对应 cpp 的 `std::string(msg, len)`：按长度取全部字节，内嵌 NUL 不截
/// 断；null（索引无效/非字符串）视为空串。
///
/// # Safety
/// `p` 非 null 时 `p..p+len` 必须是由 VM 保证有效的字符串字节，且在弹出该栈槽
/// 之前读取。
pub(crate) unsafe fn lua_str_to_string(p: *const c_char, len: usize) -> String {
  if p.is_null() {
    return String::new();
  }
  // SAFETY: 契约保证 p..p+len 有效。
  let bytes = unsafe { slice::from_raw_parts(p.cast::<u8>(), len) };
  String::from_utf8_lossy(bytes).into_owned()
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
