use ulua_common::functions::is_c_space::is_c_space;

/// 类 id：`to_ascii_lowercase(cl)` 落在类字母集中的序号（0..=10）。
/// 任何非类字节编码为 [`NON_CLASS`]（调用方退化为 `cl == c` 相等比较）。
const NON_CLASS: u8 = 0x7f;

/// 类字母总数（a c d g l p s u w x z）。
const CLASS_COUNT: usize = 11;

/// `cl` 字节 → 类编码：低 7 位 = [`class_id`]（[`NON_CLASS`] = 非类），bit7 =
/// 取反位（原 `cl` 为大写类字母时谓词取反，对应 cpp `if (isupper(cl)) res = !res`）。
/// 两张表均在编译期由与旧分支实现逐谓词相同的 [`class_pred`] 构造，运行期零初始化。
static CLASS_TABLE: [u8; 256] = build_class_table();

/// 类 id × 256 字符的谓词位图：`CLASS_MATCH[id * 256 + c]` 即原 `match lower`
/// 十一个类分支（含已废弃 'z'）对字符 `c` 的判定结果（0/1）。
static CLASS_MATCH: [u8; CLASS_COUNT * 256] = build_class_match();

const fn class_id(lower: u8) -> u8 {
  match lower {
    b'a' => 0,
    b'c' => 1,
    b'd' => 2,
    b'g' => 3,
    b'l' => 4,
    b'p' => 5,
    b's' => 6,
    b'u' => 7,
    b'w' => 8,
    b'x' => 9,
    b'z' => 10, // deprecated option
    _ => NON_CLASS,
  }
}

/// 与旧 `match lower` 十一个分支逐谓词相同（`is_ascii_*` 与 [`is_c_space`] 均为
/// const 可算），供编译期填表；`id` 恒由 [`class_id`] 产出，越界 arm 不可达。
const fn class_pred(id: usize, c: u8) -> bool {
  match id {
    0 => c.is_ascii_alphabetic(),
    1 => c.is_ascii_control(),
    2 => c.is_ascii_digit(),
    3 => c.is_ascii_graphic(),
    4 => c.is_ascii_lowercase(),
    5 => c.is_ascii_punctuation(),
    6 => is_c_space(c),
    7 => c.is_ascii_uppercase(),
    8 => c.is_ascii_alphanumeric(),
    9 => c.is_ascii_hexdigit(),
    10 => c == 0,
    _ => false,
  }
}

const fn build_class_table() -> [u8; 256] {
  let mut table = [0u8; 256];
  let mut cl = 0usize;
  while cl < 256 {
    let b = cl as u8;
    let id = class_id(b.to_ascii_lowercase());
    table[cl] = if id == NON_CLASS {
      NON_CLASS
    } else if b.is_ascii_lowercase() {
      id
    } else {
      id | 0x80
    };
    cl += 1;
  }
  table
}

const fn build_class_match() -> [u8; CLASS_COUNT * 256] {
  let mut table = [0u8; CLASS_COUNT * 256];
  let mut id = 0usize;
  while id < CLASS_COUNT {
    let mut c = 0usize;
    while c < 256 {
      table[id * 256 + c] = class_pred(id, c as u8) as u8;
      c += 1;
    }
    id += 1;
  }
  table
}

/// cpp `lstrlib.cpp match_class`：单字符类判定（`%a`/`%d`/`%s`/`%u`/`%w`/…
/// 含大写取反与非类字节相等回退）。查表形态：两次对齐读 + 异或，取代旧实现的
/// `to_ascii_lowercase` + 十一臂 match + `is_ascii_*` 多重范围比较（热点采样中
/// 该函数以真实调用边界占 patterns 7% 自时）。
///
/// 语义逐位保持：`c`/`cl` 均按调用契约截断到低 8 位查表（与旧 `(c as u8)`/
/// `(cl as u8)` 截断一致，见 `tests::class_table_matches_naive_over_full_domain`
/// 的全值域逐点对账）；非类字节返回 `cl == c`；取反位只对类字节生效。
#[inline]
pub(crate) fn match_class(c: i32, cl: i32) -> i32 {
  let entry = CLASS_TABLE[cl as u8 as usize];
  let id = (entry & 0x7f) as usize;
  if id == NON_CLASS as usize {
    return if cl == c { 1 } else { 0 };
  }
  // 调用契约：c 恒为 u8 加宽值（src_byte/pat_byte 读取点），`(c as u8)` 截断
  // 与旧实现逐位一致
  let matched = CLASS_MATCH[id * 256 + (c as u8) as usize] != 0;
  let inverted = (entry & 0x80) != 0;
  if matched != inverted { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
  use ulua_common::functions::is_c_space::is_c_space;

  /// 旧分支实现（重构前的逐谓词版本），作全值域对账的参照真值。
  fn match_class_naive(c: i32, cl: i32) -> i32 {
    let lower = (cl as u8).to_ascii_lowercase();
    let res = match lower {
      b'a' => (c as u8).is_ascii_alphabetic(),
      b'c' => (c as u8).is_ascii_control(),
      b'd' => (c as u8).is_ascii_digit(),
      b'g' => (c as u8).is_ascii_graphic(),
      b'l' => (c as u8).is_ascii_lowercase(),
      b'p' => (c as u8).is_ascii_punctuation(),
      b's' => is_c_space(c as u8),
      b'u' => (c as u8).is_ascii_uppercase(),
      b'w' => (c as u8).is_ascii_alphanumeric(),
      b'x' => (c as u8).is_ascii_hexdigit(),
      b'z' => c == 0,
      _ => return if cl == c { 1 } else { 0 },
    };
    if (cl as u8).is_ascii_lowercase() {
      if res { 1 } else { 0 }
    } else if !res {
      1
    } else {
      0
    }
  }

  #[test]
  fn class_table_matches_naive_over_full_domain() {
    for cl in 0i32..256 {
      for c in 0i32..256 {
        assert_eq!(
          super::match_class(c, cl),
          match_class_naive(c, cl),
          "c={c} cl={cl}"
        );
      }
    }
  }
}
