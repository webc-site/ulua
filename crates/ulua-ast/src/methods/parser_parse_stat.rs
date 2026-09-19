use ulua_common::fflag::{DebugLuauUserDefinedClasses, LuauExportValueSyntax};

use crate::{
  functions::get_identifier::get_identifier,
  methods::lexeme_name_is::lexeme_name_is,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_stat::AstStat,
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
      // C++ `parseFunctionStat({})`：空属性数组
      Type::RESERVED_FUNCTION => {
        return self.parse_function_stat(&AstArray::EMPTY) as *mut AstStat;
      }
      Type::RESERVED_LOCAL => {
        let start = self.lexer.current().location;
        return self.parse_local(start, start.begin, &AstArray::EMPTY, false);
      }
      Type::RESERVED_RETURN => return self.parse_return(),
      Type::RESERVED_BREAK => return self.parser_parse_break(),
      Type::ATTRIBUTE | Type::ATTRIBUTE_OPEN => return self.parse_attribute_stat(),
      _ => {}
    }

    let start = self.lexer.current().location;
    let expr = self.parse_primary_expr(true);
    // expr 由 parse_primary_expr 保证非空（无 null 返回路径），且 AST 节点
    // 分配后不可变：location 提前读为局部值，后续复用免重复解引用。
    let expr_location = unsafe { (*expr).base.location };
    let expr_begin = expr_location.begin;

    if unsafe { ast_node_is::<AstExprCall>(&*expr) } {
      return unsafe {
        (*self.allocator).alloc(AstStatExpr::new(expr_location, expr)) as *mut AstStat
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

    if ident == "type" {
      return self.parse_type_alias(&expr_location, false, expr_begin);
    }

    if DebugLuauUserDefinedClasses.get() && ident == "class" {
      return self.parse_class_stat(&start, false, false);
    } else if DebugLuauUserDefinedClasses.get() && ident == "open" {
      // `type != Name || name != "class"` 等价于 `!lexeme_name_is(.., "class")`
      if !lexeme_name_is(self.lexer.current(), "class") {
        return self.report_incomplete_class_error(expr);
      }

      self.next_lexeme(); // consume `class`
      return self.parse_class_stat(&start, false, true);
    }

    if ident == "export" {
      if LuauExportValueSyntax.get() {
        // 快照与后续判型之间无 lexer 推进，保留 &Lexeme 局部拷贝以免未来插入推进时静默变义
        let current = self.lexer.current();
        let is_local = current.r#type == Type::RESERVED_LOCAL;
        let is_function = current.r#type == Type::RESERVED_FUNCTION;
        let is_const = lexeme_name_is(current, "const");
        let is_class = DebugLuauUserDefinedClasses.get()
          && (lexeme_name_is(current, "class") || lexeme_name_is(current, "open"));

        if is_local || is_function || is_const || is_class {
          return self.parse_export_value(&expr_location, expr_begin, &AstArray::EMPTY);
        } else if lexeme_name_is(current, "type") {
          let type_keyword_position = current.location.begin;
          self.next_lexeme();
          return self.parse_type_alias(&expr_location, true, type_keyword_position);
        }
      } else if DebugLuauUserDefinedClasses.get() && lexeme_name_is(self.lexer.current(), "class") {
        self.next_lexeme();
        return self.parse_class_stat(&start, true, false);
      } else if DebugLuauUserDefinedClasses.get() && lexeme_name_is(self.lexer.current(), "open") {
        self.next_lexeme(); // consume `open`
        if !lexeme_name_is(self.lexer.current(), "class") {
          return self.report_incomplete_class_error(expr);
        }

        self.next_lexeme(); // consume `class`
        return self.parse_class_stat(&start, true, true);
      } else if lexeme_name_is(self.lexer.current(), "type") {
        let type_keyword_position = self.lexer.current().location.begin;
        self.next_lexeme();
        return self.parse_type_alias(&expr_location, true, type_keyword_position);
      }
    }

    if ident == "continue" {
      return self.parser_parse_continue(&expr_location);
    }

    if ident == "const" {
      return self.parse_local(expr_location, expr_begin, &AstArray::EMPTY, true);
    }

    if self.options.allow_declaration_syntax && ident == "declare" {
      return self.parse_declaration(&expr_location, &AstArray::EMPTY);
    }

    if start.begin == self.lexer.current().location.begin {
      self.next_lexeme();
    }

    let exprs = self.copy_initializer_list_t(&[expr]);

    self.report_stat_error(
      expr_location,
      exprs,
      AstArray::EMPTY,
      format_args!("Incomplete statement: expected assignment or a function call"),
    ) as *mut AstStat
  }

  /// `open` 后缺 class 定义的报错（原文件两处重复块）。C++ 对应
  /// `reportStatError(expr->location, {expr}, {}, "Incomplete statement: ...")`。
  fn report_incomplete_class_error(&mut self, expr: *mut AstExpr) -> *mut AstStat {
    // SAFETY: expr 非空且指向 arena 存活节点（parse_primary_expr 不变式）
    let expr_location = unsafe { (*expr).base.location };
    let exprs = self.copy_initializer_list_t(&[expr]);
    let stats = self.copy_initializer_list_t(&[]);
    self.report_stat_error(
      expr_location,
      exprs,
      stats,
      format_args!("Incomplete statement: expected a class definition after 'open'"),
    ) as *mut AstStat
  }
}
