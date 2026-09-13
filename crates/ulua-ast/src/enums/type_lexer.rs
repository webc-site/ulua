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
  pub const SEMICOLON: Type = Type(b';' as i32);
  pub const LESS: Type = Type(b'<' as i32);
  pub const GREATER: Type = Type(b'>' as i32);
  pub const PIPE: Type = Type(b'|' as i32);
  pub const QUESTION: Type = Type(b'?' as i32);
  pub const AMPERSAND: Type = Type(b'&' as i32);
  pub const CHAR_OPEN: Type = Type(b'(' as i32);
  pub const CHAR_PIPE: Type = Type(b'|' as i32);
  pub const CHAR_AMPERSAND: Type = Type(b'&' as i32);
  pub const CHAR_COMMA: Type = Type(b',' as i32);
  pub const COLON: Type = Type(b':' as i32);
  pub const COMMA: Type = Type(b',' as i32);
  pub const OPERATOR: Type = Type(b'=' as i32);
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
  pub const RESERVED_END_TOKEN: Type = Type(312);
}
