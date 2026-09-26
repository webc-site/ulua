use core::{f64::consts::PI, slice::from_raw_parts};

use ulua_ast::records::ast_name_table::AstNameTable;
use ulua_common::{
  enums::luau_builtin_function::{LuauBuiltinFunction, *},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  functions::{
    c_const::{cbool, cnum, cstring_slice, cstring_str, cvar, cvector},
    constant_type_name::constant_type_name,
  },
  records::constant::Constant,
};

// C++ BuiltinFolding.cpp:17 `const double kRadDeg = kPi / 180.0;` (~0.0174533)。
// 旧值取其倒数（180/pi），导致 math.deg/math.rad 的折叠互换了。
const K_RAD_DEG: f64 = PI / 180.0;
const K_STRING_CHAR_FOLD_LIMIT: usize = 128;

/// `2^512`（指数位 512+1023=1535）；编译期常量，分段缩放用
const K_TWO_POW_512: f64 = f64::from_bits(0x5FF0_0000_0000_0000);
/// `2^-512`（指数位 -512+1023=511）
const K_TWO_POW_NEG_512: f64 = f64::from_bits(0x1FF0_0000_0000_0000);

/// f64 IEEE754 指数偏置（ldexp 尾段直接拼指数位用）
const K_F64_EXP_BIAS: i32 = 1023;
/// f64 尾数位宽（拼指数位时的左移量）
const K_F64_MANTISSA_BITS: u32 = 52;
/// 经有符号 64 位整数中转，匹配运行时行为并优雅截断负整数
#[inline]
pub fn bit32(v: f64) -> u32 {
  v as i64 as u32
}

/// bit32 域宽（32 位无符号）
const K_BIT32_WIDTH: i32 = 32;
/// bit32 旋转量的 5 位取模掩码（`n & 31` ≡ `n % 32`，与 C++ `rotate*(n & 31)` 一致）
const K_BIT32_ROT_MASK: i32 = K_BIT32_WIDTH - 1;
/// bit32.extract/replace 宽度掩码基：`!(0xFFFFFFFE << (w-1))` 生成低 `w` 位全 1 掩码
const K_ALL_BITS_EXCEPT_LOW: u32 = 0xffff_fffe;

/// bit32.extract/replace 共用的低位掩码：`w >= 1` 时取低 `w` 位全 1。
/// 调用方已守 `w > 0 && f + w <= K_BIT32_WIDTH`，`w - 1` 不下溢、移位不越界。
#[inline]
fn low_mask(w: i32) -> u32 {
  !(K_ALL_BITS_EXCEPT_LOW << (w - 1) as u32)
}

/// bit32 多元位运算（band/bor/bxor/btest）归约共享体：任一参数非 Number 即返回
/// `None`（落 cvar 不折叠），与 C++ 逐臂 guard 一致。
fn fold_bitwise(args: &[Constant], combine: impl Fn(u32, u32) -> u32) -> Option<u32> {
  let mut r = match args.first() {
    Some(Constant::Number(v)) => bit32(*v),
    _ => return None,
  };
  for arg in &args[1..] {
    let Constant::Number(v) = arg else {
      return None;
    };
    r = combine(r, bit32(*v));
  }
  Some(r)
}

/// math.max/min 归约共享体：`better(a, r)` 对应 C++ `(a > r) ? a : r` /
/// `(a < r) ? a : r`——NaN 下比较恒 false、保留 r，语义逐位一致。
/// 任一参数非 Number 返回 `None`（落 cvar）。
fn fold_extremum(args: &[Constant], better: impl Fn(f64, f64) -> bool) -> Option<f64> {
  let mut r = match args.first() {
    Some(Constant::Number(v)) => *v,
    _ => return None,
  };
  for arg in &args[1..] {
    let Constant::Number(a) = arg else {
      return None;
    };
    if better(*a, r) {
      r = *a;
    }
  }
  Some(r)
}

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
  y * f64::from_bits(((e + K_F64_EXP_BIAS) as u64) << K_F64_MANTISSA_BITS)
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

/// 至少 `n` 个数值实参、且可选的第 `n+1` 个（bit32.extract/replace 的宽度参数）
/// 若存在也是 Number：对应 C++ `count >= n && args[0..n].type == Number
/// && (count == n || args[n].type == Number)` 判定链。
#[inline]
fn all_num_head(args: &[Constant], n: usize) -> bool {
  args.len() >= n
    && args
      .iter()
      .take(n + 1)
      .all(|a| matches!(a, Constant::Number(_)))
}

