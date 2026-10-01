use core::{ffi::c_char, ops::ControlFlow, slice::from_raw_parts};

use crate::{functions::cstr_bytes, macros::lua_l_error::luaL_error, records::header::Header};

/// 上游 Luau MAXSSIZE：((INT_MAX >> 1) + 1)，即 1073741824
const MAX_SSIZE: i32 = 1073741824;

/// cpp `getnum` 的累加续行上限：(INT_MAX - 9) / 10；先比对后累加，
/// 保证下一次 `a * 10 + d` 不溢出 i32。
const MAX_ACC: i32 = (i32::MAX - 9) / 10;

/// 纯 safe 的 Rust 切片数字解析函数。
///
/// 从 `bytes` 开头解析十进制整数，支持指定默认值 `df`。
/// 返回 `Ok((parsed_value, bytes_consumed))`。若数值超出上限则返回 `Err("size specifier is too large")`。
#[inline]
pub fn parse_getnum_bytes(bytes: &[u8], df: i32) -> Result<(i32, usize), &'static str> {
  let Some(&first) = bytes.first() else {
    return Ok((df, 0));
  };

  if !first.is_ascii_digit() {
    return Ok((df, 0));
  }

  // cpp do-while 的 `digit(**fmt)` 止停条件收为数字前缀切片
  let digits = &bytes[..bytes.iter().take_while(|&&b| b.is_ascii_digit()).count()];

  // cpp `a = a*10 + d` 折叠为 try_fold：MAX_ACC 卫兵对应续行条件
  // `a <= (INT_MAX - 9) / 10`（先比对后累加故不溢出）；(值, 已消耗位数)
  // 随折行携带，游标推进与"残留数字即溢出"判定共用同一计数
  let (a, consumed) =
    match digits
      .iter()
      .map(|&b| i32::from(b - b'0'))
      .try_fold((0_i32, 0_usize), |(a, n), d| {
        if a > MAX_ACC {
          ControlFlow::Break((a, n))
        } else {
          ControlFlow::Continue((a * 10 + d, n + 1))
        }
      }) {
      ControlFlow::Break(s) | ControlFlow::Continue(s) => s,
    };

  // cpp `if (a > MAXSSIZE || digit(**fmt))`：残留数字 ⟺ 数字前缀未消化完
  if a > MAX_SSIZE || consumed < digits.len() {
    return Err("size specifier is too large");
  }

  Ok((a, consumed))
}

/// 数字字面量扫描上限：足够长以区分正常数字与溢出（超出交由
/// `parse_getnum_bytes` 的溢出检查报错）
const MAX_DIGIT_SCAN: usize = 32;

/// 格式串读取游标（C++ `const char** fmt` 的切片/索引形态替代）。
///
/// `bytes` 覆盖格式串 payload 及其终止 NUL（长度 = strlen + 1），与 C++ 游标
/// 可读到终止符的点位同构；越界读经 `at` 归一为 NUL(0)。外层入口以
/// `cur() != 0` 钳位，pos 恒不超过串尾。
pub(crate) struct FmtCursor<'a> {
  bytes: &'a [u8],
  pos: usize,
}

impl<'a> FmtCursor<'a> {
  /// # Safety
  /// `p` 必须指向可读且 NUL 终止的串数据（`luaL_checkstring` 检出的格式串 payload）。
  pub(crate) unsafe fn from_ptr(p: *const c_char) -> Self {
    // SAFETY: 契约保证 p 串身至终止 NUL 可读
    let len = unsafe { cstr_bytes(p) }.len();
    // SAFETY: 同上，p[..=len] 覆盖 payload 与终止 NUL
    let bytes = unsafe { from_raw_parts(p as *const u8, len + 1) };
    Self { bytes, pos: 0 }
  }

  /// 游标当前字节；位于终止位时返回 0（等价 C 读终止 NUL）。
  #[inline]
  pub(crate) fn cur(&self) -> u8 {
    self.at(0)
  }

  /// 游标后偏移 `off` 处字节；串尾外一律视为终止 NUL(0)。
  #[inline]
  pub(crate) fn at(&self, off: usize) -> u8 {
    *self.bytes.get(self.pos + off).unwrap_or(&0)
  }

  /// 游标前进一位（C++ `(*fmt)++`）。
  #[inline]
  pub(crate) fn bump(&mut self) {
    self.pos += 1;
  }

  /// 自游标起长度 `len` 的有界前缀切片（数字扫描结果，恒在界内）。
  #[inline]
  fn head(&self, len: usize) -> &[u8] {
    &self.bytes[self.pos..self.pos + len]
  }
}

/// 前置条件（由 `str_format` 建立的 `Header` 对象不变式承载，非调用方裸指针契约）：
/// `h.l` 为存活 `lua_State`；`fmt` 游标位于格式串界内（越尾经 `at` 归一为 NUL）。
/// 数字扫描与游标推进均为切片/整数运算，故签名安全；`unsafe` 只留在下方报错内核。
pub(crate) fn getnum(h: &mut Header, fmt: &mut FmtCursor, df: i32) -> i32 {
  // 扫描连续数字字节，上限 MAX_DIGIT_SCAN 足以判断正常数字与溢出；
  // 原 `fmt_ptr.is_null()` 早退分支针对 checkstring 产物恒不可达，随游标索引化移除
  let len = (0..MAX_DIGIT_SCAN)
    .take_while(|&k| fmt.at(k).is_ascii_digit())
    .count();

  match parse_getnum_bytes(fmt.head(len), df) {
    Ok((val, consumed)) => {
      fmt.pos += consumed;
      val
    }
    // SAFETY: 前置条件保证 h.l 存活可抛错；luaL_error 不返回（longjmp 语义）
    Err(err) => unsafe { luaL_error!(h.l, "{}", err) },
  }
}
