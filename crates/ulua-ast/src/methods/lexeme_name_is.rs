use crate::records::lexeme::{Lexeme, Type};

/// 词法单元是否为 NAME 且名字等于 `rhs`，对应 C++ 反复出现的
/// `lexeme.type == Lexeme::Name && AstName(lexeme.data.name) == "x"`。
/// NAME 判型先行短路，`data.name` 联合体仅在 NAME 时读 active 臂。
/// 接受 `&Lexeme`：实时态传 `parser.lexer.current()`，快照态传局部拷贝。
pub fn lexeme_name_is(lexeme: &Lexeme, rhs: &str) -> bool {
  lexeme.r#type == Type::NAME && lexeme.name() == rhs
}