/// 一元数学内建 → 折叠函数指针表（编译期定表）：命中即由下方共享臂统一执行
/// `all_num(args, 1)` 守卫 + `cnum(f(num(&args[0])))`，消除逐臂重复样板。
/// deg/rad 的分段缩放与 sign 的三分支均以非捕获闭包表示（可退化 fn 指针，零开销）。
fn math_unary(bf: LuauBuiltinFunction) -> Option<fn(f64) -> f64> {
  Some(match bf {
    LBF_MATH_ABS => f64::abs,
    LBF_MATH_ACOS => f64::acos,
    LBF_MATH_ASIN => f64::asin,
    LBF_MATH_ATAN => f64::atan,
    LBF_MATH_CEIL => f64::ceil,
    LBF_MATH_COS => f64::cos,
    LBF_MATH_COSH => f64::cosh,
    LBF_MATH_DEG => |x| x / K_RAD_DEG,
    LBF_MATH_EXP => f64::exp,
    LBF_MATH_FLOOR => f64::floor,
    LBF_MATH_LOG10 => f64::log10,
    LBF_MATH_RAD => |x| x * K_RAD_DEG,
    LBF_MATH_ROUND => f64::round,
    LBF_MATH_SIGN => |x| {
      if x > 0.0 {
        1.0
      } else if x < 0.0 {
        -1.0
      } else {
        0.0
      }
    },
    LBF_MATH_SIN => f64::sin,
    LBF_MATH_SINH => f64::sinh,
    LBF_MATH_SQRT => f64::sqrt,
    LBF_MATH_TAN => f64::tan,
    LBF_MATH_TANH => f64::tanh,
    _ => return None,
  })
}

/// 一元谓词内建 → bool 折叠函数指针表，出参走 cbool（is_nan 等按 &self 取
/// 方法，须以闭包适配成 `fn(f64) -> bool`）。
fn math_unary_pred(bf: LuauBuiltinFunction) -> Option<fn(f64) -> bool> {
  Some(match bf {
    LBF_MATH_ISNAN => |x| x.is_nan(),
    LBF_MATH_ISINF => |x| x.is_infinite(),
    LBF_MATH_ISFINITE => |x| x.is_finite(),
    _ => return None,
  })
}

/// 二元数学/移位内建 → 折叠函数指针表（编译期定表）：命中即由共享臂统一执行
/// `all_num(args, 2)` 守卫 + `cnum(...)`，消除逐臂重复样板。移位族的域宽守卫
/// （`s ∉ [0,32)` 不折叠、落 cvar，负数 `as u32` 必为大值故单边比较即可）
/// 以返回 `None` 表示，与数学族的恒折叠统一在同一返回类型下。
fn math_binary(bf: LuauBuiltinFunction) -> Option<fn(f64, f64) -> Option<f64>> {
  Some(match bf {
    LBF_MATH_ATAN2 => |a, b| Some(a.atan2(b)),
    LBF_MATH_FMOD => |a, b| Some(a % b),
    LBF_MATH_LDEXP => |x, e| Some(ldexp(x, e as i32)),
    LBF_MATH_POW => |a, b| Some(a.powf(b)),
    LBF_BIT32_ARSHIFT => |v, s| {
      let (u, s) = (bit32(v), s as i32);
      ((s as u32) < K_BIT32_WIDTH as u32).then(|| (u as i32 >> s) as u32 as f64)
    },
    LBF_BIT32_LSHIFT => |v, s| {
      let (u, s) = (bit32(v), s as i32);
      ((s as u32) < K_BIT32_WIDTH as u32).then(|| (u << s) as f64)
    },
    LBF_BIT32_RSHIFT => |v, s| {
      let (u, s) = (bit32(v), s as i32);
      ((s as u32) < K_BIT32_WIDTH as u32).then(|| (u >> s) as f64)
    },
    _ => return None,
  })
}

