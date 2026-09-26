//! 内嵌 BuiltinDefinitions.cpp 各 `R"<tag>(...)<tag>"` 节的字节区间提取。
//!
//! 原实现对每次调用做 `format!` 分配 + 全文件子串扫描；内嵌文件是编译期
//! 不变量，故扫描逻辑全部下沉为 const fn（朴素 DFA 逐字节匹配，无正则），
//! 18 个已知节名的区间在编译期算好，运行期仅剩一次 match 派发 + 切片。

/// 未找到哨兵
const K_NONE: usize = usize::MAX;

const EMBEDDED_BUILTINS_CPP: &str = include_str!("embedded_builtin_definitions.cpp");
const EMBEDDED_BYTES: &[u8] = EMBEDDED_BUILTINS_CPP.as_bytes();

/// 逐字节 DFA：`from` 起查找 `needle`，返回起始下标，未找到返回 K_NONE
const fn find_from(needle: &[u8], from: usize) -> usize {
  let hay = EMBEDDED_BYTES;
  let mut i = from;
  while i + needle.len() <= hay.len() {
    let mut j = 0;
    while j < needle.len() {
      if hay[i + j] != needle[j] {
        break;
      }
      j += 1;
    }
    if j == needle.len() {
      return i;
    }
    i += 1;
  }
  K_NONE
}

/// `hay[a..a+n] == hay[b..b+n]`
const fn span_eq(a: usize, b: usize, n: usize) -> bool {
  let mut j = 0;
  while j < n {
    if EMBEDDED_BYTES[a + j] != EMBEDDED_BYTES[b + j] {
      return false;
    }
    j += 1;
  }
  true
}

/// 定位 `static ... {name} = R"<tag>( body )<tag>"` 的 body 区间。
/// 返回 (body_start, body_end)；(K_NONE, _) 表示节名未找到；
/// (start, K_NONE) 表示找到起点但无终止/缺 `(`（损坏文件）。
const fn raw_range(name: &[u8]) -> (usize, usize) {
  let mut pos = find_from(name, 0);
  while pos != K_NONE {
    // 名字之后必须紧跟 ` = R"`（与原实现 needle `{name} = R"` 等价）
    let p = pos + name.len();
    if p + 5 <= EMBEDDED_BYTES.len()
      && EMBEDDED_BYTES[p] == b' '
      && EMBEDDED_BYTES[p + 1] == b'='
      && EMBEDDED_BYTES[p + 2] == b' '
      && EMBEDDED_BYTES[p + 3] == b'R'
      && EMBEDDED_BYTES[p + 4] == b'"'
    {
      // 读 tag：`R"` 之后到首个 `(`（tag 可为空）
      let tag_start = p + 5;
      let mut q = tag_start;
      while q < EMBEDDED_BYTES.len() && EMBEDDED_BYTES[q] != b'(' {
        q += 1;
      }
      if q == EMBEDDED_BYTES.len() {
        return (K_NONE, K_NONE);
      }
      let body_start = q + 1;
      let tag_len = q - tag_start;
      // 自 body_start 起找首个 `)<tag>"` 终止串（与旧实现的相对搜索一致）
      let mut i = body_start;
      while i < EMBEDDED_BYTES.len() {
        if EMBEDDED_BYTES[i] == b')'
          && i + 1 + tag_len < EMBEDDED_BYTES.len()
          && span_eq(i + 1, tag_start, tag_len)
          && EMBEDDED_BYTES[i + 1 + tag_len] == b'"'
        {
          return (body_start, i);
        }
        i += 1;
      }
      return (body_start, K_NONE);
    }
    pos = find_from(name, pos + 1);
  }
  (K_NONE, K_NONE)
}

