use core::{ffi::c_char, slice::from_raw_parts};

use crate::{functions::cstr_bytes, macros::lua_l_error::luaL_error, records::header::Header};

/// 上游 Luau MAXSSIZE：((INT_MAX >> 1) + 1)，即 1073741824
const MAX_SSIZE: i32 = 1073741824;

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

  let mut a: i32 = 0;
  let mut idx = 0;

  loop {
    let b = bytes[idx];
    let digit_val = (b - b'0') as i32;
    a = a * 10 + digit_val;
    idx += 1;

    if idx == bytes.len() || !bytes[idx].is_ascii_digit() || a > (i32::MAX - 9) / 10 {
      break;
    }
  }

  if a > MAX_SSIZE || (idx < bytes.len() && bytes[idx].is_ascii_digit()) {
    return Err("size specifier is too large");
  }

  Ok((a, idx))
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
    // Safety: 契约保证 p 串身至终止 NUL 可读
    let len = unsafe { cstr_bytes(p) }.len();
    // Safety: 同上，p[..=len] 覆盖 payload 与终止 NUL
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

/// # Safety
///
/// `h.l` 为存活 `lua_State`；`fmt` 游标位于格式串界内（越尾经 `at` 归一为 NUL）。
pub(crate) unsafe fn getnum(h: &mut Header, fmt: &mut FmtCursor, df: i32) -> i32 {
  // 扫描连续数字字节，上限 MAX_DIGIT_SCAN 足以判断正常数字与溢出；
  // 原 `fmt_ptr.is_null()` 早退分支针对 checkstring 产物恒不可达，随游标索引化移除
  let mut len = 0;
  while len < MAX_DIGIT_SCAN && fmt.at(len).is_ascii_digit() {
    len += 1;
  }

  match parse_getnum_bytes(fmt.head(len), df) {
    Ok((val, consumed)) => {
      fmt.pos += consumed;
      val
    }
    // Safety: 契约保证 h.l 存活；luaL_error 不返回（longjmp 语义）
    Err(err) => unsafe { luaL_error!(h.l, "{}", err) },
  }
}
