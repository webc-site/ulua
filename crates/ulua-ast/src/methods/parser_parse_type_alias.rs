//! Source: `Ast/src/Parser.cpp:1405`
//!
//! Faithful port of `Parser::parseTypeAlias` — `type Name<generics> = Type`
//! (delegating to `parse_type_function` for `type function`). The `type` keyword
//! is already consumed by the caller. A missing name falls back to the parser's
//! error name. Generic comma positions are gathered in a scratch arena and
//! copied into the CST node only under `store_cst_data`.

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_stat::AstStat, ast_stat_type_alias::AstStatTypeAlias,
    cst_stat_type_alias::CstStatTypeAlias, location::Location, name::Name, parser::Parser,
    position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_type_alias(
    &mut self,
    start: &Location,
    exported: bool,
    type_keyword_position: Position,
  ) -> *mut AstStat {
    // parsing a type function
    if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      return self.parse_type_function(start, exported, type_keyword_position);
    }

    // note: `type` token is already parsed for us, so we just parse the rest
    let name = match self.parse_name_opt("type name") {
      Some(name) => name,
      // Use error name if the name is missing
      None => Name {
        name: self.name_error,
        location: self.lexer.current().location,
      },
    };

    let mut generics_open_position = Position::missing();
    let mut generics_close_position = Position::missing();
    let mut generics_comma_positions = TempVector::new(&mut self.scratch_position);
    let (generics, generic_packs) = if self.options.store_cst_data {
      self.parse_generic_type_list(
        true,
        Some(&mut generics_open_position),
        Some(&mut generics_comma_positions),
        Some(&mut generics_close_position),
      )
    } else {
      self.parse_generic_type_list(true, None, None, None)
    };

    let equals_found = self.expect_and_consume_char('=', "type alias");
    let equals_position = if equals_found {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let type_ = self.parse_type(false);

    let node = self.alloc_stat(AstStatTypeAlias::new_simple(
      Location::new(start.begin, unsafe {
        // Safety: type_ 为 61 行 parse_type 刚 arena 分配的存活类型节点（恒非空），(*type_).base.location.end 为基类前缀字段只读拷贝。
        (*type_).base.location.end
      }),
      name.name,
      name.location,
      generics,
      generic_packs,
      type_,
      exported,
    ));

    if self.options.store_cst_data {
      let generics_comma = self.copy_temp_vector_t(&generics_comma_positions);
      self.attach_cst(node, |alloc| {
        alloc.alloc(CstStatTypeAlias::new(
          type_keyword_position,
          generics_open_position,
          generics_comma,
          generics_close_position,
          equals_position,
        ))
      });
    }

    node
  }
}
