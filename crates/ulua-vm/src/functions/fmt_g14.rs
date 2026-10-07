use core::str::from_utf8;

/// C99 `snprintf("%.14g", n)` 语义的浮点转串（GC 堆枚举数字键边名，oracle
/// cpp/VM/src/lgcdebug.cpp:810）。
///
/// review.md §5 指定的 `zmij` 在此**不引入**（有意例外，§9.3）：本处需要的不是
/// 最短往返表示，而是 glibc `%.14g` 的排版规则——14 位有效数字截断 + 定点/科学
/// 自动切换 + 去尾零去尾点；两者对同一数值产出不同串（`1.23456789012345678e17`：
/// `%.14g` → `1.2345678901235e+17`，zmij 最短往返 → `123456789012345680`）。Lua
/// `tostring` 的最短表示路径另见 `luai_num_2_str.rs`（Schubfach），与本函数无关。
use crate::functions::fmt_cstr_buf::fmt_cstr_buf;

/// `%.14g` 的有效位数 P
const P: i32 = 14;
/// C 侧同款缓冲宽度（cpp lgcdebug.cpp:810 `char buf[32]`）
const BUF_MAX: usize = 32;

/// 就地去掉 NUL 结尾串小数部分尾零；小数点后至空再去掉小数点。
fn trim(buf: &mut [u8]) {
  let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
  if !buf[..end].contains(&b'.') {
    return;
  }
  let mut e = end;
  while e > 0 && buf[e - 1] == b'0' {
    e -= 1;
  }
  if e > 0 && buf[e - 1] == b'.' {
    e -= 1;
  }
  buf[e] = 0;
}

/// 把 `n` 按 C99 `%.14g` 排版写入 `buf`，NUL 结尾；`buf` 空则不写，写不下截断。
///
/// 算法：先取 `{:.13e}`（等价 C 的 `%.{P-1}e`，14 位有效数字，舍入后的首数指数
/// X 一并读出）定风格——`X < -4 || X >= P` 用科学计数，否则用定点 `{:.prec$}`
/// （`prec = P-1-X`）。两种 std 底座均为 IEEE 最近偶数舍入，与 glibc `printf`
/// 一致（tests/fmt_g14.rs 以 half-even 断言与真实 `printf` 输出对拍核实），本
/// 函数只叠「选风格 + 去尾零 + 去尾点 + 指数至少两位」的排版，不做手写长算术。
/// 特殊值对齐小写形态：`inf` / `-inf` / `nan`。
pub fn fmt_g14(buf: &mut [u8], n: f64) {
  if buf.is_empty() {
    return;
  }
  if n.is_nan() {
    fmt_cstr_buf(buf, format_args!("nan"));
    return;
  }
  if n.is_infinite() {
    fmt_cstr_buf(
      buf,
      format_args!("{}", if n > 0.0 { "inf" } else { "-inf" }),
    );
    return;
  }

  // 科学计数底座：顺带读出舍入后的十进制指数 X
  let mut sci = [0u8; BUF_MAX];
  fmt_cstr_buf(&mut sci, format_args!("{:.13e}", n));
  let s = &sci[..sci.iter().position(|&b| b == 0).unwrap_or(BUF_MAX)];
  // `{:.13e}` 恒含 'e'，且定长尾数形态上界 21 字节（符号 1 + 15 + 'e' + 指数
  // 符号 1 + 3，f64 指数绝对值 ≤ 308）< BUF_MAX，永不被 `sci` 截断掉 'e'。
  let epos = s.iter().position(|&b| b == b'e').unwrap();
  let (mant, exp) = (&s[..epos], &s[epos + 1..]);
  let neg = exp.first() == Some(&b'-');
  let digits = if neg || exp.first() == Some(&b'+') {
    &exp[1..]
  } else {
    exp
  };
  let x: i32 = digits
    .iter()
    .fold(0i32, |acc, &d| acc * 10 + i32::from(d - b'0'));
  let x = if neg { -x } else { x };

  if !(-4..P).contains(&x) {
    // 科学计数：mant 已是 14 位有效数字，去尾零/尾点后接 e±DD（至少两位）
    let mut mt = mant.len().min(buf.len() - 1);
    if mant[..mt].contains(&b'.') {
      while mt > 0 && mant[mt - 1] == b'0' {
        mt -= 1;
      }
      if mt > 0 && mant[mt - 1] == b'.' {
        mt -= 1;
      }
    }
    let mant_str = from_utf8(&mant[..mt]).unwrap_or("0");
    let sign = if x < 0 { '-' } else { '+' };
    fmt_cstr_buf(
      buf,
      format_args!("{mant_str}e{sign}{:02}", x.unsigned_abs()),
    );
  } else {
    let prec = (P - 1 - x) as usize;
    fmt_cstr_buf(buf, format_args!("{:.prec$}", n));
    trim(buf);
  }
}
