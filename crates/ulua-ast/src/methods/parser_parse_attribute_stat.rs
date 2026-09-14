use core::ptr::null_mut;

use ulua_common::FFlag::LuauExportValueSyntax;

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_name::AstName, ast_stat::AstStat,
    ast_stat_error::AstStatError, parser::Parser,
  },
};

impl Parser {
  pub fn parse_attribute_stat(&mut self) -> *mut AstStat {
    let attributes = self.parse_attributes();
    let current_type = self.lexer.current().r#type;

    match current_type {
      Type::RESERVED_FUNCTION => self.parse_function_stat(&attributes) as *mut AstStat,
      Type::RESERVED_LOCAL => {
        let attr_loc = if attributes.size > 0 {
          unsafe { (**attributes.data.add(0)).base.location }
        } else {
          self.lexer.current().location
        };

        self.parse_local(
          attr_loc,
          self.lexer.current().location.begin,
          &attributes,
          false,
        )
      }
      Type::NAME => {
        let current = self.lexer.current();
        let current_name = unsafe { current.data.name };

        if LuauExportValueSyntax.get()
          && AstName::operator_eq_c_char(
            &AstName {
              value: current_name,
            },
            c"export",
          )
        {
          let keyword_loc = current.location;
          self.next_lexeme();

          let attr_loc = if attributes.size > 0 {
            unsafe { (**attributes.data.add(0)).base.location }
          } else {
            keyword_loc
          };

          self.parse_export_value(&attr_loc, keyword_loc.begin, &attributes)
        } else if AstName::operator_eq_c_char(
          &AstName {
            value: current_name,
          },
          c"const",
        ) {
          let keyword_loc = current.location;
          self.next_lexeme();

          let attr_loc = if attributes.size > 0 {
            unsafe { (**attributes.data.add(0)).base.location }
          } else {
            keyword_loc
          };

          self.parse_local(attr_loc, keyword_loc.begin, &attributes, true)
        } else if self.options.allow_declaration_syntax
          && AstName::operator_eq_c_char(
            &AstName {
              value: current_name,
            },
            c"declare",
          )
        {
          let expr = self.parse_primary_expr(true);
          self.parse_declaration(&unsafe { (*expr).base.location }, &attributes)
        } else {
          self.parse_attribute_stat_fallthrough_to_error(&attributes) as *mut AstStat
        }
      }
      _ => self.parse_attribute_stat_fallthrough_to_error(&attributes) as *mut AstStat,
    }
  }

  fn parse_attribute_stat_fallthrough_to_error(
    &mut self,
    _attributes: &AstArray<*mut AstAttr>,
  ) -> *mut AstStatError {
    let current = *self.lexer.current();
    let loc = current.location;

    let empty_exprs = AstArray {
      data: null_mut(),
      size: 0,
    };
    let empty_lines = AstArray {
      data: null_mut(),
      size: 0,
    };

    self.report_stat_error(
      loc,
      empty_exprs,
      empty_lines,
      format_args!(
        "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got {current} instead"
      ),
    )
  }
}