// —— 编译期预计算的 18 个已知节区间 ——
const R_BASE: (usize, usize) = raw_range(b"kBuiltinDefinitionBaseSrc");
const R_BIT32: (usize, usize) = raw_range(b"kBuiltinDefinitionBit32Src");
const R_MATH: (usize, usize) = raw_range(b"kBuiltinDefinitionMathSrc");
const R_OS: (usize, usize) = raw_range(b"kBuiltinDefinitionOsSrc");
const R_COROUTINE: (usize, usize) = raw_range(b"kBuiltinDefinitionCoroutineSrc");
const R_TABLE: (usize, usize) = raw_range(b"kBuiltinDefinitionTableSrc");
const R_DEBUG: (usize, usize) = raw_range(b"kBuiltinDefinitionDebugSrc");
const R_UTF8: (usize, usize) = raw_range(b"kBuiltinDefinitionUtf8Src");
const R_BUFFER: (usize, usize) = raw_range(b"kBuiltinDefinitionBufferSrc");
const R_BUFFER_NOINT: (usize, usize) = raw_range(b"kBuiltinDefinitionBufferSrc_NOINTEGER");
const R_VECTOR: (usize, usize) = raw_range(b"kBuiltinDefinitionVectorSrc");
const R_INTEGER: (usize, usize) = raw_range(b"kBuiltinDefinitionIntegerSrc");
const R_CLASS: (usize, usize) = raw_range(b"kBuiltinDefinitionClassSrc");
const R_TYPEMETHOD: (usize, usize) = raw_range(b"kBuiltinDefinitionTypeMethodSrc");
const R_TYPEMETHOD_DEP: (usize, usize) = raw_range(b"kBuiltinDefinitionTypeMethodSrc_DEPRECATED");
const R_TYPEMETHOD_NOINT: (usize, usize) = raw_range(b"kBuiltinDefinitionTypeMethodSrc_NOINTEGER");
const R_TYPESLIB: (usize, usize) = raw_range(b"kBuiltinDefinitionTypesLibSrc");
const R_TYPESLIB_NOINT: (usize, usize) = raw_range(b"kBuiltinDefinitionTypesLibSrc_NOINTEGER");

const _: () = {
  // 已知节必须在编译期全部命中且带终止符，否则内嵌文件被改动即编译失败
  let all: [(usize, usize); 18] = [
    R_BASE,
    R_BIT32,
    R_MATH,
    R_OS,
    R_COROUTINE,
    R_TABLE,
    R_DEBUG,
    R_UTF8,
    R_BUFFER,
    R_BUFFER_NOINT,
    R_VECTOR,
    R_INTEGER,
    R_CLASS,
    R_TYPEMETHOD,
    R_TYPEMETHOD_DEP,
    R_TYPEMETHOD_NOINT,
    R_TYPESLIB,
    R_TYPESLIB_NOINT,
  ];
  let mut i = 0;
  while i < all.len() {
    assert!(all[i].0 != K_NONE && all[i].1 != K_NONE && all[i].0 <= all[i].1);
    i += 1;
  }
};

/// 取出内嵌文件中名为 `name` 的 `R"..."` 原始串体。
/// 已知节名走编译期预计算区间；未知名字退化为同一 const 扫描的运行时执行
/// （保持原「missing/unterminated」panic 语义，无额外分配）。
pub fn embedded_builtin_raw_const(name: &str) -> &'static str {
  let r = match name {
    "kBuiltinDefinitionBaseSrc" => R_BASE,
    "kBuiltinDefinitionBit32Src" => R_BIT32,
    "kBuiltinDefinitionMathSrc" => R_MATH,
    "kBuiltinDefinitionOsSrc" => R_OS,
    "kBuiltinDefinitionCoroutineSrc" => R_COROUTINE,
    "kBuiltinDefinitionTableSrc" => R_TABLE,
    "kBuiltinDefinitionDebugSrc" => R_DEBUG,
    "kBuiltinDefinitionUtf8Src" => R_UTF8,
    "kBuiltinDefinitionBufferSrc" => R_BUFFER,
    "kBuiltinDefinitionBufferSrc_NOINTEGER" => R_BUFFER_NOINT,
    "kBuiltinDefinitionVectorSrc" => R_VECTOR,
    "kBuiltinDefinitionIntegerSrc" => R_INTEGER,
    "kBuiltinDefinitionClassSrc" => R_CLASS,
    "kBuiltinDefinitionTypeMethodSrc" => R_TYPEMETHOD,
    "kBuiltinDefinitionTypeMethodSrc_DEPRECATED" => R_TYPEMETHOD_DEP,
    "kBuiltinDefinitionTypeMethodSrc_NOINTEGER" => R_TYPEMETHOD_NOINT,
    "kBuiltinDefinitionTypesLibSrc" => R_TYPESLIB,
    "kBuiltinDefinitionTypesLibSrc_NOINTEGER" => R_TYPESLIB_NOINT,
    _ => raw_range(name.as_bytes()),
  };
  if r.0 == K_NONE {
    panic!("missing embedded builtin definition {name}");
  }
  if r.1 == K_NONE {
    panic!("unterminated embedded builtin definition {name}");
  }
  // 区间边界均为 ASCII 标记（`R"<tag>(` 与 `)`）之后/之前，UTF-8 边界安全；
  // 已知 18 名已由上面的 const assert 钉死。
  &EMBEDDED_BUILTINS_CPP[r.0..r.1]
}
