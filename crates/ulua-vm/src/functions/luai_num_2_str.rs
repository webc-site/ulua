use alloc::{string::String, vec::Vec};

use ulua_common::macros::luau_assert::{LUAU_ASSERT, LUAU_UNLIKELY};

use crate::{
  functions::{
    printexp::printexp, printspecial::printspecial, printunsignedrev::printunsignedrev,
    schubfach::schubfach, trimzero::trimzero,
  },
  records::decimal::Decimal,
};

/// IEEE-754 双精度：11 位指数掩码
const EXPONENT_MASK: i32 = 0x7ff;
/// IEEE-754 双精度：52 位尾数掩码
const FRACTION_MASK: u64 = (1u64 << 52) - 1;
/// Schubfach 有效数字上界 10^17（significand 至多 17 位）
const MAX_SIGNIFICAND: u64 = 100_000_000_000_000_000;
/// schubfach 有效数字最多 17 位十进制
const MAX_DECIMAL_DIGITS: usize = 17;

/// cpp `lua_numtostr.cpp:luai_num2str`：把浮点 `n` 的 Lua 最短十进制表示写入
/// `buf`，返回写出长度（不含 NUL）。
///
/// 前置条件：`buf.len() >= LUAI_MAXNUM2STR`（转串输出上界），不满足时按越界
/// panic（原指针版为 UB），vm 内调用点均以定长数组满足，capi 壳侧以 `# Safety`
/// 契约承接。
///
/// review.md §5 的 `zmij` 在此**不引入**（有意例外，故写明理由）：本模块是 cpp
/// `VM/src/lnumprint.cpp` 的 Schubfach 移植，与 zmij 同属 Schubfach/Dragonbox
/// 一族的最短表示算法，性能量级相同（查表 + 单次 128 位乘法），换成外部库没有
/// 算法收益；而 §0 要求输出与 cpp oracle 逐字节一致，`trimzero`/`printexp` 的
/// 尾零截断、指数补零、`E` 大小写、尾点省略都是可观测行为（`string.format` 的
/// `%.14g` 是另一条 C 排版路径，见 `enumtable.rs`，与本函数无关）。外部打印器
/// 只会替换掉这一整套已对齐的排版细节，收益为零、风险是行为漂移。
///
/// pub 面：`ulua-capi` 的 `ulua_luai_num2str` 显式壳在 FFI 边界把 `*mut c_char`
/// 重建为本切片后转调本函数（vm 内部消费方均直接按 `&mut [u8]` 缓冲调用）。
pub fn luai_num2str_buf(buf: &mut [u8], n: f64) -> usize {
  // IEEE-754
  let bits = n.to_bits();
  let sign = (bits >> 63) as usize;
  let exponent = ((bits >> 52) & EXPONENT_MASK as u64) as i32;
  let fraction = bits & FRACTION_MASK;

  // specials
  if LUAU_UNLIKELY!(exponent == EXPONENT_MASK) {
    return printspecial(buf, sign as i32, fraction);
  }

  // sign bit
  buf[0] = b'-';
  let mut pos = sign; // 正数跳过占位的 '-'

  // zero
  if exponent == 0 && fraction == 0 {
    buf[pos] = b'0';
    return pos + 1;
  }

  // convert binary to decimal using Schubfach
  let d: Decimal = schubfach(exponent, fraction);
  LUAU_ASSERT!(d.s < MAX_SIGNIFICAND);

  // print the decimal to a temporary buffer; we'll need to insert the decimal
  // point and figure out the format（右对齐回写，返回首位数字下标）
  let mut decbuf = [0u8; MAX_DECIMAL_DIGITS + 3];
  let dstart = printunsignedrev(&mut decbuf, d.s);
  let dec = &decbuf[dstart..];
  let declen = dec.len();
  LUAU_ASSERT!((declen as i32) <= 17);

  let dot = declen as i32 + d.k;

  // the limits are somewhat arbitrary but changing them may require changing
  // the buffer bound contract above
  if (-5..=21).contains(&dot) {
    // fixed point format
    if dot <= 0 {
      buf[pos] = b'0';
      buf[pos + 1] = b'.';
      pos += 2;
      let pad = -dot as usize;
      buf[pos..pos + pad].fill(b'0');
      pos += pad;
      buf[pos..pos + declen].copy_from_slice(dec);
      trimzero(buf, pos + declen)
    } else if dot as usize == declen {
      // no dot
      buf[pos..pos + declen].copy_from_slice(dec);
      pos + declen
    } else if (dot as usize) < declen {
      // dot in the middle
      let dot = dot as usize;
      buf[pos..pos + dot].copy_from_slice(&dec[..dot]);
      buf[pos + dot] = b'.';
      buf[pos + dot + 1..pos + declen + 1].copy_from_slice(&dec[dot..]);
      trimzero(buf, pos + declen + 1)
    } else {
      // no dot, zero padding
      buf[pos..pos + declen].copy_from_slice(dec);
      buf[pos + declen..pos + dot as usize].fill(b'0');
      pos + dot as usize
    }
  } else {
    // scientific format
    buf[pos] = dec[0];
    buf[pos + 1] = b'.';
    buf[pos + 2..pos + declen + 1].copy_from_slice(&dec[1..]);

    let mut e = trimzero(buf, pos + declen + 1);
    if buf[e - 1] == b'.' {
      e -= 1;
    }

    e += printexp(&mut buf[e..], dot - 1);
    e
  }
}

/// 安全包装：Lua `tostring` 语义的浮点转字符串（Schubfach 精确输出）。
/// 供 ulua-rt 等 crate 复用，替代近似的手写 `format!`。
pub fn lua_number_to_string(n: f64) -> String {
  use crate::macros::luai_maxnum_2_str::LUAI_MAXNUM2STR;

  let mut buf = [0u8; LUAI_MAXNUM2STR as usize];
  let len = luai_num2str_buf(&mut buf, n);
  let bytes: Vec<u8> = buf[..len].to_vec();
  // 数字输出恒为 ASCII，UTF-8 校验必然通过。
  String::from_utf8(bytes).expect("luai_num2str 恒产出 ASCII 数字文本")
}

// r7-tprod2 尾矿台账（本文件票面 1 枚：让 1）——:128 `String::from_utf8(bytes)`
// 消费 Vec 不重配，全程恒 1 次堆配（owned String 下限）+ ≤LUAI_MAXNUM2STR 字节
// ASCII 校验扫描；免扫描需 from_utf8_unchecked 即在 safe fn 新增 unsafe，违本票
// 「ptr→ref 仅既有 unsafe 块内」纪律，让。
