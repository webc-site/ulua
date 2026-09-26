//! `Lexeme::Type` (`Ast/include/Luau/Lexer.h`).
//!
//! Faithful port. In Luau the token type is a plain integer: values `1..255`
//! are literal character codes (so `'+'`, `'-'`, `'<'` are valid token types),
//! and the named multi-character tokens begin at `Char_END = 256`. A fieldless
//! Rust enum cannot represent the single-character values, so `Type` is a
//! newtype over `i32` (the C++ enum's underlying type) with associated consts.
//! `Type::EQUAL`-style paths still resolve, a single-char token is `Type(c)`,
//! and the derived `Ord` matches the C++ `<`/`>=` range checks against
//! `Char_END` / `Reserved_BEGIN`.

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Type(pub i32);

impl Type {
  pub const EOF: Type = Type(0);

  // 1..255 means actual character values
  // （以下具名常量均为单字符 ASCII 码；多字符 token 从 CHAR_END=256 起。
  // 注意 `EQUAL_SIGN`(b'='，赋值) 与 `EQUAL`(257，比较 '==') 是不同 token。）
  pub const SEMICOLON: Type = Type(b';' as i32);
  pub const LESS: Type = Type(b'<' as i32);
  pub const GREATER: Type = Type(b'>' as i32);
  pub const PIPE: Type = Type(b'|' as i32);
  pub const QUESTION: Type = Type(b'?' as i32);
  pub const AMPERSAND: Type = Type(b'&' as i32);
  pub const COLON: Type = Type(b':' as i32);
  pub const COMMA: Type = Type(b',' as i32);
  /// 单字符 `'='`（cpp 直接以 `'='` 比较，如 `matchRecoveryStopOnToken['=']`）：
  /// 通用「赋值」运算符 token（generic 默认值 `<> = T`、local 初始化 `= expr`）。
  pub const EQUAL_SIGN: Type = Type(b'=' as i32);
  pub const PLUS: Type = Type(b'+' as i32);
  pub const MINUS: Type = Type(b'-' as i32);
  pub const STAR: Type = Type(b'*' as i32);
  pub const SLASH: Type = Type(b'/' as i32);
  pub const PERCENT: Type = Type(b'%' as i32);
  pub const CARET: Type = Type(b'^' as i32);
  /// 单字符 `'!'`（cpp 直接以 `'!'` 比较，如 `~=`/`!=` 混淆检测的首字符判定）。
  pub const BANG: Type = Type(b'!' as i32);
  pub const HASH: Type = Type(b'#' as i32);
  pub const DOT: Type = Type(b'.' as i32);
  pub const LBRACKET: Type = Type(b'[' as i32);
  pub const RBRACKET: Type = Type(b']' as i32);
  pub const LBRACE: Type = Type(b'{' as i32);
  pub const RBRACE: Type = Type(b'}' as i32);
  pub const LPAREN: Type = Type(b'(' as i32);
  pub const RPAREN: Type = Type(b')' as i32);
  pub const CHAR_END: Type = Type(256);

  pub const EQUAL: Type = Type(257);
  pub const LESS_EQUAL: Type = Type(258);
  pub const GREATER_EQUAL: Type = Type(259);
  pub const NOT_EQUAL: Type = Type(260);
  pub const DOT2: Type = Type(261);
  pub const DOT3: Type = Type(262);
  pub const SKINNY_ARROW: Type = Type(263);
  pub const DOUBLE_COLON: Type = Type(264);
  pub const FLOOR_DIV: Type = Type(265);

  pub const INTERP_STRING_BEGIN: Type = Type(266);
  pub const INTERP_STRING_MID: Type = Type(267);
  pub const INTERP_STRING_END: Type = Type(268);
  // An interpolated string with no expressions (like `x`)
  pub const INTERP_STRING_SIMPLE: Type = Type(269);

  pub const ADD_ASSIGN: Type = Type(270);
  pub const SUB_ASSIGN: Type = Type(271);
  pub const MUL_ASSIGN: Type = Type(272);
  pub const DIV_ASSIGN: Type = Type(273);
  pub const FLOOR_DIV_ASSIGN: Type = Type(274);
  pub const MOD_ASSIGN: Type = Type(275);
  pub const POW_ASSIGN: Type = Type(276);
  pub const CONCAT_ASSIGN: Type = Type(277);

