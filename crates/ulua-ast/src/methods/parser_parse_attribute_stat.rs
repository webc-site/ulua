use ulua_common::fflag::LuauExportValueSyntax;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_stat::AstStat, location::Location, parser::Parser,
  },
};

impl Parser {
  /// attributes 首元素的基类 location（无属性时回退 `fallback`）：属性节点由
  /// parse_attributes 以 arena 分配产出，size>0 门控下首元素恒非空存活。
  pub(crate) fn first_attr_location(
    &self,
    attributes: &AstArray<*mut AstAttr>,
    fallback: Location,
  ) -> Location {
    if attributes.size > 0 {
      slot_ref(attributes.as_slice()[0]).base.location
    } else {
      fallback
    }
  }

  pub fn parse_attribute_stat(&mut self) -> *mut AstStat {
    let attributes = self.parse_attributes();
    let current_type = self.lexer.current().r#type;

    match current_type {
      Type::RESERVED_FUNCTION => self.parse_function_stat(&attributes).cast::<AstStat>(),
      Type::RESERVED_LOCAL => {
        let attr_loc = self.first_attr_location(&attributes, self.lexer.current().location);

        self.parse_local(
          attr_loc,
          self.lexer.current().location.begin,
          &attributes,
          false,
        )
      }
      Type::NAME => {
        let current = self.lexer.current();
        // NAME 臂下 data.name 是 active 臂，name() 安全读出，供三处名字比较复用
        let current_ident = current.name();

        if LuauExportValueSyntax.get() && current_ident == "export" {
          let keyword_loc = current.location;
          self.next_lexeme();

          let attr_loc = self.first_attr_location(&attributes, keyword_loc);

          self.parse_export_value(&attr_loc, keyword_loc.begin, &attributes)
        } else if current_ident == "const" {
          let keyword_loc = current.location;
          self.next_lexeme();

          let attr_loc = self.first_attr_location(&attributes, keyword_loc);

          self.parse_local(attr_loc, keyword_loc.begin, &attributes, true)
        } else if self.options.allow_declaration_syntax && current_ident == "declare" {
          let expr = self.parse_primary_expr(true);
          self.parse_declaration(&slot_ref(expr).base.location, &attributes)
        } else {
          self.parse_attribute_stat_fallthrough_to_error(&attributes)
        }
      }
      _ => self.parse_attribute_stat_fallthrough_to_error(&attributes),
    }
  }

  fn parse_attribute_stat_fallthrough_to_error(
    &mut self,
    _attributes: &AstArray<*mut AstAttr>,
  ) -> *mut AstStat {
    let current = *self.lexer.current();
    let loc = current.location;

    let empty_exprs = AstArray::EMPTY;
    let empty_lines = AstArray::EMPTY;

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
