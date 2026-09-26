use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_type::AstType,
  parse_node_result::ParseNodeResult, parse_options::ParseOptions, parser::Parser,
};

impl Parser {
  /// cpp 自由函数 `Luau::parseType(buffer, bufferSize, names, allocator, options)`
  /// （`Ast/include/Luau/Parser.h`）：从源码字节解析单个类型。
  ///
  /// 命名去机器味：旧名 `parse_type_c_char_usize_...` 照抄 cpp 形参类型表，
  /// 但签名早已是 `AsRef<[u8]>` 借用形态，C 类型不在签名中，名不副实。
  pub fn parse_type_source<B>(
    buffer: &B,
    names: &mut AstNameTable,
    allocator: &mut Allocator,
    options: ParseOptions,
  ) -> ParseNodeResult<AstType>
  where
    B: AsRef<[u8]> + ?Sized,
  {
    LUAU_TIMETRACE_SCOPE!("Parser::parseType", "Parser");

    Parser::run_parse::<AstType, _>(buffer.as_ref(), names, allocator, options, |parser| {
      parser.parse_type(false)
    })
  }
}