  pub const RAW_STRING: Type = Type(278);
  pub const QUOTED_STRING: Type = Type(279);
  pub const NUMBER: Type = Type(280);
  pub const NAME: Type = Type(281);

  pub const COMMENT: Type = Type(282);
  pub const BLOCK_COMMENT: Type = Type(283);

  pub const ATTRIBUTE: Type = Type(284);
  pub const ATTRIBUTE_OPEN: Type = Type(285);

  pub const BROKEN_STRING: Type = Type(286);
  pub const BROKEN_COMMENT: Type = Type(287);
  pub const BROKEN_UNICODE: Type = Type(288);
  pub const BROKEN_INTERP_DOUBLE_BRACE: Type = Type(289);
  pub const ERROR: Type = Type(290);

  pub const RESERVED_BEGIN: Type = Type(291);
  pub const RESERVED_AND: Type = Type::RESERVED_BEGIN; // = 291
  pub const RESERVED_BREAK: Type = Type(292);
  pub const RESERVED_DO: Type = Type(293);
  pub const RESERVED_ELSE: Type = Type(294);
  pub const RESERVED_ELSEIF: Type = Type(295);
  /// 关键字 `end`（cpp 驼峰 `Lexeme::Type::ReservedEnd`），是保留字区间内的普通
  /// 成员；勿与区间末尾哨兵 [`Type::RESERVED_END_TOKEN`] 混淆。
  pub const RESERVED_END: Type = Type(296);
  pub const RESERVED_FALSE: Type = Type(297);
  pub const RESERVED_FOR: Type = Type(298);
  pub const RESERVED_FUNCTION: Type = Type(299);
  pub const RESERVED_IF: Type = Type(300);
  pub const RESERVED_IN: Type = Type(301);
  pub const RESERVED_LOCAL: Type = Type(302);
  pub const RESERVED_NIL: Type = Type(303);
  pub const RESERVED_NOT: Type = Type(304);
  pub const RESERVED_OR: Type = Type(305);
  pub const RESERVED_REPEAT: Type = Type(306);
  pub const RESERVED_RETURN: Type = Type(307);
  pub const RESERVED_THEN: Type = Type(308);
  pub const RESERVED_TRUE: Type = Type(309);
  pub const RESERVED_UNTIL: Type = Type(310);
  pub const RESERVED_WHILE: Type = Type(311);
  /// 保留字区间的**末尾哨兵**（cpp `Lexeme::Type::Reserved_END`，全大写带下划线，
  /// 枚举最后一个值之后一位）：`[RESERVED_BEGIN, RESERVED_END_TOKEN)` 半开区间界定
  /// 全部关键字，也是 `matchRecoveryStopOnToken` 表的长度基准；它本身不是任何 token。
  /// 勿与关键字 `end`（[`Type::RESERVED_END`]）混淆。
  pub const RESERVED_END_TOKEN: Type = Type(312);

  /// 携带 `data`/`length` 载荷（源缓冲字节区间）的词素类型集合。
  /// `Lexeme::with_data` 的构造契约与 `Lexeme::get_length` 的读取前置共用这一
  /// 判定：此前两处各写一份 10 元 `||` 链，增删载荷类型时极易单边漂移。
  const DATA_PAYLOAD: [i32; 10] = [
    Self::RAW_STRING.0,
    Self::QUOTED_STRING.0,
    Self::INTERP_STRING_BEGIN.0,
    Self::INTERP_STRING_MID.0,
    Self::INTERP_STRING_END.0,
    Self::INTERP_STRING_SIMPLE.0,
    Self::BROKEN_INTERP_DOUBLE_BRACE.0,
    Self::NUMBER.0,
    Self::COMMENT.0,
    Self::BLOCK_COMMENT.0,
  ];

  /// 本词素是否携带 `data`/`length` 载荷（`LexemeData::data` 指针字段的有效性判据）。
  /// 表在编译期定形，`contains` 由编译器折成跳转表，与旧 `||` 链同形。
  #[inline]
  pub fn has_data_payload(self) -> bool {
    Self::DATA_PAYLOAD.contains(&self.0)
  }
}
