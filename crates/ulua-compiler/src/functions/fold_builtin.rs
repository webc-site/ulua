use core::{f64::consts::PI, slice::from_raw_parts};

use ulua_ast::records::ast_name_table::AstNameTable;
use ulua_common::{
  enums::luau_builtin_function::{LuauBuiltinFunction, *},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  functions::{
    bit_32::bit32,
    cbool::cbool,
    cnum::cnum,
    cstring_builtin_folding::{cstring_slice, cstring_str},
    ctype::ctype,
    ctypeof::ctypeof,
    cvar::cvar,
    cvector::cvector,
  },
  records::constant::Constant,
};

// C++ BuiltinFolding.cpp:17 `const double kRadDeg = kPi / 180.0;` (~0.0174533).
// The previous value was the inverse (180/pi), which swapped math.deg/math.rad folding.
const K_RAD_DEG: f64 = PI / 180.0;
const K_STRING_CHAR_FOLD_LIMIT: usize = 128;

/// `2^512`（指数位 512+1023=1535）；编译期常量，分段缩放用
const K_TWO_POW_512: f64 = f64::from_bits(0x5FF0_0000_0000_0000);
/// `2^-512`（指数位 -512+1023=511）
const K_TWO_POW_NEG_512: f64 = f64::from_bits(0x1FF0_0000_0000_0000);

/// C++ `ldexp(x, int(e))`（BuiltinFolding.cpp:217）：`x * 2^e`。
/// 按 ±512 分段缩放，保证中间值与 `2^e` 本身都不溢出/下溢；
/// 直接 `x * 2.0.powi(e)` 会在 e≥1024 / e≤-1024 时把 `2^e` 先算成 inf/0，
/// 与 C 库在可表示结果上逐位不一致（如 ldexp(1, -1074) 应得 4.9e-324）。
fn ldexp(x: f64, e: i32) -> f64 {
  if x == 0.0 || !x.is_finite() {
    return x;
  }
  const CHUNK: i32 = 512;
  let (mut y, mut e) = (x, e);
  while e > CHUNK {
    y *= K_TWO_POW_512;
    e -= CHUNK;
    if !y.is_finite() {
      return y; // 中途溢出即最终溢出，符号不变
    }
  }
  while e < -CHUNK {
    y *= K_TWO_POW_NEG_512;
    e += CHUNK;
    if y == 0.0 {
      return y; // 中途下溢即最终下溢
    }
  }
  y * f64::from_bits(((e + 1023) as u64) << 52)
}

/// 读取数值常量：调用方（下述 match 各臂）均已按 C++ 前置条件校验 `Number`。
#[inline]
fn num(c: &Constant) -> f64 {
  match c {
    Constant::Number(v) => *v,
    _ => {
      LUAU_ASSERT!(false);
      0.0
    }
  }
}

