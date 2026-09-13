use core::ptr::null_mut;

use crate::{
  functions::{is_type_follow::is_type_follow, should_parse_type_pack::should_parse_type_pack},
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_type::AstType, ast_type_group::AstTypeGroup,
    ast_type_or_pack::AstTypeOrPack, ast_type_pack_explicit::AstTypePackExplicit,
    cst_node::CstNode, cst_type_group::CstTypeGroup, cst_type_pack_explicit::CstTypePackExplicit,
    lexeme::Type, match_lexeme::MatchLexeme, parser::Parser, position::Position,
    temp_vector::TempVector,
  },
  rtti::{ast_node_as, cst_node_as},
};

impl Parser {
  pub fn parse_type_params(
    &mut self,
    opening_position: Option<&mut Position>,
    mut comma_positions: Option<&mut TempVector<'_, Position>>,
    closing_position: Option<&mut Position>,
  ) -> AstArray<AstTypeOrPack> {
    let mut parameters: Vec<AstTypeOrPack> = Vec::new();

    if self.lexer.current().r#type == Type::LESS {
      let begin = *self.lexer.current();
      if let Some(pos) = opening_position {
        *pos = begin.location.begin;
      }
      self.next_lexeme();

      loop {
        if should_parse_type_pack(&mut self.lexer) {
          let type_pack = self.parse_type_pack();
          parameters.push(AstTypeOrPack {
            r#type: null_mut(),
            type_pack,
          });
        } else if self.lexer.current().r#type == Type(b'(' as i32) {
          let begin_loc = self.lexer.current().location;
          let mut type_ = null_mut();
          let mut type_pack = null_mut();
          let c = self.lexer.current().r#type;

          if c != Type::PIPE && c != Type::AMPERSAND {
            let type_or_type_pack = self.parse_simple_type(true, false);
            type_ = type_or_type_pack.r#type;
            type_pack = type_or_type_pack.type_pack;
          }

          if !type_pack.is_null() {
            let explicit_type_pack =
              unsafe { ast_node_as::<AstTypePackExplicit>(type_pack as *mut AstNode) };
            if !explicit_type_pack.is_null()
              && unsafe { (*explicit_type_pack).type_list.tail_type.is_null() }
              && unsafe { (*explicit_type_pack).type_list.types.size == 1 }
              && is_type_follow(self.lexer.current().r#type)
            {
              let parenthesized_type =
                unsafe { *(*explicit_type_pack).type_list.types.data.add(0) };

              // cpp 无 LuauCstTypeGroup flag：AstTypeGroup 无条件创建，
              // CstTypeGroup 仅受 storeCstData 门控
              let type_group = unsafe {
                (*self.allocator).alloc(AstTypeGroup::new(
                  (*parenthesized_type).base.location,
                  parenthesized_type,
                ))
              };

              if self.options.store_cst_data
                && let Some(cst_node) = self
                  .cst_node_map
                  .find(&(explicit_type_pack as *mut AstNode))
                && !cst_node.is_null()
              {
                let cst_explicit_type_pack =
                  unsafe { cst_node_as::<CstTypePackExplicit>(*cst_node) };
                if !cst_explicit_type_pack.is_null() {
                  let close_pos = unsafe { (*cst_explicit_type_pack).close_parentheses_position };
                  let cst_node_group =
                    unsafe { (*self.allocator).alloc(CstTypeGroup::new(close_pos)) };
                  self
                    .cst_node_map
                    .try_insert(type_group as *mut AstNode, cst_node_group as *mut CstNode);
                }
              }

              parameters.push(AstTypeOrPack {
                r#type: self.parse_type_suffix(type_group as *mut AstType, &begin_loc),
                type_pack: null_mut(),
              });
            } else {
              parameters.push(AstTypeOrPack {
                r#type: null_mut(),
                type_pack,
              });
            }
          } else {
            parameters.push(AstTypeOrPack {
              r#type: self.parse_type_suffix(type_, &begin_loc),
              type_pack: null_mut(),
            });
          }
        } else if self.lexer.current().r#type == Type::GREATER && parameters.is_empty() {
          break;
        } else {
          parameters.push(AstTypeOrPack {
            r#type: self.parse_type_bool(false),
            type_pack: null_mut(),
          });
        }

        if self.lexer.current().r#type == Type::COMMA {
          if let Some(vec) = comma_positions.as_deref_mut() {
            vec.push_back(self.lexer.current().location.begin);
          }
          self.next_lexeme();
        } else {
          break;
        }
      }

      let closing_bracket_found =
        self.expect_match_and_consume('>', &MatchLexeme::new(&begin), false);
      if let Some(pos) = closing_position
        && closing_bracket_found
      {
        *pos = self.lexer.previous_location().begin;
      }
    }

    self.copy_initializer_list_t(&parameters)
  }
}
