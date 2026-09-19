use core::{
  ptr::{NonNull, null_mut},
  sync::atomic::Ordering,
};

use ulua_common::{
  dfflag::DebugLuauReportReturnTypeVariadicWithTypeSuffix,
  fflag::LuauSingleTypeOptionalPackReturnsAttributeParens,
};

use crate::{
  enums::type_lexer::Type,
  functions::should_parse_type_pack::should_parse_type_pack,
  records::{
    ast_array::AstArray, ast_type::AstType, ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup, ast_type_intersection::AstTypeIntersection,
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_union::AstTypeUnion,
    cst_type_function::CstTypeFunction, cst_type_group::CstTypeGroup,
    cst_type_pack_explicit::CstTypePackExplicit, location::Location, match_lexeme::MatchLexeme,
    parser::Parser, position::Position, temp_vector::TempVector,
  },
  rtti::ast_node_is,
  type_aliases::ast_argument_name::AstArgumentName,
};

impl Parser {
  pub fn parse_return_type(&mut self) -> *mut AstTypePack {
    self.increment_recursion_counter("type annotation");
    let begin = *self.lexer.current();

    if self.lexer.current().r#type != Type('(' as i32) {
      if should_parse_type_pack(&mut self.lexer) {
        return self.parse_type_pack();
      } else {
        let ty = self.parse_type(false);
        let types_array = self.copy_initializer_list_t(&[ty]);
        let node = unsafe {
          (*self.allocator).alloc(AstTypePackExplicit::new(
            (*ty).base.location,
            AstTypeList {
              types: types_array,
              tail_type: null_mut(),
            },
          ))
        };
        if self.options.store_cst_data {
          self.attach_cst(node, |alloc| alloc.alloc(CstTypePackExplicit::new()));
        }
        return node as *mut AstTypePack;
      }
    }

    self.next_lexeme();
    self.match_recovery_stop_on_token[Type::SKINNY_ARROW.0 as usize] += 1;

    let mut result: TempVector<'_, *mut AstType> = TempVector::new(&mut self.scratch_type);
    let mut result_names: TempVector<'_, Option<AstArgumentName>> =
      TempVector::new(&mut self.scratch_opt_arg_name);
    let mut comma_positions: TempVector<'_, Position> = TempVector::new(&mut self.scratch_position);
    let mut name_colon_positions: TempVector<'_, Position> =
      TempVector::new(&mut self.scratch_position_2);

    // 可选尾注 pack：`None` 即 cpp 的 `nullptr`。
    let mut vararg_annotation: Option<NonNull<AstTypePack>> = None;

