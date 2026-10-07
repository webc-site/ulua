use crate::{
  enums::type_lexer::Type,
  functions::{optional_node::node_opt, should_parse_type_pack::should_parse_type_pack},
  records::{
    ast_array::AstArray, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, cst_generic_type::CstGenericType,
    cst_generic_type_pack::CstGenericTypePack, match_lexeme::MatchLexeme, parser::Parser,
    position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_generic_type_list(
    &mut self,
    with_default_values: bool,
    open_position: Option<&mut Position>,
    comma_positions: Option<&mut TempVector<'_, Position>>,
    close_position: Option<&mut Position>,
  ) -> (
    AstArray<*mut AstGenericType>,
    AstArray<*mut AstGenericTypePack>,
  ) {
    let mut comma_positions = comma_positions;
    let mut names: TempVector<'_, *mut AstGenericType> =
      TempVector::new(&mut self.scratch_generic_types);
    let mut name_packs: TempVector<'_, *mut AstGenericTypePack> =
      TempVector::new(&mut self.scratch_generic_type_packs);

    if self.lexer.current().r#type == Type::LESS {
      let begin = self.lexer.current();
      // cpp Parser.cpp:4577 保存 '<' 的 lexeme，稍后以 MatchLexeme(begin) 交给
      // expectMatchAndConsume('>')，报错信息据此定位 '<' 的列号。
      let begin_match = MatchLexeme::new(begin);
      if let Some(open_position) = open_position {
        *open_position = begin.location.begin;
      }
      self.next_lexeme();

      let mut seen_pack = false;
      let mut seen_default = false;

      loop {
        let name_location = self.lexer.current().location;
        // C++ `parseName()` here passes NO context (nullptr default), so the
        // error is "Expected identifier, got X" — not "...when parsing
        // generic type parameter". Pass "" to match.
        let name = self.parse_name("");
        if self.lexer.current().r#type == Type::DOT3 || seen_pack {
          seen_pack = true;

          let mut ellipsis_position = Position::missing();
          if self.lexer.current().r#type != Type::DOT3 {
            self.report(
              self.lexer.current().location,
              format_args!("Generic types come before generic type packs"),
            );
          } else {
            ellipsis_position = self.lexer.current().location.begin;
            self.next_lexeme();
          }

          if with_default_values && self.lexer.current().r#type == Type::EQUAL_SIGN {
            seen_default = true;
            let equals_position = self.lexer.current().location.begin;
            self.next_lexeme();

            if should_parse_type_pack(&mut self.lexer) {
              let type_pack = self.parse_type_pack();

              let node = self.alloc(AstGenericTypePack::new(name_location, name.name, type_pack));
              self.attach_cst(node, |alloc| {
                alloc.alloc(CstGenericTypePack::new(ellipsis_position, equals_position))
              });
              name_packs.push_back(node);
            } else {
              let type_or_pack = self.parse_simple_type_or_pack();

              if let Some(t) = type_or_pack.as_type() {
                self.report(
                  t.base.location,
                  format_args!("Expected type pack after '=', got type"),
                );
              }

              let default_type_pack = type_or_pack.as_pack_node();

              let node = self.alloc(AstGenericTypePack::new(
                name_location,
                name.name,
                default_type_pack,
              ));
              self.attach_cst(node, |alloc| {
                alloc.alloc(CstGenericTypePack::new(ellipsis_position, equals_position))
              });
              name_packs.push_back(node);
            }
          } else {
            if seen_default {
              self.report(
                self.lexer.current().location,
                format_args!("Expected default type pack after type pack name"),
              );
            }

            let node = self.alloc(AstGenericTypePack::new(name_location, name.name, None));
            self.attach_cst(node, |alloc| {
              alloc.alloc(CstGenericTypePack::new(
                ellipsis_position,
                Position::missing(),
              ))
            });
            name_packs.push_back(node);
          }
        } else {
          if with_default_values && self.lexer.current().r#type == Type::EQUAL_SIGN {
            seen_default = true;
            let equals_position = self.lexer.current().location.begin;
            self.next_lexeme();

            let default_type = self.parse_type(false);

            let node = self.alloc(AstGenericType::new(
              name_location,
              name.name,
              node_opt(default_type),
            ));
            if self.options.store_cst_data {
              self.attach_cst(node, |alloc| {
                alloc.alloc(CstGenericType::new(equals_position))
              });
            }
            names.push_back(node);
          } else {
            if seen_default {
              self.report(
                self.lexer.current().location,
                format_args!("Expected default type after type name"),
              );
            }

            let node = self.alloc(AstGenericType::new(name_location, name.name, None));
            if self.options.store_cst_data {
              self.attach_cst(node, |alloc| {
                alloc.alloc(CstGenericType::new(Position::missing()))
              });
            }
            names.push_back(node);
          }
        }

        if self.lexer.current().r#type == Type::COMMA {
          if let Some(comma_positions) = comma_positions.as_deref_mut() {
            comma_positions.push_back(self.lexer.current().location.begin);
          }
          self.next_lexeme();

          if self.lexer.current().r#type == Type::GREATER {
            self.report(
              self.lexer.current().location,
              format_args!("Expected type after ',' but got '>' instead"),
            );
            break;
          }
        } else {
          break;
        }
      }

      let closing_bracket_found = self.expect_match_and_consume('>', &begin_match, false);
      if let Some(close_position) = close_position
        && closing_bracket_found
      {
        *close_position = self.lexer.previous_location().begin;
      }
    }

    let generics: AstArray<*mut AstGenericType> = self.copy_temp_vector_t(&names);
    let generic_packs: AstArray<*mut AstGenericTypePack> = self.copy_temp_vector_t(&name_packs);
    (generics, generic_packs)
  }
}