/// C++ `foldBuiltin`：折叠内建函数调用。C++ 侧的 `(args 指针, count)` 对
/// 收敛为借用切片，越界/null 风险由类型系统消除（无参内建即空切片）。
pub(crate) fn fold_builtin(
  string_table: &mut AstNameTable,
  bfid: i32,
  args: &[Constant],
) -> Constant {
  let count = args.len();

  // bfid → 枚举：校验后再 match（消除 guard 链 `as i32`），未知 id 落 cvar()
  let Some(bf) = LuauBuiltinFunction::from_id(bfid) else {
    return cvar();
  };

  // 一元数学/谓词内建共享折叠臂：查表命中且 all_num(1) 守卫通过才折叠；
  // 守卫失败仍落到末尾 cvar()，与原逐臂 `if` 空过语义一致
  if let Some(f) = math_unary(bf)
    && all_num(args, 1)
  {
    return cnum(f(num(&args[0])));
  }
  if let Some(f) = math_unary_pred(bf)
    && all_num(args, 1)
  {
    return cbool(f(num(&args[0])));
  }
  // 二元数学/移位内建共享折叠臂：守卫失败（all_num 不满足或移位越界）落到
  // 末尾 cvar()，与原逐臂 `if` 空过语义一致
  if let Some(f) = math_binary(bf)
    && all_num(args, 2)
    && let Some(v) = f(num(&args[0]), num(&args[1]))
  {
    return cnum(v);
  }

  match bf {
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

    // max/min 只差比较方向：归约臂共用，`better` 对应 C++ `(a > r) ? a : r` /
    // `(a < r) ? a : r`（NaN 下比较恒 false、保留 r，语义逐位一致）。
    LBF_MATH_MAX | LBF_MATH_MIN => {
      let better: fn(f64, f64) -> bool = if bf == LBF_MATH_MAX {
        |a, r| a > r
      } else {
        |a, r| a < r
      };

      if let Some(r) = fold_extremum(args, better) {
        return cnum(r);
      }
    }

    // 多元位运算族（band/bor/bxor/btest）：归约算子查表选定，btest 只把出参
    // 从数值换成非零判定，其余形态一致。
    LBF_BIT32_BAND | LBF_BIT32_BOR | LBF_BIT32_BXOR | LBF_BIT32_BTEST => {
      let combine: fn(u32, u32) -> u32 = match bf {
        LBF_BIT32_BOR => |a, b| a | b,
        LBF_BIT32_BXOR => |a, b| a ^ b,
        _ => |a, b| a & b,
      };

      if let Some(r) = fold_bitwise(args, combine) {
        return if bf == LBF_BIT32_BTEST {
          cbool(r != 0)
        } else {
          cnum(r as f64)
        };
      }
    }

    LBF_BIT32_BNOT => {
      if all_num(args, 1) {
        return cnum((!bit32(num(&args[0]))) as f64);
      }
    }

    LBF_BIT32_EXTRACT => {
      if all_num_head(args, 2) {
        let u = bit32(num(&args[0]));
        let f = num(&args[1]) as i32;
        let w = if count == 2 { 1 } else { num(&args[2]) as i32 };
        if f >= 0 && w > 0 && f + w <= K_BIT32_WIDTH {
          let m = low_mask(w);
          return cnum(((u >> f) & m) as f64);
        }
      }
    }

    // 左右旋转只差旋转方向（`n & 31` 同余），共用一条臂。
    LBF_BIT32_LROTATE | LBF_BIT32_RROTATE if all_num(args, 2) => {
      let u = bit32(num(&args[0]));
      let s = (num(&args[1]) as i32) & K_BIT32_ROT_MASK;
      // rotate_left(n) ≡ (u << (n&31)) | (u >> ((32-n)&31))，含 n=0
      let rotated = if bf == LBF_BIT32_LROTATE {
        u.rotate_left(s as u32)
      } else {
        u.rotate_right(s as u32)
      };
      return cnum(rotated as f64);
    }

    LBF_BIT32_REPLACE => {
      if all_num_head(args, 3) {
        let n = bit32(num(&args[0]));
        let v = bit32(num(&args[1]));
        let f = num(&args[2]) as i32;
        let w = if count == 3 { 1 } else { num(&args[3]) as i32 };
        if f >= 0 && w > 0 && f + w <= K_BIT32_WIDTH {
          let m = low_mask(w);
          return cnum(((n & !(m << f)) | ((v & m) << f)) as f64);
        }
      }
    }

    // type/typeof 只差 vector 一臂语义（同一元数与未知量守卫）：共享骨架
    // `constant_type_name` 按 typeof 旗标分流，两级诊断触发次序与折叠结果逐位不变。
    LBF_TYPE | LBF_TYPEOF => {
      if count == 1 && !args[0].is_unknown() {
        return constant_type_name(&args[0], bf == LBF_TYPEOF);
      }
    }

    // `string.byte(s[, i])` 两形态只差索引来源：无索引形态即 i=1 恒成立，
    // 合并按 count==1 补 1，越界/空串落 cvar 的兜底与原两臂逐位一致。
    LBF_STRING_BYTE => {
      if (1..=2).contains(&count)
        && matches!(args[0], Constant::Str(_))
        && (count == 1 || matches!(args[1], Constant::Number(_)))
      {
        let i = if count == 1 { 1 } else { num(&args[1]) as i32 };
        if i > 0
          && let Some(&b) = args[0].get_string_bytes().get(i as usize - 1)
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
        let bytes = if name.is_null() {
          &[]
        } else {
          // Safety: 进入 else 分支已保证 `name.value` 非空。它是上一步 `get_or_add_slice(&buf[..count])`
          // 在 string_table 中 intern 得到的指针，指向一段至少 `count` 字节、比本次借用存活且地址稳定的存储；
          // 元素为 u8（对齐 1，恒对齐），`from_raw_parts(.., count)` 的范围完全落在该分配内，故切片合法。
          unsafe { from_raw_parts(name.value, count) }
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

    // vector.create(x,y[,z[,w]]) 2..=4 元三形态只差分量个数：实参逐个填入
    // 前 count 槽、缺省分量保持 0.0，任一实参非 Number 或元数越界不折叠，
    // 与原逐臂守卫逐位一致。
    LBF_VECTOR => {
      if (2..=4).contains(&count) && all_num(args, count) {
        let mut v = [0.0f64; 4];
        for (dst, src) in v.iter_mut().zip(args) {
          *dst = num(src);
        }
        return cvector(v[0], v[1], v[2], v[3]);
      }
    }

    // 守卫形式（同原 ISFINITE 臂）：all_num 失败落 `_` 臂 → 末尾 cvar()，语义不变
    LBF_MATH_LERP if all_num(args, 3) => {
      let (a, b, t) = (num(&args[0]), num(&args[1]), num(&args[2]));
      let v = if t == 1.0 { b } else { a + (b - a) * t };
      return cnum(v);
    }

    _ => {}
  }

  cvar()
}
