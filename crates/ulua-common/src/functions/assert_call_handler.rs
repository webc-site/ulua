use core::{ffi::c_char, str::from_utf8};

#[cfg(feature = "std")]
use crate::functions::c_str::cstr_bytes;
use crate::{functions::assert_handler::assert_handler, macros::luau_noinline::LUAU_NOINLINE};

/// C 字符串 → `&str`：NULL 或非 UTF-8 时回退 `fallback`。
///
/// # Safety
/// `p` 非空时必须指向以 NUL 结尾的缓冲区。
#[cfg(feature = "std")]
unsafe fn display_cstr(p: *const c_char, fallback: &str) -> &str {
  // strict 解码语义（非 UTF-8 → fallback）与 cstr_cow 的 lossy 译法不等价，
  // 故判空分支与 from_utf8 保留于此，只把指针读取收口到 cstr_bytes 门面。
  // Safety: 本函数前置条件保证 `p` 非空时指向以 NUL 结尾的缓冲区；先判空——
  // `p` 为空则直接返回 fallback，仅当非空才读指针，满足 cstr_bytes 的前置条件。
  if p.is_null() {
    return fallback;
  }
  from_utf8(unsafe { cstr_bytes(p) }).unwrap_or(fallback)
}

LUAU_NOINLINE! {
    /// # Safety
    /// `expression`、`file` 和 `function` 必须是有效的以 nul 结尾的 C 字符串或 null。
    ///
    /// 返回值即 cpp `assertCallHandler` 的 `int`：非 0 表示调用方应继续触发断点，
    /// 0 表示 host 处理器已自行接管本次断言（`LUAU_ASSERT` 据此跳过 debugbreak）。
    pub unsafe fn assert_call_handler(
        expression: *const c_char,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> i32 {
        if let Some(handler) = assert_handler() {
            // Safety: `handler` 是宿主注册的 `unsafe fn`，其前置条件与本函数一致
            // （三个 `*const c_char` 为 NUL 结尾 C 字符串或 null）；此处原样转发本函数入参，
            // 前提由本函数的调用方（见 `assert_fail` 及 `LUAU_ASSERT` 宏）保证。
            unsafe {
                return handler(expression, file, line, function);
            }
        }

        // 无自定义处理器：在 LUAU_DEBUGBREAK 触发进程中断前打印断言信息
        // （刻意增强：cpp Common.h:58-66 默认不打日志直接 return 1，这里补 stderr 输出便于诊断（DELIBERATE DEVIATION），将消息输出至 stderr）。
        // 若没有此输出，失败将直接变为静默的 int 3，难以诊断。
        #[cfg(feature = "std")]
        unsafe {
            // Safety: 本函数安全前置条件保证指针为 NUL 结尾 C 字符串或 null。
            let expr = display_cstr(expression, "null");
            let f = display_cstr(file, "null");
            eprintln!("LUAU_ASSERT failed: {} ({}:{})", expr, f, line);
        }

        1
    }
}

/// cpp `LUAU_ASSERT` 中 `expr` 为假的那一半：上报失败并回答"是否还要断点"。
///
/// `expression_with_nul` / `file_with_nul` 由宏以 `concat!(..., "\0")` 生成，
/// 带结尾 NUL。这一约定用 [`cbytes_or_invalid`] 校验后以 NUL 结尾字节切片
/// （不满足即回退 `b"<invalid expr>\0"` / `b"<invalid file>\0"` 占位常量）
/// 表达，指针位点只剩 `.as_ptr().cast()`，
/// 不再引入 `CStr` 类型。
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
  let expr = cbytes_or_invalid(expression_with_nul, INVALID_EXPR);
  let file = cbytes_or_invalid(file_with_nul, INVALID_FILE);
  // Safety: 上面两步与 `UNKNOWN` 常量保证三个实参都是合法的 NUL 结尾 C 字符串。
  unsafe {
    assert_call_handler(
      expr.as_ptr().cast(),
      file.as_ptr().cast(),
      line,
      UNKNOWN.as_ptr().cast(),
    ) != 0
  }
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
