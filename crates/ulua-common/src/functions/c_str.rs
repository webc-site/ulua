//! C 字符串指针 → Rust 字符串的统一收口：与 [`crate::functions::c_slice`] 同族，
//! 把「先判 null 再逐字节读宿主缓冲」的手工守卫收敛到唯一一处，避免各 crate
//! 各自复制一份 lossy 解码包装。
//!
//! **层定位标注（review.md §10）**：本模块的存在意义是支撑 `ulua-capi` / `ulua-vm`
//! 的 C 字符串层——它是全仓读取宿主 NUL 结尾缓冲区的唯一合法门面，入参形态
//! `*const c_char` 属 C ABI 边界契约、予以保留（各消费者一律经此转 Rust 类型，
//! 不得自行解引用宿主缓冲）。内部实现完全 Rust 化：NUL 扫描单点收口到私有
//! [`from_c_ptr`]（原 C 串读取构造的零类型替代），读取方向两枚公开函数只剩
//! 判空形态与解码策略的差异。写入方向（静态字节串 / 动态字节串 → `*const c_char`）
//! 同样收口于本门面（[`cstr`] / [`with_c_str`]），消费者不得散落 `.as_ptr().cast()`。
//!
//! ## 消费者普查（review.md §10 复审 · cstr-final-r5）
//!
//! 真 C 边界（豁免保留）：`ulua-capi`（C ABI 实现层本身）与 `ulua-vm`——后者是
//! C 库移植，其 `lua_*` API 面即 lua.h 契约（`lua_getinfo` 的 `what` 模板、
//! `lua_Debug` 的 `*const c_char` 字段、`lua_pushcclosurek` 的 debugname、
//! `lua_exception::what()`），跨 crate 消费者必须经本门面进出其缓冲区。
//! 合法消费清单：ulua-rt（`debug_cstr` / `is_lua_what_cstr` 读 `LuaDebug` 回填
//! 字段；`GETINFO_*` / `*_NAME` 静态模板走契约参数位）、ulua-web（`cstr_cow` 读
//! `lua_exception::what()` 与 `ar.short_src`）、ulua-analysis / ulua-require /
//! ulua-repl-cli / ulua-code-gen / ulua-conformance 等 VM C API 消费者、以及
//! ulua-common 自身的 [`assert_call_handler`](crate::functions::assert_call_handler::assert_call_handler)
//! C 形入口（宿主注入的 `extern "C-unwind"` `AssertHandler` 即真边界）。
//!
//! 内部 Rust-to-Rust 路径不再借道 C 形 API：`assert_fail`（`LUAU_ASSERT!` 宏的
//! 全仓展开点）现直接把宏产物 NUL 结尾 `&[u8]` 交给安全核心，指针折算只发生在
//! 转发宿主处理器这一个真边界处；`wasm_libc` 的扫描面已完全 Rust 化、不再引用
//! 本门面。

use alloc::{borrow::Cow, string::String};
use core::ffi::{CStr, c_char};

/// 判空 + 单点 NUL 扫描：null 哨兵按 cpp「空字符串」语义译成 `None`，
/// 由调用门面各按自身的空值形态承接（`Cow::Borrowed("")` / 空切片）。
///
/// # Safety
/// `p` 非 null 时必须指向以 NUL 结尾、且在返回值生命周期 `'a` 内持续有效的缓冲区。
#[inline]
unsafe fn from_c_ptr<'a>(p: *const c_char) -> Option<&'a [u8]> {
  if p.is_null() {
    return None;
  }
  // Safety: 前置条件保证 `p` 非空且指向 NUL 结尾缓冲区，[`CStr::from_ptr`] 在首个
  // NUL 处截断读取（libc `strlen` 语义），[`CStr::to_bytes`] 返回不含终止符的字节
  // 切片、存活期即调用方实例化的 `'a`——与原手工逐字节扫描 + `from_raw_parts` 逐位等价。
  Some(unsafe { CStr::from_ptr(p) }.to_bytes())
}

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
  // Safety: 前置条件逐字转授 [`from_c_ptr`] 的同款契约。
  match unsafe { from_c_ptr(p) } {
    Some(bytes) => String::from_utf8_lossy(bytes),
    None => Cow::Borrowed(""),
  }
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
  // Safety: 前置条件逐字转授 [`from_c_ptr`] 的同款契约。
  unsafe { from_c_ptr(p) }.unwrap_or(&[])
}

/// 静态 NUL 结尾字节串字面量（`b"name\0"`）→ `*const c_char` 的唯一收口（写入方向，
/// 与读取方向的 [`cstr_cow`] / [`cstr_bytes`] 对偶）：测试与宿主代码一律用
/// `b"..\0"` 字节串，只在调用 `*const c_char` 契约 API 的收口点经此转裸指针，
/// 不散落 `.as_ptr().cast()` 逐点 cast（review.md §10）。
///
/// `bytes` 须为以 NUL 结尾的静态字节串（`b"name\0"` 字面量即满足）；返回指针
/// 随 `'static` 缓冲区存活，满足 C 侧对参数存活期的要求。
#[inline]
pub fn cstr(bytes: &'static [u8]) -> *const c_char {
  bytes.as_ptr().cast()
}

/// Rust 字节串 → 瞬时 NUL 结尾收口：补一个尾部 NUL，把仅在闭包调用期内有效的
/// `*const c_char` 交给闭包。用于 callee 当场复制/驻留字符串的 `*const c_char`
/// 契约（如 `lua_setfield`/`lua_pushcclosure` 走 `lua_s_new` 入 intern 表），
/// 免在各调用点散落堆分配的 NUL 结尾缓冲。
///
/// # Safety 契约（由 callee 侧保证）
/// 传入指针不得被闭包保存或跨调用使用；callee 必须在闭包返回前完成复制/驻留。
#[inline]
pub fn with_c_str<R>(bytes: &[u8], f: impl FnOnce(*const c_char) -> R) -> R {
  // 零拷贝快路径：输入已自带 NUL 终止符
  if bytes.last().copied() == Some(0) {
    return f(bytes.as_ptr().cast());
  }

  // 栈缓冲快路径：绝大多数 Lua 标识符 / 模块名 ≤ 127 字节，零堆分配
  if bytes.len() < 128 {
    let mut buf = [0u8; 128];
    buf[..bytes.len()].copy_from_slice(bytes);
    buf[bytes.len()] = 0;
    f(buf.as_ptr().cast())
  } else {
    use alloc::vec::Vec;

    let mut buf = Vec::with_capacity(bytes.len() + 1);
    buf.extend_from_slice(bytes);
    buf.push(0);
    // `buf` 以 NUL 结尾且在整个闭包调用期内存活。
    f(buf.as_ptr().cast())
  }
}
