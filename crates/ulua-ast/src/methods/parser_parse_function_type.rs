use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup, ast_type_list::AstTypeList, ast_type_or_pack::AstTypeOrPack,
    ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    cst_type_function::CstTypeFunction, cst_type_group::CstTypeGroup,
    cst_type_pack_explicit::CstTypePackExplicit, location::Location, match_lexeme::MatchLexeme,
    parser::Parser, position::Position, temp_vector::TempVector,
  },
  rtti::ast_node_is,
};

impl Parser {
  pub fn parse_function_type(
    &mut self,
    allow_pack: bool,
    attributes: &AstArray<*mut AstAttr>,
  ) -> AstTypeOrPack {
    self.increment_recursion_counter("type annotation");

    let mut force_function_type = self.lexer.current().r#type == Type::LESS;

    let begin = *self.lexer.current();

    let mut generics_open_position = Position::missing();
    let mut generics_comma_positions = AstArray::EMPTY;
    let mut generics_close_position = Position::missing();

    let (generics, generic_packs) = if self.options.store_cst_data {
      let mut local_generics_comma_positions = TempVector::new(&mut self.scratch_position);
      let res = self.parse_generic_type_list(
        false,
        Some(&mut generics_open_position),
        Some(&mut local_generics_comma_positions),
        Some(&mut generics_close_position),
      );
      generics_comma_positions = self.copy_temp_vector_t(&local_generics_comma_positions);
      res
    } else {
      self.parse_generic_type_list(false, None, None, None)
    };

    let parameter_start = *self.lexer.current();

    let open_args_found = self.expect_and_consume_type(Type::LPAREN, "function parameters");

    self.match_recovery_stop_on_token[Type::SKINNY_ARROW.0 as usize] += 1;

    let mut params = TempVector::new(&mut self.scratch_type);
    let mut names = TempVector::new(&mut self.scratch_opt_arg_name);
    let mut name_colon_positions = TempVector::new(&mut self.scratch_position);
    let mut arg_comma_positions = TempVector::new(&mut self.scratch_position_2);
    // cpp `AstTypePack* varargAnnotation = nullptr`（Parser.cpp:1385）：无 `...` 尾注即 None。
    let mut vararg_annotation: Option<NonNull<AstTypePack>> = None;

    if self.lexer.current().r#type != Type::RPAREN {
      // cpp：storeCstData 时把位置向量传下去收集，否则传 nullptr（即 None）。
      let (comma, name_colon) = if self.options.store_cst_data {
        (
          Some(&mut arg_comma_positions),
          Some(&mut name_colon_positions),
        )
      } else {
        (None, None)
      };
      vararg_annotation = self.parse_type_list(&mut params, &mut names, comma, name_colon);
    }

    let close_args_location = self.lexer.current().location;
    let close_args_found =
      self.expect_match_and_consume(')', &MatchLexeme::new(&parameter_start), true);

    self.match_recovery_stop_on_token[Type::SKINNY_ARROW.0 as usize] -= 1;

    let param_types = self.copy_temp_vector_t(&params);

    if !names.is_empty() {
      force_function_type = true;
    }

    let return_type_introducer = self.lexer.current().r#type == Type::SKINNY_ARROW
      || self.lexer.current().r#type == Type::COLON;

    // Not a function at all. Just a parenthesized type. Or maybe a type pack with a single element
    if params.len() == 1
      && vararg_annotation.is_none()
      && !force_function_type
      && !return_type_introducer
    {
      if allow_pack {
        let node = self.alloc_type_pack(AstTypePackExplicit::new(
          begin.location,
          AstTypeList::new(param_types, None),
        ));
        if self.options.store_cst_data {
          let open_pos = if open_args_found {
            parameter_start.location.begin
          } else {
            Position::missing()
          };
          let close_pos = if close_args_found {
            close_args_location.begin
          } else {
            Position::missing()
          };
          let comma_positions_array = self.copy_temp_vector_t(&arg_comma_positions);
          self.attach_cst(node, |alloc| {
            alloc.alloc(CstTypePackExplicit::with_positions(
              open_pos,
              close_pos,
              comma_positions_array,
            ))
          });
        }
        return AstTypeOrPack::from_type_pack(node);
      } else {
        let node = self.alloc_type(AstTypeGroup::new(
          Location::new(parameter_start.location.begin, close_args_location.end),
          params[0],
        ));
        if self.options.store_cst_data {
          let close_pos = if close_args_found {
            close_args_location.begin
          } else {
            Position::missing()
          };
          self.attach_cst(node, |alloc| alloc.alloc(CstTypeGroup::new(close_pos)));
        }
        return AstTypeOrPack::from_type(node);
      }
    }

    if !force_function_type && !return_type_introducer && allow_pack {
      let node = self.alloc_type_pack(AstTypePackExplicit::new(
        begin.location,
        AstTypeList::new(param_types, vararg_annotation),
      ));
      if self.options.store_cst_data {
        let open_pos = if open_args_found {
          parameter_start.location.begin
        } else {
          Position::missing()
        };
        let close_pos = if close_args_found {
          close_args_location.begin
        } else {
          Position::missing()
        };
        let comma_positions_array = self.copy_temp_vector_t(&arg_comma_positions);
        self.attach_cst(node, |alloc| {
          alloc.alloc(CstTypePackExplicit::with_positions(
            open_pos,
            close_pos,
            comma_positions_array,
          ))
        });
      }
      return AstTypeOrPack::from_type_pack(node);
    }

    let param_names = self.copy_temp_vector_t(&names);
    let return_arrow_position = self.lexer.current().location.begin;
    let node = self.parse_function_type_tail(
      &begin,
      *attributes,
      generics,
      generic_packs,
      param_types,
      param_names,
      vararg_annotation,
    );

    if self.options.store_cst_data
      // Safety: node 是 parse_function_type_tail 的 arena 产物（alloc 恒非空；判空折叠为
      // None 即整条不成立，等价旧守卫）；物化共享引用后 ast_node_is::<AstTypeFunction>
      // 仅读偏移 0 的 class_index 做类型判定（只读），单线程下节点未被并发改写，判定与
      // 后续 attach CST 一致。
      && unsafe { node.as_ref() }.is_some_and(ast_node_is::<AstTypeFunction>)
    {
      let names_colon_array = self.copy_temp_vector_t(&name_colon_positions);
      let args_comma_array = self.copy_temp_vector_t(&arg_comma_positions);
      let open_args_pos = if open_args_found {
        parameter_start.location.begin
      } else {
        Position::missing()
      };
      let close_args_pos = if close_args_found {
        close_args_location.begin
      } else {
        Position::missing()
      };
      self.attach_cst(node, |alloc| {
        alloc.alloc(CstTypeFunction::new(
          generics_open_position,
          generics_comma_positions,
          generics_close_position,
          open_args_pos,
          names_colon_array,
          args_comma_array,
          close_args_pos,
          return_arrow_position,
        ))
      });
    }

    AstTypeOrPack::from_type(node)
  }
}
