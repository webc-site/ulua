//! 断言失败上报面：C 形入口 + 安全内部核心。
//!
//! 层定位（review.md §10）：本文件只有一个真 C 边界——宿主经
//! [`set_assert_handler`](crate::functions::assert_handler::set_assert_handler)
//! 注入的 `extern "C-unwind"` 处理器（指针参数即其 ABI 契约）。其余消费者
//! （`LUAU_ASSERT!` 宏经 [`assert_fail`] 的全仓展开、code-gen 的手工展开点）
//! 手里本就是宏生成的 NUL 结尾 `&[u8]`，故内部路径全程 `&[u8]`，
//! 不再要求调用点先铸成 `*const c_char`；指针↔切片的折算收口在
//! [`assert_report`] 一处（处理器调用前取 `as_ptr`，入口进核心前判空取串）。

use core::{ffi::c_char, ptr, str::from_utf8};

use crate::{
  functions::{assert_handler::assert_handler, c_str::cstr_bytes},
  macros::luau_noinline::LUAU_NOINLINE,
};

/// 断言上报的安全内部核心：`None` 即宿主 C ABI 的 null 哨兵，`Some(bytes)`
/// 须为在返回前存活的合法字符串（宏产物含结尾 NUL；入口折算串止于首个 NUL，
/// 两种形态对处理器都可读作同一 C 串）。
///
/// 返回值即 cpp `assertCallHandler` 的 `int`：非 0 表示调用方应继续触发断点，
/// 0 表示 host 处理器已自行接管本次断言。
fn assert_report(
  expression: Option<&[u8]>,
  file: Option<&[u8]>,
  line: i32,
  function: Option<&[u8]>,
) -> i32 {
  if let Some(handler) = assert_handler() {
    // Safety: 本文件唯一的宿主边界。`handler` 是宿主注册的 `unsafe fn`，其前置
    // 条件为「各实参为 NUL 结尾 C 字符串或 null」；上方 `Option` 契约保证每个
    // `Some` 切片的缓冲区在首个 NUL 处可读终止、`None` 原样译回 null 指针。
    return unsafe {
      handler(
        expression.map_or(ptr::null(), |bytes| bytes.as_ptr().cast()),
        file.map_or(ptr::null(), |bytes| bytes.as_ptr().cast()),
        line,
        function.map_or(ptr::null(), |bytes| bytes.as_ptr().cast()),
      )
    };
  }

  // 无自定义处理器：在 LUAU_DEBUGBREAK 触发进程中断前打印断言信息
  // （刻意增强：cpp Common.h:58-66 默认不打日志直接 return 1，这里补 stderr 输出便于诊断（DELIBERATE DEVIATION），将消息输出至 stderr）。
  // 若没有此输出，失败将直接变为静默的 int 3，难以诊断。
  #[cfg(feature = "std")]
  {
    // strict 解码语义（null / 非 UTF-8 → 占位文案）全程只读切片，无指针参与。
    fn display(opt: Option<&[u8]>) -> &str {
      opt.map_or("null", |bytes| from_utf8(bytes).unwrap_or("null"))
    }
    eprintln!(
      "LUAU_ASSERT failed: {} ({}:{})",
      display(expression),
      display(file),
      line
    );
  }

  1
}

LUAU_NOINLINE! {
    /// C 形入口（保留 cpp `assertCallHandler` 的签名镜像，供已处于指针形态的
    /// 手工展开点，如 code-gen 的调试构建路径调用）：判空后把三个指针折算成
    /// `Option<&[u8]>` 交给安全核心 [`assert_report`]，本函数自身不再触碰处理器。
    ///
    /// # Safety
    /// `expression`、`file` 和 `function` 必须是有效的以 nul 结尾的 C 字符串或 null。
    pub unsafe fn assert_call_handler(
        expression: *const c_char,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> i32 {
        // 判空折叠为 `Option`；非 null 串的读取收口到 c_str 门面（review.md §10）。
        // Safety: 前置条件（NUL 结尾缓冲）由本函数 `# Safety` 契约逐字转授
        // `cstr_bytes`；折算出的切片只借用调用方缓冲区、随核心在本次调用内消费。
        let to_opt = |p: *const c_char| {
            if p.is_null() {
              None
            } else {
              Some(unsafe { cstr_bytes(p) })
            }
        };
        assert_report(to_opt(expression), to_opt(file), line, to_opt(function))
    }
}

/// cpp `LUAU_ASSERT` 中 `expr` 为假的那一半：上报失败并回答"是否还要断点"。
///
/// `expression_with_nul` / `file_with_nul` 由宏以 `concat!(..., "\0")` 生成，
/// 带结尾 NUL。这一约定用 [`cbytes_or_invalid`] 校验后以 NUL 结尾字节切片
/// （不满足即回退 `b"<invalid expr>\0"` / `b"<invalid file>\0"` 占位常量）
/// 表达，直接交给安全核心——内部路径零指针、零 `unsafe`（review.md §10：
/// Rust-to-Rust 调用不经 C 形 API）。
///
/// 返回 `true` 等价 cpp `assertCallHandler(...) != 0`：无人接管，调用方应继续
/// `LUAU_DEBUGBREAK!()`；返回 `false` 表示 host 处理器已接管，不得再断点。
///
/// b28 裁定：必须保持 `pub`——`LUAU_ASSERT!` 宏体经 `$crate::…::assert_fail` 在
/// **下游 crate 展开**消费（token 面扫描看不见宏展开引用，#40 零消费点计数为假阳性）。
#[inline(never)]
#[cold]
pub fn assert_fail(expression_with_nul: &str, file_with_nul: &str, line: i32) -> bool {
  const INVALID_EXPR: &[u8] = b"<invalid expr>\0";
  const INVALID_FILE: &[u8] = b"<invalid file>\0";
  const UNKNOWN: &[u8] = b"unknown\0";
  assert_report(
    Some(cbytes_or_invalid(expression_with_nul, INVALID_EXPR)),
    Some(cbytes_or_invalid(file_with_nul, INVALID_FILE)),
    line,
    Some(UNKNOWN),
  ) != 0
}

/// 校验「以单个结尾 NUL 终止」的宏产物字节串（末字节为 0 且其前无其它 0），
/// 满足即返回含终止符的整段切片，不满足（正常不可达，只有宏被绕过手传串才会
/// 发生）时回退 `fallback`——两条分支都是合法的 NUL 结尾字节串。
fn cbytes_or_invalid<'a>(s: &'a str, fallback: &'static [u8]) -> &'a [u8] {
  let b = s.as_bytes();
  match b {
    [body @ .., 0] if !body.contains(&0) => b,
    _ => fallback,
  }
}
