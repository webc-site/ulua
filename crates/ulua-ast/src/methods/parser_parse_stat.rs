use core::ptr::null_mut;

use ulua_common::FFlag::{DebugLuauUserDefinedClasses, LuauExportValueSyntax};

use crate::{
  functions::get_identifier::get_identifier,
  records::{
    ast_array::AstArray, ast_expr_call::AstExprCall, ast_name::AstName, ast_stat::AstStat,
    ast_stat_expr::AstStatExpr, lexeme::Type, parser::Parser,
  },
  rtti::ast_node_is,
};

impl Parser {
  pub fn parse_stat(&mut self) -> *mut AstStat {
    match self.lexer.current().r#type {
      Type::RESERVED_IF => return self.parse_if(),
      Type::RESERVED_WHILE => return self.parse_while(),
      Type::RESERVED_DO => return self.parse_do(),
      Type::RESERVED_FOR => return self.parse_for(),
      Type::RESERVED_REPEAT => return self.parse_repeat(),
      Type::RESERVED_FUNCTION => {
        return self.parse_function_stat(&AstArray {
          data: null_mut(),
          size: 0,
        }) as *mut AstStat;
      }
      Type::RESERVED_LOCAL => {
        let start = self.lexer.current().location;
        return self.parse_local(
          start,
          start.begin,
          &AstArray {
            data: null_mut(),
            size: 0,
          },
          false,
        );
      }
      Type::RESERVED_RETURN => return self.parse_return(),
      Type::RESERVED_BREAK => return self.parser_parse_break(),
      Type::ATTRIBUTE | Type::ATTRIBUTE_OPEN => return self.parse_attribute_stat(),
      _ => {}
    }

    let start = self.lexer.current().location;
    let expr = self.parse_primary_expr(true);

    if unsafe { ast_node_is::<AstExprCall>(&*expr) } {
      return unsafe {
        (*self.allocator).alloc(AstStatExpr::new((*expr).base.location, expr)) as *mut AstStat
      };
    }

    let current_type = self.lexer.current().r#type;
    if current_type == Type(',' as i32) || current_type == Type('=' as i32) {
      return self.parse_assignment(expr);
    }

    if let Some(op) = self.parse_compound_op(self.lexer.current()) {
      return self.parse_compound_assignment(expr, op);
    }

    let ident = get_identifier(expr);

    if ident.operator_eq_c_char(c"type") {
      return self.parse_type_alias(&unsafe { (*expr).base.location }, false, unsafe {
        (*expr).base.location.begin
      });
    }

    if DebugLuauUserDefinedClasses.get() && ident.operator_eq_c_char(c"class") {
      return self.parse_class_stat(&start, false, false);
    } else if DebugLuauUserDefinedClasses.get() && ident.operator_eq_c_char(c"open") {
      if self.lexer.current().r#type != Type::NAME
        || unsafe {
          !AstName::ast_name_c_char(self.lexer.current().data.name).operator_eq_c_char(c"class")
        }
      {
        let exprs = self.copy_initializer_list_t(&[expr]);
        let stats = self.copy_initializer_list_t(&[]);
        return self.report_stat_error(
          unsafe { (*expr).base.location },
          exprs,
          stats,
          format_args!("Incomplete statement: expected a class definition after 'open'"),
        ) as *mut AstStat;
      }

      self.next_lexeme(); // consume `class`
      return self.parse_class_stat(&start, false, true);
    }

    if ident.operator_eq_c_char(c"export") {
      if LuauExportValueSyntax.get() {
        let current = self.lexer.current();
        let is_local = current.r#type == Type::RESERVED_LOCAL;
        let is_function = current.r#type == Type::RESERVED_FUNCTION;
        let is_const = current.r#type == Type::NAME
          && AstName::ast_name_c_char(unsafe { current.data.name }).operator_eq_c_char(c"const");
        let is_class = DebugLuauUserDefinedClasses.get()
          && current.r#type == Type::NAME
          && (AstName::ast_name_c_char(unsafe { current.data.name }).operator_eq_c_char(c"class")
            || AstName::ast_name_c_char(unsafe { current.data.name }).operator_eq_c_char(c"open"));

        if is_local || is_function || is_const || is_class {
          return self.parse_export_value(
            &unsafe { (*expr).base.location },
            unsafe { (*expr).base.location.begin },
            &AstArray {
              data: null_mut(),
              size: 0,
            },
          );
        } else if current.r#type == Type::NAME
          && AstName::ast_name_c_char(unsafe { current.data.name }).operator_eq_c_char(c"type")
        {
          let type_keyword_position = current.location.begin;
          self.next_lexeme();
          return self.parse_type_alias(
            &unsafe { (*expr).base.location },
            true,
            type_keyword_position,
          );
        }
      } else if DebugLuauUserDefinedClasses.get()
        && AstName::ast_name_c_char(unsafe { self.lexer.current().data.name })
          .operator_eq_c_char(c"class")
      {
        self.next_lexeme();
        return self.parse_class_stat(&start, true, false);
      } else if DebugLuauUserDefinedClasses.get()
        && AstName::ast_name_c_char(unsafe { self.lexer.current().data.name })
          .operator_eq_c_char(c"open")
      {
        self.next_lexeme(); // consume `open`
        if self.lexer.current().r#type != Type::NAME
          || unsafe {
            !AstName::ast_name_c_char(self.lexer.current().data.name).operator_eq_c_char(c"class")
          }
        {
          let exprs = self.copy_initializer_list_t(&[expr]);
          let stats = self.copy_initializer_list_t(&[]);
          return self.report_stat_error(
            unsafe { (*expr).base.location },
            exprs,
            stats,
            format_args!("Incomplete statement: expected a class definition after 'open'"),
          ) as *mut AstStat;
        }

        self.next_lexeme(); // consume `class`
        return self.parse_class_stat(&start, true, true);
      } else if self.lexer.current().r#type == Type::NAME
        && AstName::ast_name_c_char(unsafe { self.lexer.current().data.name })
          .operator_eq_c_char(c"type")
      {
        let type_keyword_position = self.lexer.current().location.begin;
        self.next_lexeme();
        return self.parse_type_alias(
          &unsafe { (*expr).base.location },
          true,
          type_keyword_position,
        );
      }
    }

    if ident.operator_eq_c_char(c"continue") {
      return self.parser_parse_continue(&unsafe { (*expr).base.location });
    }

    if ident.operator_eq_c_char(c"const") {
      return self.parse_local(
        unsafe { (*expr).base.location },
        unsafe { (*expr).base.location.begin },
        &AstArray {
          data: null_mut(),
          size: 0,
        },
        true,
      );
    }

    if self.options.allow_declaration_syntax && ident.operator_eq_c_char(c"declare") {
      return self.parse_declaration(
        &unsafe { (*expr).base.location },
        &AstArray {
          data: null_mut(),
          size: 0,
        },
      );
    }

    if start
      .begin
      .operator_eq(&self.lexer.current().location.begin)
    {
      self.next_lexeme();
    }

    let expr_location = unsafe { (*expr).base.location };
    let exprs = self.copy_initializer_list_t(&[expr]);

    self.report_stat_error(
      expr_location,
      exprs,
      AstArray {
        data: null_mut(),
        size: 0,
      },
      format_args!("Incomplete statement: expected assignment or a function call"),
    ) as *mut AstStat
  }
}
