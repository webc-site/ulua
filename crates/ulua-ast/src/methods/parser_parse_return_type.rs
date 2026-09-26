use core::{ptr::NonNull, sync::atomic::Ordering};

use ulua_common::{
  dfflag::DebugLuauReportReturnTypeVariadicWithTypeSuffix,
  fflag::LuauSingleTypeOptionalPackReturnsAttributeParens,
};

use crate::{
  enums::type_lexer::Type,
  functions::{
    optional_node::{node_opt, slot_opt, slot_ref},
    should_parse_type_pack::should_parse_type_pack,
  },
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
  /// cpp `Parser::parseReturnType`（`Parser.cpp:2600`）：`Type | '(' TypeList ')'`。
  ///
  /// 返回 `None` 只可能是 `parse_type_pack` 断言分支的产物（该分支已被这里的
  /// `should_parse_type_pack` 守卫排除），与 cpp `return nullptr` 逐位同形。
  pub fn parse_return_type(&mut self) -> Option<NonNull<AstTypePack>> {
    self.increment_recursion_counter("type annotation");
    let begin = *self.lexer.current();

    if self.lexer.current().r#type != Type::LPAREN {
      if should_parse_type_pack(&mut self.lexer) {
        return self.parse_type_pack();
      } else {
        let ty = self.parse_type(false);
        let types_array = self.copy_initializer_list_t(&[ty]);
        let node = self.alloc_type_pack(AstTypePackExplicit::new(
          slot_ref(ty).base.location,
          AstTypeList::new(types_array, None),
        ));
        if self.options.store_cst_data {
          self.attach_cst(node, |alloc| alloc.alloc(CstTypePackExplicit::new()));
        }
        return node_opt(node);
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

    if self.lexer.current().r#type != Type::RPAREN {
      // cpp：storeCstData 时把位置向量传下去收集，否则传 nullptr（即 None）。
      let (comma, name_colon) = if self.options.store_cst_data {
        (Some(&mut comma_positions), Some(&mut name_colon_positions))
      } else {
        (None, None)
      };
      vararg_annotation = self.parse_type_list(&mut result, &mut result_names, comma, name_colon);
    }

    let location = Location::new(begin.location.begin, self.lexer.current().location.end);
    let close_parentheses_position =
      self.expect_match_and_consume_position(')', &MatchLexeme::new(&begin), true);

    self.match_recovery_stop_on_token[Type::SKINNY_ARROW.0 as usize] -= 1;

    if self.lexer.current().r#type != Type::SKINNY_ARROW && result_names.is_empty() {
      if result.len() == 1 {
        let inner: *mut AstType;
        let mut parens_belong_to_inner_group = false;

        if LuauSingleTypeOptionalPackReturnsAttributeParens.get() {
          let curr_type = self.lexer.current().r#type;
          let is_type_follow =
            curr_type == Type::PIPE || curr_type == Type::QUESTION || curr_type == Type::AMPERSAND;
          if vararg_annotation.is_none() && is_type_follow {
            inner = self.alloc_type(AstTypeGroup::new(location, result[0]));
            parens_belong_to_inner_group = true;
            if self.options.store_cst_data {
              self.attach_cst(inner, |alloc| {
                // close_parentheses_position 未命中时即 missing，无需再按
                // found 标志二选一（与被收敛的旧 if/else 逐值相等）。
                alloc.alloc(CstTypeGroup::new(close_parentheses_position))
              });
            }
          } else {
            inner = result[0];
          }
        } else {
          inner = if vararg_annotation.is_none() {
            self.alloc_type(AstTypeGroup::new(location, result[0]))
          } else {
            result[0]
          };
          if vararg_annotation.is_none() && self.options.store_cst_data {
            self.attach_cst(inner, |alloc| {
              // 契约同上：未命中即 missing，直接用位置值。
              alloc.alloc(CstTypeGroup::new(close_parentheses_position))
            });
          }
        }

        let return_type = self.parse_type_suffix(NonNull::new(inner), &begin.location);
        let return_type_ref = slot_ref(return_type);

        if DebugLuauReportReturnTypeVariadicWithTypeSuffix.get()
          && vararg_annotation.is_some()
          && (ast_node_is::<AstTypeUnion>(return_type_ref)
            || ast_node_is::<AstTypeIntersection>(return_type_ref))
        {
          crate::LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX
            .store(true, Ordering::Relaxed);
        }

        let end_pos = if result.len() == 1 {
          location.end
        } else {
          return_type_ref.base.location.end
        };

        let types_array = self.copy_initializer_list_t(&[return_type]);
        let node = self.alloc_type_pack(AstTypePackExplicit::new(
          Location::new(location.begin, end_pos),
          AstTypeList::new(types_array, vararg_annotation),
        ));

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
          return node_opt(node);
        } else if self.options.store_cst_data {
          self.attach_cst(node, |alloc| alloc.alloc(CstTypePackExplicit::new()));
          return node_opt(node);
        }
        return node_opt(node);
      }

      let types_array = self.copy_temp_vector_t(&result);
      let node = self.alloc_type_pack(AstTypePackExplicit::new(
        location,
        AstTypeList::new(types_array, vararg_annotation),
      ));

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
      return node_opt(node);
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
      vararg_annotation,
    );

    if self.options.store_cst_data
      // tail 可空：`slot_opt` 折叠 null（等价旧 `!tail.is_null()` 前置），Some 侧只读判型。
      && slot_opt(tail).is_some_and(ast_node_is::<AstTypeFunction>)
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
    let node = self.alloc_type_pack(AstTypePackExplicit::new(
      Location::new(location.begin, slot_ref(tail).base.location.end),
      AstTypeList::new(types_array, None),
    ));

    if self.options.store_cst_data {
      self.attach_cst(node, |alloc| alloc.alloc(CstTypePackExplicit::new()));
    }

    node_opt(node)
  }
}
