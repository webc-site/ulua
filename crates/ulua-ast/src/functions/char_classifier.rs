//! 字节域字符类的单一实现（cpp 判定族的 Rust 形态）。
//!
//! oracle：`cpp/Ast/src/Lexer.cpp:269`（`isAlpha`）、`:275`（`isDigit`）、
//! `:286`（`isNewline`）、`cpp/Ast/include/Luau/Lexer.h:259`（`isSpace`），
//! 以及 `cpp/Ast/src/PrettyPrinter.cpp:32`（`isIdentifierChar` =
//! `isIdentifierStartChar || isDigit`，`'_'` 归入起始字符集）。
//!
//! 形态：字符类由 `const fn` 在编译期折叠成 256 项位图（`.rodata`，零运行期
//! 构建、零堆分配），谓词函数只剩「一次界内取表 + 一次位与」。表与 cpp 判定式
//! 的逐位等价由下方 `const _` 自检在编译期锁定，改表失配即编译失败（参照
//! `ulua-common/src/functions/get_op_length.rs` 的定表自检形态）。
//!
//! 值域：cpp 侧这些谓词的入参是 `char`（字节），本 crate 全部调用点传入的
//! `char` 均出自 `Lexer::peekch()`（`buffer[i] as char`，≤ 0xFF）或
//! `StringWriter::last_char`（同为字节提升），故 256 项表即完整定义域。
//! 域外（> 0xFF，即真正的多字节码点）不属于 cpp `char` 域，此处统一判否——
//! 这比旧实现的 `ch as u8` 截断更严（截断会把 U+2061 误判为字母），在本 crate
//! 的实际值域内两者观测一致。

/// 字母（cpp `isAlpha`：`unsigned((ch | ' ') - 'a') < 26`）。
const ALPHA: u8 = 1 << 0;
/// 十进制数字（cpp `isDigit`：`unsigned(ch - '0') < 10`）。
const DIGIT: u8 = 1 << 1;
/// 空白（cpp `isSpace`：`' ' | '\t' | '\r' | '\n' | '\v' | '\f'`）。
const SPACE: u8 = 1 << 2;
/// 换行（cpp `isNewline`：`ch == '\n'`）。
const NEWLINE: u8 = 1 << 3;
/// 标识符起始字符集：字母 + `'_'`（cpp 调用点写作 `isAlpha(ch) || ch == '_'`，
/// 如 `Lexer.cpp:1004/1020`）。
const IDENT_START: u8 = ALPHA | UNDERSCORE;
/// 标识符字符集：起始字符 + 数字（cpp `readName` 的续读条件
/// `isAlpha(ch) || isDigit(ch) || ch == '_'`，`Lexer.cpp:699/713`）。
const IDENT: u8 = ALPHA | DIGIT | UNDERSCORE;
/// `'_'` 单独占一位，供起始/续读条件复用（值即下标 0x5F）。
const UNDERSCORE: u8 = 1 << 4;

/// 编译期定表：`K_CHAR_CLASS[byte]` 即该字节的字符类位图。
const K_CHAR_CLASS: [u8; 256] = build_char_class();

const fn build_char_class() -> [u8; 256] {
  let mut table = [0u8; 256];
  let mut ch = 0u16;
  while ch < 256 {
    let byte = ch as u8;
    let mut bits = 0u8;

    // cpp isAlpha
    if ((byte | b' ').wrapping_sub(b'a')) < 26 {
      bits |= ALPHA;
    }
    // cpp isDigit
    if byte.wrapping_sub(b'0') < 10 {
      bits |= DIGIT;
    }
    // cpp isSpace
    if matches!(byte, b' ' | b'\t' | b'\r' | b'\n' | 0x0b | 0x0c) {
      bits |= SPACE;
    }
    if byte == b'\n' {
      bits |= NEWLINE;
    }
    if byte == b'_' {
      bits |= UNDERSCORE;
    }

    table[byte as usize] = bits;
    ch += 1;
  }
  table
}

#[inline]
const fn class(ch: char) -> u8 {
  let code = ch as u32;
  if code < 256 {
    K_CHAR_CLASS[code as usize]
  } else {
    0
  }
}

#[inline]
pub fn is_alpha(ch: char) -> bool {
  class(ch) & ALPHA != 0
}

#[inline]
pub fn is_digit(c: char) -> bool {
  class(c) & DIGIT != 0
}

#[inline]
pub fn is_newline(ch: char) -> bool {
  class(ch) & NEWLINE != 0
}

#[inline]
pub fn is_space(ch: char) -> bool {
  class(ch) & SPACE != 0
}

/// cpp `isAlpha(ch) || ch == '_'`（`Lexer.cpp:1004/1020`）的单源形态。
#[inline]
pub fn is_identifier_start_char(c: char) -> bool {
  class(c) & IDENT_START != 0
}

/// cpp `isAlpha(ch) || isDigit(ch) || ch == '_'`（`Lexer.cpp:699/713`）的单源形态。
#[inline]
pub fn is_identifier_char(c: char) -> bool {
  class(c) & IDENT != 0
}

// 定表自检：与 cpp 判定式逐字节等价，漂表即编译失败。
const _: () = {
  let mut ch = 0u16;
  while ch < 256 {
    let byte = ch as u8;
    let bits = K_CHAR_CLASS[ch as usize];

    assert!(
      (bits & ALPHA != 0) == ((byte | b' ').wrapping_sub(b'a') < 26),
      "isAlpha 表位与 cpp (Lexer.cpp:269) 失配"
    );
    assert!(
      (bits & DIGIT != 0) == (byte.wrapping_sub(b'0') < 10),
      "isDigit 表位与 cpp (Lexer.cpp:275) 失配"
    );
    assert!(
      (bits & SPACE != 0) == matches!(byte, b' ' | b'\t' | b'\r' | b'\n' | 0x0b | 0x0c),
      "isSpace 表位与 cpp (Lexer.h:259) 失配"
    );
    assert!(
      (bits & NEWLINE != 0) == (byte == b'\n'),
      "isNewline 表位与 cpp (Lexer.cpp:286) 失配"
    );
    assert!((bits & UNDERSCORE != 0) == (byte == b'_'), "'_' 表位漂移");

    ch += 1;
  }

  // 关键采样：数字段/字母段/`'@'`（属性名前缀，必须非字母）/域外折叠。
  assert!(K_CHAR_CLASS[b'7' as usize] & DIGIT != 0);
  assert!(K_CHAR_CLASS[b'A' as usize] & ALPHA != 0);
  assert!(K_CHAR_CLASS[b'z' as usize] & ALPHA != 0);
  assert!(K_CHAR_CLASS[b'@' as usize] == 0);
  assert!(class('\u{2061}') == 0);
  assert!(class('\u{c1}') & ALPHA == 0);
};
