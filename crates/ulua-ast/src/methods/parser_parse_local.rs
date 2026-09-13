use core::ptr::null_mut;

use crate::{
  enums::type_lexer::Type,
  functions::is_enough_values::is_enough_values,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction, cst_node::CstNode,
    cst_stat_local::CstStatLocal, cst_stat_local_function::CstStatLocalFunction,
    location::Location, parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_local(
    &mut self,
    start: Location,
    keyword_position: Position,
    attributes: &AstArray<*mut AstAttr>,
    is_const: bool,
  ) -> *mut AstStat {
    if !is_const {
      self.next_lexeme();
    }

    if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      let mut match_function = *self.lexer.current();
      self.next_lexeme();

      // C++ patches matchFunction's column to where `local` starts so that
      // `local function` and a column-0 `end` align for the missed-indentation
      // "did you forget to close X" suspect heuristic. The port patched only a
      // COPY (function_keyword_position) but passed the UNPATCHED match_function
      // to parse_function_body, so the end-match compared the `function` keyword
      // column (6) against `end` (0) and recorded the wrong suspect.
      let function_keyword_position = match_function.location.begin;
      if match_function.location.begin.line == start.begin.line {
        match_function.location.begin.column = start.begin.column;
      }

      let name = self.parse_name("variable name");

      self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] += 1;

      let (body, var) = self.parse_function_body(
        false,
        &match_function,
        &name.name,
        Some(&name),
        attributes,
        is_const,
      );

      self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] -= 1;

      let location = Location::new(start.begin, unsafe { (*body).base.base.location.end });

      let node = unsafe {
        (*self.allocator).alloc(AstStatLocalFunction::new(location, var, body, is_const))
      };

      if self.options.store_cst_data {
        let cst_node = unsafe {
          (*self.allocator).alloc(CstStatLocalFunction::new(
            keyword_position,
            function_keyword_position,
          ))
        };
        self
          .cst_node_map
          .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
      }

      node as *mut AstStat
    } else {
      if attributes.size != 0 {
        let current = *self.lexer.current();
        return self.report_stat_error(
          current.location,
          AstArray {
            data: null_mut(),
            size: 0,
          },
          AstArray {
            data: null_mut(),
            size: 0,
          },
          format_args!(
            "Expected 'function' after local declaration with attribute, but got {current} instead",
          ),
        ) as *mut AstStat;
      }

      self.match_recovery_stop_on_token[Type::OPERATOR.0 as usize] += 1;

      let mut names = TempVector::new(&mut self.scratch_binding);
      let mut vars_comma_positions = AstArray {
        data: null_mut(),
        size: 0,
      };

      if self.options.store_cst_data {
        self.parse_binding_list(
          &mut names,
          false,
          &mut vars_comma_positions,
          null_mut(),
          null_mut(),
          is_const,
        );
      } else {
        self.parse_binding_list(
          &mut names,
          false,
          null_mut(),
          null_mut(),
          null_mut(),
          is_const,
        );
      }

      self.match_recovery_stop_on_token[Type::OPERATOR.0 as usize] -= 1;

      let mut vars = TempVector::new(&mut self.scratch_local);
      let mut values = TempVector::new(&mut self.scratch_expr);
      let mut values_comma_positions = TempVector::new(&mut self.scratch_position);

      let mut equals_sign_location = None;

      if self.lexer.current().r#type == Type::OPERATOR {
        equals_sign_location = Some(self.lexer.current().location);
        self.next_lexeme();

        if self.options.store_cst_data {
          self.parse_expr_list(&mut values, Some(&mut values_comma_positions));
        } else {
          self.parse_expr_list(&mut values, None);
        }
      }

      for name in names.iter() {
        vars.push_back(self.push_local(name));
      }

      let end = if values.empty() {
        *self.lexer.previous_location()
      } else {
        unsafe { (**values.back()).base.location }
      };

      let node = unsafe {
        (*self.allocator).alloc(AstStatLocal::new(
          Location::new(start.begin, end.end),
          self.copy_temp_vector_t(&vars),
          self.copy_temp_vector_t(&values),
          equals_sign_location,
          is_const,
        ))
      };

      if self.options.store_cst_data {
        let cst_node = unsafe {
          (*self.allocator).alloc(CstStatLocal::new(
            self.extract_annotation_colon_positions(&names),
            vars_comma_positions,
            self.copy_temp_vector_t(&values_comma_positions),
          ))
        };
        self
          .cst_node_map
          .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
      }

      // const 声明必然值不够时（如 `const foo`、`const bar, baz = 42`）报错，
      // 但声明本身仍合法，按原样返回节点。
      if is_const && !is_enough_values(&mut values, vars.size()) {
        self.report(
          unsafe { (*node).base.base.location },
          format_args!("Missing initializer in const declaration"),
        );
      }

      node as *mut AstStat
    }
  }
}
