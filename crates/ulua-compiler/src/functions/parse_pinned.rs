use alloc::boxed::Box;

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
  parse_result::ParseResult, parser::Parser,
};

/// Box 钉堆：AstNameTable/Parser 内部存 `*mut Allocator`（捕获宿主地址），
/// 宿主一旦移动即悬垂（同 tests.rs string_table! 先例）。堆地址永不移动。
/// 所有权元组返回，由调用方持有钉堆不变量。
pub(crate) fn parse_pinned<B>(
  source: &B,
  parse_options: &ParseOptions,
) -> (Box<Allocator>, AstNameTable, ParseResult)
where
  B: AsRef<[u8]> + ?Sized,
{
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(
    source,
    &mut names,
    &mut allocator,
    parse_options.clone(),
  );
  (allocator, names, result)
}
