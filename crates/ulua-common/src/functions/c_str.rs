//! C 字符串指针 → Rust 字符串的统一收口：与 [`crate::functions::c_slice`] 同族，
//! 把「先判 null 再逐字节读宿主缓冲」的手工守卫收敛到唯一一处，避免各 crate
//! 各自复制一份 lossy 解码包装。
//!
//! **层定位标注（review.md §10）**：§10 的唯一豁免层是 `ulua-capi`（C ABI 实现层本身）。
//! 本模块不是「豁免清单」，而是把尚未 Rust 化的 `*const c_char` 位点**收口到唯一一处**的
//! 过渡门面：全仓读取宿主 NUL 结尾缓冲区只能经此，消费者不得自行解引用宿主缓冲、
//! 不得散落 `.as_ptr().cast()`。内部实现完全 Rust 化：NUL 扫描单点收口到私有
//! [`from_c_ptr`]，产出 `&[u8]`/`Cow<str>`（§10：C 串包装类型 `CStr`/`CString` 零残留，
//! 不借道标准库的 C 字符串垫片），读取方向两枚公开函数只剩判空形态与解码策略的差异。
//! 写入方向（静态字节串 / 动态字节串 → `*const c_char`）同样收口于本门面
//! （[`cstr`] / [`with_c_str`]）。
//!
//! ## 消费者普查（review.md §10 复审 · cstr-final-r5）
//!
//! 真 C 边界（`*const c_char` 属契约要求、长期保留）：仅 `ulua-capi`，以及宿主注入的
//! `extern "C-unwind"` 回调（如本 crate 的
//! [`assert_call_handler`](crate::functions::assert_call_handler::assert_call_handler)）。
//! `ulua-vm` 的 `lua_*` C 形态面（`lua_getinfo` 的 `what` 模板、`LuaDebug` 的
//! `*const c_char` 字段、`lua_pushcclosurek` 的 debugname、`lua_exception::what()`）
//! **不是 FFI 边界、不构成豁免**：它是 lua.h 的形状复刻，Rust 侧消费者一律应拿
//! `&[u8]`/`&str`/`Cow`，这些签名属待消灭对象（review.md §3「`c_char` 仅在 FFI」+ §10，
//! 与 vm 内部 `lua_*` 裸指针收形同批推进）。在其改完之前，跨 crate 进出其缓冲区仍必须
//! 经本门面，不得新增绕过路径。既有消费清单：ulua-rt（`debug_cstr` / `is_lua_what_cstr`
//! 读 `LuaDebug` 回填字段；`GETINFO_*` / `*_NAME` 静态模板走契约参数位）、ulua-web
//! （`cstr_cow` 读 `lua_exception::what()` 与 `ar.short_src`）、ulua-analysis /
//! ulua-require / ulua-repl-cli / ulua-code-gen / ulua-conformance 等 VM C API 消费者。
//!
//! 内部 Rust-to-Rust 路径不再借道 C 形 API：`assert_fail`（`LUAU_ASSERT!` 宏的
//! 全仓展开点）现直接把宏产物 NUL 结尾 `&[u8]` 交给安全核心，指针折算只发生在
//! 转发宿主处理器这一个真边界处；`wasm_libc` 的扫描面已完全 Rust 化、不再引用
//! 本门面。

use alloc::{borrow::Cow, string::String};
use core::{ffi::c_char, slice::from_raw_parts};

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
  // Safety: 前置条件担保缓冲区内必有终止 NUL：光标自 `start` 起逐字节推进，
  // 在首个 NUL 处停止、不越过缓冲区（libc `strlen` 语义：先定长、后取界内切片，
  // 与原借道标准库 C 串垫片 `from_ptr` 取字节的实现逐位等价）。`[start, end)` 即首个 NUL
  // 前的全部字节，均在调用方缓冲区内且存活期覆盖 `'a`，满足
  // `from_raw_parts` 的非空与范围内要求；`offset_from` 非负（`end >= start`），
  // `as usize` 无截断。
  unsafe {
    let start = p.cast::<u8>();
    let mut end = start;
    while *end != 0 {
      end = end.add(1);
    }
    Some(from_raw_parts(start, end.offset_from(start) as usize))
  }
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

/// `with_c_str` 栈缓冲容量：绝大多数 Lua 标识符 / 模块名 ≤ 127 字节，
/// 命中即走零堆分配的栈快路径。
const STACK_BUF_CAP: usize = 128;

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

  // 栈缓冲快路径：容量内零堆分配
  if bytes.len() < STACK_BUF_CAP {
    let mut buf = [0u8; STACK_BUF_CAP];
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