/// 恰好 `n` 个参数且全部为 Number（对应 C++ `count == n && args[i].type == Number` 链）。
#[inline]
fn all_num(args: &[Constant], n: usize) -> bool {
  args.len() == n && args.iter().all(|a| matches!(a, Constant::Number(_)))
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn fold_builtin(
  string_table: &mut AstNameTable,
  bfid: i32,
  args: *const Constant,
  count: usize,
) -> Constant {
  // `slice::from_raw_parts(null, 0)` 是 UB；无参内建会传 null `args` 且 count 为 0，
  // 此时直接给空切片。
  let args = if count == 0 {
    &[][..]
  } else {
    unsafe { from_raw_parts(args, count) }
  };

  // bfid → 枚举：校验后再 match（消除 guard 链 `as i32`），未知 id 落 cvar()
  let Some(bf) = LuauBuiltinFunction::from_id(bfid) else {
    return cvar();
  };

  match bf {
    LBF_MATH_ABS => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).abs());
      }
    }

    LBF_MATH_ACOS => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).acos());
      }
    }

    LBF_MATH_ASIN => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).asin());
      }
    }

    LBF_MATH_ATAN2 => {
      if all_num(args, 2) {
        return cnum(num(&args[0]).atan2(num(&args[1])));
      }
    }

    LBF_MATH_ATAN => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).atan());
      }
    }

    LBF_MATH_CEIL => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).ceil());
      }
    }

    LBF_MATH_COSH => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).cosh());
      }
    }

    LBF_MATH_COS => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).cos());
      }
    }

    LBF_MATH_DEG => {
      if all_num(args, 1) {
        return cnum(num(&args[0]) / K_RAD_DEG);
      }
    }

    LBF_MATH_EXP => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).exp());
      }
    }

    LBF_MATH_FLOOR => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).floor());
      }
    }

    LBF_MATH_FMOD => {
      if all_num(args, 2) {
        return cnum(num(&args[0]) % num(&args[1]));
      }
    }

    LBF_MATH_LDEXP => {
      if all_num(args, 2) {
        return cnum(ldexp(num(&args[0]), num(&args[1]) as i32));
      }
    }

    LBF_MATH_LOG10 => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).log10());
      }
    }

    LBF_MATH_LOG => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).ln());
      } else if all_num(args, 2) {
        let base = num(&args[1]);
        if base == 2.0 {
          return cnum(num(&args[0]).log2());
        } else if base == 10.0 {
          return cnum(num(&args[0]).log10());
        } else {
          return cnum(num(&args[0]).ln() / base.ln());
        }
      }
    }

    LBF_MATH_MAX => {
      if count >= 1 && matches!(args[0], Constant::Number(_)) {
        // 折叠手写 max：NaN 语义须与 C++ `(a > r) ? a : r` 一致
        let mut r = num(&args[0]);
        for arg in &args[1..count] {
          if !matches!(arg, Constant::Number(_)) {
            return cvar();
          }
          let a = num(arg);
          r = if a > r { a } else { r };
        }
        return cnum(r);
      }
    }

    LBF_MATH_MIN => {
      if count >= 1 && matches!(args[0], Constant::Number(_)) {
        // 折叠手写 min：NaN 语义须与 C++ `(a < r) ? a : r` 一致
        let mut r = num(&args[0]);
        for arg in &args[1..count] {
          if !matches!(arg, Constant::Number(_)) {
            return cvar();
          }
          let a = num(arg);
          r = if a < r { a } else { r };
        }
        return cnum(r);
      }
    }

    LBF_MATH_POW => {
      if all_num(args, 2) {
        return cnum(num(&args[0]).powf(num(&args[1])));
      }
    }

    LBF_MATH_RAD => {
      if all_num(args, 1) {
        return cnum(num(&args[0]) * K_RAD_DEG);
      }
    }

    LBF_MATH_SINH => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).sinh());
      }
    }

    LBF_MATH_SIN => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).sin());
      }
    }

    LBF_MATH_SQRT => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).sqrt());
      }
    }

    LBF_MATH_TANH => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).tanh());
      }
    }

    LBF_MATH_TAN => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).tan());
      }
    }

    LBF_BIT32_ARSHIFT => {
      if all_num(args, 2) {
        let u = bit32(num(&args[0]));
        let s = num(&args[1]) as i32;
        if (s as u32) < 32 {
          return cnum((u as i32 >> s) as u32 as f64);
        }
      }
    }

    LBF_BIT32_BAND => {
      if count >= 1 && matches!(args[0], Constant::Number(_)) {
        let mut r = bit32(num(&args[0]));
        for arg in &args[1..count] {
          if !matches!(arg, Constant::Number(_)) {
            return cvar();
          }
          r &= bit32(num(arg));
        }
        return cnum(r as f64);
      }
    }

    LBF_BIT32_BNOT => {
      if all_num(args, 1) {
        return cnum((!bit32(num(&args[0]))) as f64);
      }
    }

    LBF_BIT32_BOR => {
      if count >= 1 && matches!(args[0], Constant::Number(_)) {
        let mut r = bit32(num(&args[0]));
        for arg in &args[1..count] {
          if !matches!(arg, Constant::Number(_)) {
            return cvar();
          }
          r |= bit32(num(arg));
        }
        return cnum(r as f64);
      }
    }

    LBF_BIT32_BXOR => {
      if count >= 1 && matches!(args[0], Constant::Number(_)) {
        let mut r = bit32(num(&args[0]));
        for arg in &args[1..count] {
          if !matches!(arg, Constant::Number(_)) {
            return cvar();
          }
          r ^= bit32(num(arg));
        }
        return cnum(r as f64);
      }
    }

    LBF_BIT32_BTEST => {
      if count >= 1 && matches!(args[0], Constant::Number(_)) {
        let mut r = bit32(num(&args[0]));
        for arg in &args[1..count] {
          if !matches!(arg, Constant::Number(_)) {
            return cvar();
          }
          r &= bit32(num(arg));
        }
        return cbool(r != 0);
      }
    }

    LBF_BIT32_EXTRACT => {
      if count >= 2
        && matches!(args[0], Constant::Number(_))
        && matches!(args[1], Constant::Number(_))
        && (count == 2 || matches!(args[2], Constant::Number(_)))
      {
        let u = bit32(num(&args[0]));
        let f = num(&args[1]) as i32;
        let w = if count == 2 { 1 } else { num(&args[2]) as i32 };
        if f >= 0 && w > 0 && f + w <= 32 {
          let m = !(0xfffffffeu32 << (w - 1));
          return cnum(((u >> f) & m) as f64);
        }
      }
    }

    LBF_BIT32_LROTATE => {
      if all_num(args, 2) {
        let u = bit32(num(&args[0]));
        let s = num(&args[1]) as i32;
        // rotate_left(n) ≡ (u << (n&31)) | (u >> ((32-n)&31))，含 n=0
        return cnum(u.rotate_left((s & 31) as u32) as f64);
      }
    }

    LBF_BIT32_LSHIFT => {
      if all_num(args, 2) {
        let u = bit32(num(&args[0]));
        let s = num(&args[1]) as i32;
        if (s as u32) < 32 {
          return cnum((u << s) as f64);
        }
      }
    }

    LBF_BIT32_REPLACE => {
      if count >= 3
        && matches!(args[0], Constant::Number(_))
        && matches!(args[1], Constant::Number(_))
        && matches!(args[2], Constant::Number(_))
        && (count == 3 || matches!(args[3], Constant::Number(_)))
      {
        let n = bit32(num(&args[0]));
        let v = bit32(num(&args[1]));
        let f = num(&args[2]) as i32;
        let w = if count == 3 { 1 } else { num(&args[3]) as i32 };
        if f >= 0 && w > 0 && f + w <= 32 {
          let m = !(0xfffffffeu32 << (w - 1));
          return cnum(((n & !(m << f)) | ((v & m) << f)) as f64);
        }
      }
    }

    LBF_BIT32_RROTATE => {
      if all_num(args, 2) {
        let u = bit32(num(&args[0]));
        let s = num(&args[1]) as i32;
        // rotate_right(n) ≡ (u >> (n&31)) | (u << ((32-n)&31))，含 n=0
        return cnum(u.rotate_right((s & 31) as u32) as f64);
      }
    }

    LBF_BIT32_RSHIFT => {
      if all_num(args, 2) {
        let u = bit32(num(&args[0]));
        let s = num(&args[1]) as i32;
        if (s as u32) < 32 {
          return cnum((u >> s) as f64);
        }
      }
    }

    LBF_TYPE => {
      if count == 1 && !args[0].is_unknown() {
        return ctype(&args[0]);
      }
    }

    LBF_STRING_BYTE => {
      if count == 1 && matches!(args[0], Constant::Str(_)) {
        let s = args[0].get_string_bytes();
        if let Some(&b) = s.first() {
          return cnum(b as f64);
        }
      } else if count == 2
        && matches!(args[0], Constant::Str(_))
        && matches!(args[1], Constant::Number(_))
      {
        let i = num(&args[1]) as i32;
        let s = args[0].get_string_bytes();
        if i > 0
          && let Some(&b) = s.get(i as usize - 1)
        {
          return cnum(b as f64);
        }
      }
    }

    LBF_STRING_CHAR => {
      if count < K_STRING_CHAR_FOLD_LIMIT {
        let mut buf = [0u8; K_STRING_CHAR_FOLD_LIMIT];
        for (i, arg) in args.iter().enumerate() {
          if !matches!(arg, Constant::Number(_)) {
            return cvar();
          }
          let ch = num(arg) as i32;
          if (ch as u8 as i32) != ch {
            return cvar();
          }
          buf[i] = ch as u8;
        }
        if count == 0 {
          return cstring_str("");
        }
        // AstName 不携带长度（intern 表里的 C 串），且 char 参可为 0（内嵌 NUL），
        // 必须用 count 而非 strlen 取字节
        let name = string_table.get_or_add_slice(&buf[..count]);
        let bytes = if name.value.is_null() {
          &[]
        } else {
          unsafe { from_raw_parts(name.value as *const u8, count) }
        };
        return cstring_slice(bytes);
      }
    }

    LBF_STRING_LEN => {
      if count == 1
        && let Constant::Str(s) = &args[0]
      {
        return cnum(f64::from(s.len));
      }
    }

    LBF_TYPEOF => {
      if count == 1 && !args[0].is_unknown() {
        return ctypeof(&args[0]);
      }
    }

    LBF_STRING_SUB => {
      if count >= 2 && matches!(args[0], Constant::Str(_)) && matches!(args[1], Constant::Number(_))
      {
        if count >= 3 && !matches!(args[2], Constant::Number(_)) {
          return cvar();
        }
        let str_bytes = args[0].get_string_bytes();
        let len = str_bytes.len();
        let mut start = num(&args[1]) as i32;
        let mut end = if count >= 3 {
          num(&args[2]) as i32
        } else {
          len as i32
        };

        // 相对位置：负数从串尾回数
        if start < 0 {
          start += len as i32 + 1;
        }
        if end < 0 {
          end += len as i32 + 1;
        }
        if end < 1 {
          return cstring_str("");
        }
        // start 钳到串首，end 钳到串尾
        start = start.max(1);
        end = end.min(len as i32);

        if start <= end {
          let sub = &str_bytes[start as usize - 1..end as usize];
          let name = string_table.get_or_add_slice(sub);
          return cstring_slice(name.as_bytes());
        }
        return cstring_str("");
      }
    }

    LBF_MATH_CLAMP => {
      if all_num(args, 3) {
        let (min, max) = (num(&args[1]), num(&args[2]));
        if min <= max {
          let v = num(&args[0]).clamp(min, max);
          return cnum(v);
        }
      }
    }

    LBF_MATH_SIGN => {
      if all_num(args, 1) {
        let v = num(&args[0]);
        return cnum(if v > 0.0 {
          1.0
        } else if v < 0.0 {
          -1.0
        } else {
          0.0
        });
      }
    }

    LBF_MATH_ROUND => {
      if all_num(args, 1) {
        return cnum(num(&args[0]).round());
      }
    }

    LBF_VECTOR => {
      if count >= 2
        && matches!(args[0], Constant::Number(_))
        && matches!(args[1], Constant::Number(_))
      {
        if count == 2 {
          return cvector(num(&args[0]), num(&args[1]), 0.0, 0.0);
        } else if count == 3 && matches!(args[2], Constant::Number(_)) {
          return cvector(num(&args[0]), num(&args[1]), num(&args[2]), 0.0);
        } else if count == 4
          && matches!(args[2], Constant::Number(_))
          && matches!(args[3], Constant::Number(_))
        {
          return cvector(num(&args[0]), num(&args[1]), num(&args[2]), num(&args[3]));
        }
      }
    }

    LBF_MATH_LERP => {
      if all_num(args, 3) {
        let (a, b, t) = (num(&args[0]), num(&args[1]), num(&args[2]));
        let v = if t == 1.0 { b } else { a + (b - a) * t };
        return cnum(v);
      }
    }

    LBF_MATH_ISNAN => {
      if all_num(args, 1) {
        return cbool(num(&args[0]).is_nan());
      }
    }

    LBF_MATH_ISINF => {
      if all_num(args, 1) {
        return cbool(num(&args[0]).is_infinite());
      }
    }

    LBF_MATH_ISFINITE if all_num(args, 1) => {
      return cbool(num(&args[0]).is_finite());
    }

    _ => {}
  }

  cvar()
}
