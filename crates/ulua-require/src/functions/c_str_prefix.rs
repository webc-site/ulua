use core::ffi::c_char;

use ulua_vm::{functions::lua_pushlstring::lua_pushlstring, records::lua_state::lua_State};

/// 真 FFI 边界收口时使用的栈内联缓冲大小：registry 键与 chunkname 通常远短于此，
/// 超长才退化为一次堆分配（cpp 侧 `std::string` 同样在超长时分配）。
const INLINE_BUFFER_SIZE: usize = 128;

/// 对应 cpp `std::string::c_str()` 的字节视图：按首个 NUL 截断。
/// 返回切片保证不含 NUL（Lua 字符串是字节串，非 UTF-8）。
pub(crate) fn c_str_prefix(s: &[u8]) -> &[u8] {
  let end = memchr::memchr(0, s).unwrap_or(s.len());
  &s[..end]
}

/// 内部字节串 → 真 FFI 边界（`*const c_char`）的唯一收口：补结尾 NUL 后调用 `f`，
/// 借用只在该调用内有效，故以闭包形式收口，指针无法逃逸。
///
/// 短串走栈内联缓冲（零堆分配），超长退化为一次 `Vec` 分配；含 NUL 时按首个 NUL
/// 截断，与 cpp 把 `std::string::c_str()` 交给 C 侧 `strlen` 的语义一致。
pub(crate) fn with_c_str<R>(s: &[u8], f: impl FnOnce(*const c_char) -> R) -> R {
  let s = c_str_prefix(s);

  if s.len() < INLINE_BUFFER_SIZE {
    // 定长缓冲恒以 0 结尾，只需覆写前 len 字节
    let mut buffer = [0u8; INLINE_BUFFER_SIZE];
    buffer[..s.len()].copy_from_slice(s);
    f(buffer.as_ptr().cast::<c_char>())
  } else {
    let mut owned = s.to_vec();
    owned.push(0);
    f(owned.as_ptr().cast::<c_char>())
  }
}

/// 对应 cpp `lua_pushstring(L, s.c_str())` 的零拷贝等价实现：按首个 NUL 截断后
/// 经 `lua_pushlstring` 推入（cpp `lua_pushstring` 内部同为 strlen + pushlstring），
/// 免去 NUL 补齐与 `CString` 分配。
///
/// # Safety
/// `l` 必须指向存活的 `lua_State`。
pub(crate) unsafe fn push_c_str(l: *mut lua_State, s: &[u8]) {
  let s = c_str_prefix(s);
  unsafe { lua_pushlstring(l, s.as_ptr().cast::<c_char>(), s.len()) };
}

/// cpp 的模块注册键归一：`std::tolower` 逐字节小写后 `lua_pushstring(c_str())`。
/// 无大写时零分配直推，有则转小写；与 `check_registered_modules` 共用同一形态，
/// 保证注册与查表两侧的键一致。
///
/// # Safety
/// `l` 必须指向存活的 `lua_State`。
pub(crate) unsafe fn push_lowered_c_str(l: *mut lua_State, s: &[u8]) {
  unsafe {
    let s = c_str_prefix(s);
    if s.iter().any(u8::is_ascii_uppercase) {
      let lowered = s.to_ascii_lowercase();
      lua_pushlstring(l, lowered.as_ptr().cast::<c_char>(), lowered.len());
    } else {
      lua_pushlstring(l, s.as_ptr().cast::<c_char>(), s.len());
    }
  }
}
