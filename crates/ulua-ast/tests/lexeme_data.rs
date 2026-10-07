//! `LexemeData` 具名字段化（ast-lexeme 波：cpp 匿名 union → `data` 指针 +
//! `codepoint` 两字段）后「构造时非活跃字段恒 0/null」不变量的对齐测试。
//!
//! cpp 里读哪一臂由 `Lexeme::Type` 决定、另一臂是覆写前的残值；Rust 收口的
//! 三个构造点（[`Lexeme::new`] 取 [`LexemeData::EMPTY`]、[`Lexeme::with_data`]
//! 与 [`Lexeme::with_name`] 各写己臂）把「非活跃臂恒 0/null」钉成硬不变量。
//! 本文件逐构造点钉死该形状，防止后续重构在臂间引入残值漂移。

use ulua_ast::{
  enums::type_lexer::Type,
  records::{
    ast_name::AstName,
    lexeme::{Lexeme, LexemeData},
    location::Location,
  },
};

#[test]
fn empty_lexeme_payload_is_double_zero() {
  // 无负载构造（cpp 默认构造的 `nullptr` 臂）：指针与码点两字段全零。
  let lx = Lexeme::new(Location::default(), Type::EOF);
  assert!(
    lx.data.data.is_none(),
    "EMPTY 构造下 `data` 指针臂须为缺席（None）"
  );
  assert_eq!(lx.data.codepoint, 0, "EMPTY 构造下 `codepoint` 臂须为 0");
  // 常量构造点与逐字段零形像一致。
  assert!(LexemeData::EMPTY.data.is_none());
  assert_eq!(LexemeData::EMPTY.codepoint, 0);
}

#[test]
fn with_data_leaves_codepoint_arm_zero() {
  // 载荷构造只写 `data` 指针臂（透传切片地址），非活跃的 `codepoint` 臂恒 0。
  static PAYLOAD: &[u8] = b"0x1F";
  let lx = Lexeme::with_data(Location::default(), Type::NUMBER, PAYLOAD);
  assert_eq!(
    lx.data.as_ptr(),
    PAYLOAD.as_ptr(),
    "`data` 臂为透传的地址值"
  );
  assert_eq!(
    lx.data.codepoint, 0,
    "with_data 后 `codepoint` 非活跃臂须为 0"
  );
}

#[test]
fn with_name_leaves_codepoint_arm_zero() {
  // 名构造只写同一 `data` 指针臂（cpp `name` 臂合并于此），`codepoint` 臂恒 0。
  static NAME: &[u8] = b"foo\0";
  let name = AstName::from_raw_parts(NAME.as_ptr().cast(), 3);
  let lx = Lexeme::with_name(Location::default(), Type::NAME, name);
  assert_eq!(lx.data.as_ptr(), name.as_ptr(), "`data` 臂即 AstName 指针");
  assert_eq!(
    lx.data.codepoint, 0,
    "with_name 后 `codepoint` 非活跃臂须为 0"
  );
}

#[test]
fn broken_unicode_keeps_pointer_arm_null() {
  // BROKEN_UNICODE 的唯一码点写入形制（`read_utf_8_error` 末尾：EMPTY 基座上只写
  // `codepoint` 字段）；同函数前两枚早退（坏首字节、缺延续字节）以 codepoint=0
  // 原样构造返回，属同型构造但非码点写入位。本测试钉写入侧：码点写入不得波及指针臂。
  let mut lx = Lexeme::new(Location::default(), Type::BROKEN_UNICODE);
  assert!(lx.data.data.is_none());
  assert_eq!(lx.data.codepoint, 0);
  lx.data.codepoint = 0x2022;
  assert!(
    lx.data.data.is_none(),
    "写码点后 `data` 指针臂须仍为 null（臂间零耦合）"
  );
}
