//! C 字符串指针 → Rust 字符串的统一收口：与 [`crate::functions::c_slice`] 同族，
//! 把「先判 null 再 `CStr::from_ptr`」的手工守卫收敛到唯一一处，避免各 crate
//! 各自复制一份 lossy 解码包装。

use alloc::borrow::Cow;
use core::ffi::{CStr, c_char};

/// 以 NUL 结尾的 C 字符串 → `Cow<str>`（UTF-8 宽容解码）：合法 UTF-8 时零拷贝
/// 借用，仅在非法字节处分配替换串；null 哨兵按 cpp 的「空字符串」语义译成空串。
///
/// 为什么返回 `Cow<'a, str>` 而不是 `Cow<'static, str>`：借用分支的数据其实是
/// `p` 指向的宿主缓冲区，声明成 `'static` 是对调用方的谎报；这里把生命周期交回
/// 调用方按缓冲区真实存活期实例化。
///
/// # Safety
/// `p` 非 null 时必须指向以 NUL 结尾、且在返回值生命周期 `'a` 内持续有效的缓冲区。
pub unsafe fn cstr_cow<'a>(p: *const c_char) -> Cow<'a, str> {
  if p.is_null() {
    return Cow::Borrowed("");
  }
  // Safety: 前置条件保证 `p` 非空且指向 NUL 结尾缓冲区，`CStr::from_ptr` 因此
  // 落在一个以 NUL 终止的有效字节序列上；其内容存活期即调用方实例化的 `'a`。
  unsafe { CStr::from_ptr(p) }.to_string_lossy()
}

/// 以 NUL 结尾的 C 字符串 → 首个 NUL 前的字节切片（免解码的原字节）：null 哨兵
/// 按「空字符串」语义译成空切片，非 null 时与 cpp 把 `const char*` 直接交给
/// `std::string`/查表键的行为一致（NUL 本身不含在结果内）。
///
/// 与 [`cstr_cow`] 的差别只在有无 UTF-8 解码：字节版没有 lossy 分支，
/// 借用恒为合法字节序列，不存在需要分配替换串的情形，故直接返回 `&'a [u8]`
/// 而非 `Cow`（生命周期语义与 `cstr_cow` 相同，交由调用方按缓冲区真实存活期实例化）。
///
/// # Safety
/// `p` 非 null 时必须指向以 NUL 结尾、且在返回值生命周期 `'a` 内持续有效的缓冲区。
pub unsafe fn cstr_bytes<'a>(p: *const c_char) -> &'a [u8] {
  if p.is_null() {
    return &[];
  }
  // Safety: 前置条件保证 `p` 非空且指向 NUL 结尾缓冲区，`CStr::from_ptr` 落在
  // 有效字节序列上，`to_bytes` 返回其 NUL 前借用的视图（存活期即 `'a`）。
  unsafe { CStr::from_ptr(p) }.to_bytes()
}

/// Rust 字节串 → 瞬时 NUL 结尾收口：补一个尾部 NUL，把仅在闭包调用期内有效的
/// `*const c_char` 交给闭包。用于 callee 当场复制/驻留字符串的 `*const c_char`
/// 契约（如 `lua_setfield`/`lua_pushcclosure` 走 `lua_s_new` 入 intern 表），
/// 免在各调用点散落 `CString`。
///
/// # Safety 契约（由 callee 侧保证）
/// 传入指针不得被闭包保存或跨调用使用；callee 必须在闭包返回前完成复制/驻留。
#[inline]
pub fn with_c_str<R>(bytes: &[u8], f: impl FnOnce(*const c_char) -> R) -> R {
  use alloc::vec::Vec;

  let mut buf = Vec::with_capacity(bytes.len() + 1);
  buf.extend_from_slice(bytes);
  buf.push(0);
  // `buf` 以 NUL 结尾且在整个闭包调用期内存活。
  f(buf.as_ptr().cast())
}
