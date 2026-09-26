//! 纯 Rust `strtoull`（u64 整数解析），与 `strtod_shim` 同一暴露模式。
//!
//! VM 的 `luaO_str2d` / `luaO_str2l` / `luaB_tonumber` 原先经
//! `unsafe extern "C"` 绑定 libc `strtoul`/`strtoull`：native 走平台 libc，
//! wasm 走 `strtod_shim` 侧的 `#[no_mangle]` 符号垫片（strto* 族历来归属该
//! 模块，不在 `wasm_libc`——后者只垫 malloc/mmap/gmtime_r 等非解析面符号），
//! 两套语义有漂移（溢出处理不一致）。此处提供全平台统一的单次遍历状态机，
//! 语义对齐 C 标准 + glibc/Darwin 实现：
//!
//! - 跳前导空白（` \t\n\v\f\r`）→ 可选 `+`/`-`（负号按模 2^64 取反）→
//!   `base == 0` 自动探测（`0x` 前缀→16，前导 `0`→8，否则 10）；`base == 16`
//!   识别可选 `0x`/`0X` 前缀 → 按进制消费数字。
//! - `0x` 后无十六进制数字时只消费 `0`（POSIX 最长有效前缀规则）。
//! - 溢出：C 返回 `ULLONG_MAX` 并置 `errno = ERANGE`。三个调用方均不读
//!   `errno`，故不模拟 errno，仅保留饱和返回值 `u64::MAX`（该值会传播到
//!   `tonumber` 的返回，与 native libc 一致）。
//! - 无转换：返回 0 且 `endptr == nptr`。
//!
//! 扫描核心 [`crate::strtoull_shim::parse_c_ull`]（u64 出口）/
//! [`crate::strtoull_shim::parse_c_ll`]（i64 出口，对应
//! `strtoll`）共享同一状态机，两者的调用方（`luaO_str2d` / `luaO_str2l`）直接
//! 以字节切片取用；只有 `luaB_tonumber` 仍按 C 形态调用，保留 C 指针入口
//! [`crate::strtoull_shim::rust_strtoull`]（`strtod_shim` 侧的同型 C 垫片已于
//! b22 退役——该符号在全仓零 extern 消费后不再需要）。

use core::{ffi::c_char, ptr::null_mut};

use crate::functions::{c_str::cstr_bytes, is_c_space::is_c_space};

/// 单字符 → 数字值（`a`/`A`→10 …），非字母数字返回 `None`；
/// 是否为当前进制的合法数字由调用方按 `radix` 过滤。
#[inline]
fn digit_val(c: u8) -> Option<u64> {
  match c {
    b'0'..=b'9' => Some(u64::from(c - b'0')),
    b'a'..=b'z' => Some(u64::from(c - b'a') + 10),
    b'A'..=b'Z' => Some(u64::from(c - b'A') + 10),
    _ => None,
  }
}

/// 扫描中间结果：无符号累积值、u64 域溢出标志、符号、消费字节数
/// （含前导空白 + 符号 + 前缀 + 数字，即 C `endptr - nptr`）。
/// 供 u64/i64 两个出口按各自的 C 饱和域组装。
struct Scan {
  val: u64,
  overflow: bool,
  neg: bool,
  consumed: usize,
}

/// `strtoull`/`strtoll` 共用的单次遍历状态机：
/// 空白 → 符号 → `0x` 前缀/进制探测 → 数字累积。
/// 溢出后继续消费数字（保持 endptr 前进）但停止累积。
/// 无数字（`None`）表示"无转换"（C 置 `endptr == nptr` 且返回 0）。
fn scan(b: &[u8], base: u32) -> Option<Scan> {
  if base != 0 && !(2..=36).contains(&base) {
    return None; // C：不支持的 base，无转换
  }

  let ws = b.iter().position(|&c| !is_c_space(c)).unwrap_or(b.len());
  let rest = &b[ws..];
  let (mut i, neg) = match rest.first() {
    Some(b'-') => (1, true),
    Some(b'+') => (1, false),
    _ => (0, false),
  };

  // `0x`/`0X` 仅在 base 0/16 识别，且后随十六进制数字才算前缀；
  // 否则 "0x" 只消费 "0"（如 "0x"、"0xg"，endptr 停在 'x'）。
  let hex_prefix = (base == 0 || base == 16)
    && matches!(rest.get(i), Some(b'0'))
    && matches!(rest.get(i + 1), Some(b'x' | b'X'))
    && rest.get(i + 2).is_some_and(u8::is_ascii_hexdigit);
  let mut radix = base;
  if hex_prefix {
    i += 2;
    radix = 16;
  } else if base == 0 {
    radix = if matches!(rest.get(i), Some(b'0')) {
      8
    } else {
      10
    };
  }
  let radix = u64::from(radix);

  let dig_start = i;
  let mut acc: u64 = 0;
  let mut overflow = false;
  while let Some(&c) = rest.get(i) {
    let Some(d) = digit_val(c).filter(|&d| d < radix) else {
      break;
    };
    i += 1;
    if !overflow {
      match acc.checked_mul(radix).and_then(|v| v.checked_add(d)) {
        Some(v) => acc = v,
        // C 饱和语义：继续消费数字（endptr 前进）但停止累积。
        None => overflow = true,
      }
    }
  }

  if i == dig_start {
    return None; // 无数字 → 无转换 → endptr == nptr
  }
  Some(Scan {
    val: acc,
    overflow,
    neg,
    consumed: ws + i,
  })
}

