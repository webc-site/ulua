use crate::enums::type_lexer::Type;

/// 类型联合跟随集（'|' '?' '&'）：具名 const 模式 matches!，替代码值散比
#[inline]
pub(crate) fn is_type_follow(c: Type) -> bool {
  matches!(c, Type::PIPE | Type::QUESTION | Type::AMPERSAND)
}
