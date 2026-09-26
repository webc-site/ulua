use core::ptr::NonNull;

use crate::records::{ast_type_or_pack::AstTypeOrPack, parser::Parser};

impl Parser {
  pub fn parse_simple_type_or_pack(&mut self) -> AstTypeOrPack {
    let old_recursion_count = self.recursion_counter;

    let begin = self.lexer.current().location;

    let result = self.parse_simple_type(true, false);

    // cpp 在此处 `LUAU_ASSERT(!type)`：enum 的互斥形态已在类型层排除「两侧同时有值」，
    // 断言无需（也不可能）再运行一次。
    if matches!(result, AstTypeOrPack::Pack(_)) {
      return result;
    }

    self.recursion_counter = old_recursion_count;

    // `as_type()` 取不出类型即 cpp 的 null 哨兵，parse_type_suffix 同样按「无前导类型」处理。
    AstTypeOrPack::from_type(self.parse_type_suffix(result.as_type().map(NonNull::from), &begin))
  }
}