    if self.lexer.current().r#type != Type(')' as i32) {
      if self.options.store_cst_data {
        vararg_annotation = NonNull::new(self.parse_type_list(
          &mut result,
          &mut result_names,
          &mut comma_positions,
          &mut name_colon_positions,
        ));
      } else {
        vararg_annotation = NonNull::new(self.parse_type_list(
          &mut result,
          &mut result_names,
          null_mut(),
          null_mut(),
        ));
      }
    }

    let location = Location::new(begin.location.begin, self.lexer.current().location.end);
    let close_paren_found = self.expect_match_and_consume(')', &MatchLexeme::new(&begin), true);
    let close_parentheses_position = if close_paren_found {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    self.match_recovery_stop_on_token[Type::SKINNY_ARROW.0 as usize] -= 1;

    if self.lexer.current().r#type != Type::SKINNY_ARROW && result_names.empty() {
      if result.size() == 1 {
        let inner: *mut AstType;
        let mut parens_belong_to_inner_group = false;

        if LuauSingleTypeOptionalPackReturnsAttributeParens.get() {
          let curr_type = self.lexer.current().r#type;
          let is_type_follow =
            curr_type == Type::PIPE || curr_type == Type::QUESTION || curr_type == Type::AMPERSAND;
          if vararg_annotation.is_none() && is_type_follow {
            inner = unsafe {
              (*self.allocator).alloc(AstTypeGroup::new(location, *result.operator_index(0)))
                as *mut AstType
            };
            parens_belong_to_inner_group = true;
            if self.options.store_cst_data {
              self.attach_cst(inner, |alloc| {
                alloc.alloc(CstTypeGroup::new(if close_paren_found {
                  close_parentheses_position
                } else {
                  Position::missing()
                }))
              });
            }
          } else {
            inner = *result.operator_index(0);
          }
        } else {
          inner = if vararg_annotation.is_none() {
            unsafe {
              (*self.allocator).alloc(AstTypeGroup::new(location, *result.operator_index(0)))
                as *mut AstType
            }
          } else {
            *result.operator_index(0)
          };
          if vararg_annotation.is_none() && self.options.store_cst_data {
            self.attach_cst(inner, |alloc| {
              alloc.alloc(CstTypeGroup::new(if close_paren_found {
                close_parentheses_position
              } else {
                Position::missing()
              }))
            });
          }
        }

        let return_type = self.parse_type_suffix(NonNull::new(inner), &begin.location);

        if DebugLuauReportReturnTypeVariadicWithTypeSuffix.get()
          && vararg_annotation.is_some()
          && (ast_node_is::<AstTypeUnion>(unsafe { &*return_type })
            || ast_node_is::<AstTypeIntersection>(unsafe { &*return_type }))
        {
          crate::LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX
            .store(true, Ordering::Relaxed);
        }

        let end_pos = if result.size() == 1 {
          location.end
        } else {
          unsafe { (*return_type).base.location.end }
        };

        let types_array = self.copy_initializer_list_t(&[return_type]);
        let node = unsafe {
          (*self.allocator).alloc(AstTypePackExplicit::new(
            Location::new(location.begin, end_pos),
            AstTypeList {
              types: types_array,
              tail_type: vararg_annotation.map_or(null_mut(), NonNull::as_ptr),
            },
          ))
        };

        if LuauSingleTypeOptionalPackReturnsAttributeParens.get() {
          if parens_belong_to_inner_group {
            self.attach_cst(node, |alloc| alloc.alloc(CstTypePackExplicit::new()));
          } else {
            let comma_positions_array = self.copy_temp_vector_t(&comma_positions);
            self.attach_cst(node, |alloc| {
              alloc.alloc(CstTypePackExplicit::with_positions(
                location.begin,
                close_parentheses_position,
                comma_positions_array,
              ))
            });
          }
          return node as *mut AstTypePack;
        } else if self.options.store_cst_data {
          self.attach_cst(node, |alloc| alloc.alloc(CstTypePackExplicit::new()));
          return node as *mut AstTypePack;
        }
        return node as *mut AstTypePack;
      }

      let types_array = self.copy_temp_vector_t(&result);
      let node = unsafe {
        (*self.allocator).alloc(AstTypePackExplicit::new(
          location,
          AstTypeList {
            types: types_array,
            tail_type: vararg_annotation.map_or(null_mut(), NonNull::as_ptr),
          },
        ))
      };

      if self.options.store_cst_data {
        let comma_positions_array = self.copy_temp_vector_t(&comma_positions);
        self.attach_cst(node, |alloc| {
          alloc.alloc(CstTypePackExplicit::with_positions(
            location.begin,
            close_parentheses_position,
            comma_positions_array,
          ))
        });
      }
      return node as *mut AstTypePack;
    }

    let return_arrow_position = self.lexer.current().location.begin;
    // C++ passes `copy(result)` / `copy(resultNames)` — allocator copies. The port
    // passed raw pointers into the scratch TempVectors, but parse_function_type_tail
    // recursively parses the `-> ret` which CLEARS scratch_opt_arg_name (and
    // scratch_type), corrupting the names to None. Copy before the recursion.
    let params_copy = self.copy_temp_vector_t(&result);
    let param_names_copy = self.copy_temp_vector_t(&result_names);
    let tail = self.parse_function_type_tail(
      &begin,
      AstArray::EMPTY,
      AstArray::EMPTY,
      AstArray::EMPTY,
      params_copy,
      param_names_copy,
      vararg_annotation.map_or(null_mut(), NonNull::as_ptr),
    );

    if self.options.store_cst_data
      && !tail.is_null()
      && unsafe { ast_node_is::<AstTypeFunction>(&*tail) }
    {
      let name_colon_positions_array = self.copy_temp_vector_t(&name_colon_positions);
      let comma_positions_array = self.copy_temp_vector_t(&comma_positions);
      self.attach_cst(tail, |alloc| {
        alloc.alloc(CstTypeFunction::new(
          Position::missing(),
          AstArray::EMPTY,
          Position::missing(),
          location.begin,
          name_colon_positions_array,
          comma_positions_array,
          close_parentheses_position,
          return_arrow_position,
        ))
      });
    }

    let types_array = self.copy_initializer_list_t(&[tail]);
    let node = unsafe {
      (*self.allocator).alloc(AstTypePackExplicit::new(
        Location::new(location.begin, (*tail).base.location.end),
        AstTypeList {
          types: types_array,
          tail_type: null_mut(),
        },
      ))
    };

    if self.options.store_cst_data {
      self.attach_cst(node, |alloc| alloc.alloc(CstTypePackExplicit::new()));
    }

    node as *mut AstTypePack
  }
}