/// 从 `b`（NUL 结尾 C 字符串的字节，不含 NUL）扫描前导 C 无符号整数。
/// 返回 `(value, consumed)`，`consumed == 0` 表示"无转换"
/// （C `strtoull` 置 `endptr == nptr`）。
///
/// `base`：`0` 自动探测进制，`2..=36` 指定进制，其它值无转换（对应 C 的
/// `EINVAL` 路径，调用方已用 `luaL_argcheck` 保证 2..=36，此处兜底）。
pub fn parse_c_ull(b: &[u8], base: u32) -> (u64, usize) {
  let Some(s) = scan(b, base) else {
    return (0, 0);
  };
  let val = if s.overflow {
    u64::MAX // C：ERANGE 时返回 ULLONG_MAX
  } else if s.neg {
    s.val.wrapping_neg() // C：负号在无符号域按模 2^64 取反
  } else {
    s.val
  };
  (val, s.consumed)
}

/// 同 [`parse_c_ull`]，但有符号出口（C `strtoll`）：负值合法域
/// `[-2^63, 2^63)`，正溢出饱和 `i64::MAX`，负溢出饱和 `i64::MIN`
/// （C ERANGE 返回 `LLONG_MAX`/`LLONG_MIN`）。
pub fn parse_c_ll(b: &[u8], base: u32) -> (i64, usize) {
  let Some(s) = scan(b, base) else {
    return (0, 0);
  };
  let val = if s.overflow {
    if s.neg { i64::MIN } else { i64::MAX }
  } else if s.neg {
    // 2^63 是合法的 -2^63；超出则下溢饱和 i64::MIN
    if s.val > 1 << 63 {
      i64::MIN
    } else {
      s.val.wrapping_neg() as i64
    }
  } else if s.val > i64::MAX as u64 {
    i64::MAX
  } else {
    s.val as i64
  };
  (val, s.consumed)
}

/// 纯 Rust `strtoull`：C ABI 形态（`nptr` + `endptr` 出参），供仍按 C 形书写的
/// VM 移植代码（`luaB_tonumber`）取代 libc FFI；其余调用方（`luaO_str2d` /
/// `luaO_str2l`）直接用切片入口 [`parse_c_ull`] / [`parse_c_ll`]。语义见
/// [`parse_c_ull`]：`*endptr` 指向第一个未消费字节，无转换时指回 `nptr`。
///
/// 移除前提（在案专批）：待 ulua-vm 的 `lua_b_tonumber` 从本 C 指针入口切到
/// [`parse_c_ull`] 切片口后，此入口随唯一调用方一并退役。
///
/// # Safety
/// `s` 非空时必须指向以 NUL 结尾的缓冲区；`endptr` 必须可写。`s == null`
/// 为合法输入（libc 语义：仅写 `*endptr = null` 即返回，不解引用 `s`）。
pub unsafe fn rust_strtoull(s: *const c_char, endptr: &mut *mut c_char, base: u32) -> u64 {
  // Safety: 前置条件保证 `s` 指向以 NUL 结尾的缓冲区、`&mut *endptr` 可写。`s` 为空时仅写
  // `*endptr = null_mut()`（endptr 由契约保证可写）即返回，不解引用 `s`；非空时 `CStr::from_ptr(s)`
  // 因 NUL 结尾而合法。`parse_c_ull` 至多消费到 NUL 之前的字节，故 `consumed ≤ strlen(s)`，
  // `(s as *mut c_char).add(consumed)` 落在缓冲区界内（指向 NUL 或其前一字节），写 endptr 合法。
  // 保留空指针：本函数是 libc `strtoull` 的 C ABI 替身，nptr 为 NULL 时把 endptr
  // 写成空指针即 glibc/Darwin 的逐位行为，endptr 的类型（*mut c_char）由调用方 C 形态决定。
  unsafe {
    if s.is_null() {
      *endptr = null_mut();
      return 0;
    }
    let (val, consumed) = parse_c_ull(cstr_bytes(s), base);
    // consumed == 0 ⇒ 无转换 ⇒ endptr == nptr（C 语义）
    *endptr = (s as *mut c_char).add(consumed);
    val
  }
}
