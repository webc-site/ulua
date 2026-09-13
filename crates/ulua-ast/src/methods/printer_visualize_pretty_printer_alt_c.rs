//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。

use core::ptr::null;

use ulua_common::{FFlag, LUAU_ASSERT, records::variant::Variant2};

use crate::{
  records::{
    ast_array::AstArray, ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_expr_binary::AstExprBinaryOp, ast_local::AstLocal, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass, ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue, ast_stat_declare_global::AstStatDeclareGlobal,
    ast_stat_error::AstStatError, ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    ast_stat_repeat::AstStatRepeat, ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias, ast_stat_type_function::AstStatTypeFunction,
    ast_stat_while::AstStatWhile, comma_separator_inserter::CommaSeparatorInserter,
    cst_generic_type::CstGenericType, cst_generic_type_pack::CstGenericTypePack,
    cst_stat_assign::CstStatAssign, cst_stat_compound_assign::CstStatCompoundAssign,
    cst_stat_do::CstStatDo, cst_stat_for::CstStatFor, cst_stat_for_in::CstStatForIn,
    cst_stat_function::CstStatFunction, cst_stat_local::CstStatLocal,
    cst_stat_local_function::CstStatLocalFunction, cst_stat_repeat::CstStatRepeat,
    cst_stat_return::CstStatReturn, cst_stat_type_alias::CstStatTypeAlias,
    cst_stat_type_function::CstStatTypeFunction, position::Position, printer::Printer,
  },
  rtti::{ast_node_is, ast_node_try_as_mut},
};

pub trait IntoAstStatPrinter {
  fn into_ast_stat_mut(self) -> *mut AstStat;
}

impl IntoAstStatPrinter for &mut AstStat {
  fn into_ast_stat_mut(self) -> *mut AstStat {
    self
  }
}

impl IntoAstStatPrinter for *mut AstStat {
  fn into_ast_stat_mut(self) -> *mut AstStat {
    self
  }
}

impl IntoAstStatPrinter for &*mut AstStat {
  fn into_ast_stat_mut(self) -> *mut AstStat {
    *self
  }
}

impl<'a> Printer<'a> {
  pub fn visualize_ast_stat<S: IntoAstStatPrinter>(&mut self, program: S) {
    let program = unsafe { &mut *program.into_ast_stat_mut() };
    self.advance(program.base.location.begin);

    if let Some(block) =
      unsafe { ast_node_try_as_mut::<AstStatBlock>(program as *mut AstStat as *mut AstNode) }
    {
      let cst_node = self.lookup_cst_node::<CstStatDo>(program as *mut AstStat as *mut AstNode);
      if !cst_node.is_null() {
        self.writer.keyword("do");
        self.advance(unsafe { (*cst_node).stats_start_position });
        for s in AstArray::iter(&block.body) {
          self.visualize_ast_stat(s);
        }
        self.maybe_advance_and_write(unsafe { &(*cst_node).end_position }, "end", false);
      } else {
        for s in AstArray::iter(&block.body) {
          self.visualize_ast_stat(s);
        }
        self.advance(block.base.base.location.end);
        self.write_end(&program.base.location);
      }
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatIf>(program as *mut AstStat as *mut AstNode) }
    {
      self.writer.keyword("if");
      self.visualize_else_if(a);
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatWhile>(program as *mut AstStat as *mut AstNode) }
    {
      self.writer.keyword("while");
      self.visualize_ast_expr(a.condition);
      self.advance(a.do_location.begin);
      self.writer.keyword("do");
      self.visualize_block_ast_stat_block(a.body);
      self.advance(unsafe { (*a.body).base.base.location.end });
      self.writer.keyword("end");
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatRepeat>(program as *mut AstStat as *mut AstNode) }
    {
      self.writer.keyword("repeat");
      self.visualize_block_ast_stat_block(a.body);
      let cst_node = self.lookup_cst_node::<CstStatRepeat>(program as *mut AstStat as *mut AstNode);
      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).until_position }, "until", false);
      } else {
        self.advance_before(unsafe { (*a.condition).base.location.begin }, 6);
        self.writer.keyword("until");
      }
      self.visualize_ast_expr(a.condition);
    } else if ast_node_is::<AstStatBreak>(program as *mut AstStat as *mut AstNode) {
      self.writer.keyword("break");
    } else if ast_node_is::<AstStatContinue>(program as *mut AstStat as *mut AstNode) {
      self.writer.keyword("continue");
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatReturn>(program as *mut AstStat as *mut AstNode) }
    {
      let cst_node = self.lookup_cst_node::<CstStatReturn>(program as *mut AstStat as *mut AstNode);
      self.writer.keyword("return");
      let mut comma = CommaSeparatorInserter::new(
        self.writer,
        if !cst_node.is_null() {
          unsafe { (*cst_node).comma_positions.data }
        } else {
          null()
        },
      );
      for expr in AstArray::iter(&a.list) {
        comma.operator_call(self.writer);
        self.visualize_ast_expr(expr);
      }
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatExpr>(program as *mut AstStat as *mut AstNode) }
    {
      self.visualize_ast_expr(a.expr);
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatLocal>(program as *mut AstStat as *mut AstNode) }
    {
      let cst_node = self.lookup_cst_node::<CstStatLocal>(program as *mut AstStat as *mut AstNode);
      if FFlag::LuauExportValueSyntax.get() && a.is_exported {
        self.writer.keyword("export");
        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).declaration_keyword_position });
        }
      }
      self
        .writer
        .keyword(if a.is_const { "const" } else { "local" });
      let mut var_comma = CommaSeparatorInserter::new(
        self.writer,
        if !cst_node.is_null() {
          unsafe { (*cst_node).vars_comma_positions.data }
        } else {
          null()
        },
      );
      self.visualize_local_vars(
        &a.vars,
        &mut var_comma,
        if !cst_node.is_null() {
          Some(unsafe { (*cst_node).vars_annotation_colon_positions.as_slice() })
        } else {
          None
        },
      );
      if let Some(loc) = a.equals_sign_location {
        self.advance(loc.begin);
        self.writer.symbol("=");
      }
      let mut value_comma = CommaSeparatorInserter::new(
        self.writer,
        if !cst_node.is_null() {
          unsafe { (*cst_node).values_comma_positions.data }
        } else {
          null()
        },
      );
      for value in AstArray::iter(&a.values) {
        value_comma.operator_call(self.writer);
        self.visualize_ast_expr(value);
      }
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatFor>(program as *mut AstStat as *mut AstNode) }
    {
      let cst_node = self.lookup_cst_node::<CstStatFor>(program as *mut AstStat as *mut AstNode);
      self.writer.keyword("for");
      self.visualize_ast_local_position(
        unsafe { &*a.var },
        if !cst_node.is_null() {
          unsafe { (*cst_node).annotation_colon_position }
        } else {
          Position::missing()
        },
      );
      if !cst_node.is_null() {
        self.advance(unsafe { (*cst_node).equals_position });
      }
      self.writer.symbol("=");
      self.visualize_ast_expr(a.from);
      if !cst_node.is_null() {
        self.maybe_advance_and_write(&unsafe { (*cst_node).end_comma_position }, ",", false);
      } else {
        self.writer.symbol(",");
      }
      self.visualize_ast_expr(a.to);
      if !a.step.is_null() {
        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).step_comma_position });
        }
        self.writer.symbol(",");
        self.visualize_ast_expr(a.step);
      }
      self.advance(a.do_location.begin);
      self.writer.keyword("do");
      self.visualize_block_ast_stat_block(a.body);
      self.advance(unsafe { (*a.body).base.base.location.end });
      self.writer.keyword("end");
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatForIn>(program as *mut AstStat as *mut AstNode) }
    {
      let cst_node = self.lookup_cst_node::<CstStatForIn>(program as *mut AstStat as *mut AstNode);
      self.writer.keyword("for");
      let mut var_comma = CommaSeparatorInserter::new(
        self.writer,
        if !cst_node.is_null() {
          unsafe { (*cst_node).vars_comma_positions.data }
        } else {
          null()
        },
      );
      self.visualize_local_vars(
        &a.vars,
        &mut var_comma,
        if !cst_node.is_null() {
          Some(unsafe { (*cst_node).vars_annotation_colon_positions.as_slice() })
        } else {
          None
        },
      );
      self.advance(a.in_location.begin);
      self.writer.keyword("in");
      let mut val_comma = CommaSeparatorInserter::new(
        self.writer,
        if !cst_node.is_null() {
          unsafe { (*cst_node).values_comma_positions.data }
        } else {
          null()
        },
      );
      for val in AstArray::iter(&a.values) {
        val_comma.operator_call(self.writer);
        self.visualize_ast_expr(val);
      }
      self.advance(a.do_location.begin);
      self.writer.keyword("do");
      self.visualize_block_ast_stat_block(a.body);
      self.advance(unsafe { (*a.body).base.base.location.end });
      self.writer.keyword("end");
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatAssign>(program as *mut AstStat as *mut AstNode) }
    {
      let cst_node = self.lookup_cst_node::<CstStatAssign>(program as *mut AstStat as *mut AstNode);
      let mut var_comma = CommaSeparatorInserter::new(
        self.writer,
        if !cst_node.is_null() {
          unsafe { (*cst_node).vars_comma_positions.data }
        } else {
          null()
        },
      );
      for var in AstArray::iter(&a.vars) {
        var_comma.operator_call(self.writer);
        self.visualize_ast_expr(var);
      }
      if !cst_node.is_null() {
        self.maybe_advance_and_write(&unsafe { (*cst_node).equals_position }, "=", false);
      } else {
        self.writer.space();
        self.writer.symbol("=");
      }
      let mut value_comma = CommaSeparatorInserter::new(
        self.writer,
        if !cst_node.is_null() {
          unsafe { (*cst_node).values_comma_positions.data }
        } else {
          null()
        },
      );
      for value in AstArray::iter(&a.values) {
        value_comma.operator_call(self.writer);
        self.visualize_ast_expr(value);
      }
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstStatCompoundAssign>(program as *mut AstStat as *mut AstNode)
    } {
      let cst_node =
        self.lookup_cst_node::<CstStatCompoundAssign>(program as *mut AstStat as *mut AstNode);
      self.visualize_ast_expr(a.var);
      if !cst_node.is_null() {
        self.advance(unsafe { (*cst_node).op_position });
      }
      let symbol = match a.op {
        AstExprBinaryOp::Add => "+=",
        AstExprBinaryOp::Sub => "-=",
        AstExprBinaryOp::Mul => "*=",
        AstExprBinaryOp::Div => "/=",
        AstExprBinaryOp::FloorDiv => "//=",
        AstExprBinaryOp::Mod => "%=",
        AstExprBinaryOp::Pow => "^=",
        AstExprBinaryOp::Concat => "..=",
        _ => {
          LUAU_ASSERT!(false);
          ""
        }
      };
      if !symbol.is_empty() {
        if cst_node.is_null() {
          self.writer.maybe_space(
            &unsafe { (*a.value).base.location.begin },
            symbol.len() as i32,
          );
        }
        self.writer.symbol(symbol);
      }
      self.visualize_ast_expr(a.value);
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatFunction>(program as *mut AstStat as *mut AstNode) }
    {
      for attr in unsafe { AstArray::iter(&(*a.func).attributes) } {
        self.visualize_attribute(unsafe { &mut **attr });
      }
      let cst_node =
        self.lookup_cst_node::<CstStatFunction>(program as *mut AstStat as *mut AstNode);
      if !cst_node.is_null() {
        self.advance(unsafe { (*cst_node).function_keyword_position });
      }
      self.writer.keyword("function");
      self.visualize_ast_expr(a.name);
      self.visualize_function_body(a.func);
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstStatLocalFunction>(program as *mut AstStat as *mut AstNode)
    } {
      for attr in unsafe { AstArray::iter(&(*a.func).attributes) } {
        self.visualize_attribute(unsafe { &mut **attr });
      }
      let cst_node =
        self.lookup_cst_node::<CstStatLocalFunction>(program as *mut AstStat as *mut AstNode);
      if !cst_node.is_null() {
        self.advance(unsafe { (*cst_node).local_keyword_position });
      }
      if FFlag::LuauExportValueSyntax.get() && unsafe { (*a.name).is_exported } {
        self.writer.keyword("export");
      } else if unsafe { (*a.name).is_const } {
        self.writer.keyword("const");
      } else {
        self.writer.keyword("local");
      }
      if !cst_node.is_null() {
        self.advance(unsafe { (*cst_node).function_keyword_position });
      } else {
        self.writer.space();
      }
      self.writer.keyword("function");
      self.advance(unsafe { (*a.name).location.begin });
      self
        .writer
        .identifier(unsafe { (*a.name).name.as_str_or_empty() });
      self.visualize_function_body(a.func);
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatTypeAlias>(program as *mut AstStat as *mut AstNode) }
    {
      if self.write_types {
        let cst_node =
          self.lookup_cst_node::<CstStatTypeAlias>(program as *mut AstStat as *mut AstNode);
        if a.exported {
          self.writer.keyword("export");
        }
        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).type_keyword_position });
        }
        self.writer.keyword("type");
        self.advance(a.name_location.begin);
        self.writer.identifier(a.name.as_str_or_empty());
        if a.generics.size > 0 || a.generic_packs.size > 0 {
          if !cst_node.is_null() {
            self.advance(unsafe { (*cst_node).generics_open_position });
          }
          self.writer.symbol("<");
          let mut comma = CommaSeparatorInserter::new(
            self.writer,
            if !cst_node.is_null() {
              unsafe { (*cst_node).generics_comma_positions.data }
            } else {
              null()
            },
          );
          for o in AstArray::iter(&a.generics) {
            let o = *o;
            comma.operator_call(self.writer);
            self.writer.advance(unsafe { &(*o).base.location.begin });
            self
              .writer
              .identifier(unsafe { (*o).name.as_str_or_empty() });
            if !unsafe { (*o).default_value.is_null() } {
              let generic_type_cst_node = self.lookup_cst_node::<CstGenericType>(o as *mut AstNode);
              if !generic_type_cst_node.is_null() {
                self.advance(unsafe { (*generic_type_cst_node).default_equals_position });
              } else {
                self
                  .writer
                  .maybe_space(unsafe { &(*(*o).default_value).base.location.begin }, 2);
              }
              self.writer.symbol("=");
              self.visualize_type_annotation(unsafe { (*o).default_value });
            }
          }
          for o in AstArray::iter(&a.generic_packs) {
            let o = *o;
            comma.operator_call(self.writer);
            let generic_type_pack_cst_node =
              self.lookup_cst_node::<CstGenericTypePack>(o as *mut AstNode);
            self.writer.advance(unsafe { &(*o).base.location.begin });
            self
              .writer
              .identifier(unsafe { (*o).name.as_str_or_empty() });
            if !generic_type_pack_cst_node.is_null() {
              self.maybe_advance_and_write(
                &unsafe { (*generic_type_pack_cst_node).ellipsis_position },
                "...",
                false,
              );
            } else {
              self.writer.symbol("...");
            }
            if !unsafe { (*o).default_value.is_null() } {
              if !cst_node.is_null() {
                self.advance(unsafe { (*generic_type_pack_cst_node).default_equals_position });
              } else {
                self
                  .writer
                  .maybe_space(unsafe { &(*(*o).default_value).base.location.begin }, 2);
              }
              self.writer.symbol("=");
              self.visualize_type_pack_annotation(
                unsafe { &mut *(*o).default_value },
                false,
                false,
                false,
              );
            }
          }
          if !cst_node.is_null() {
            self.maybe_advance_and_write(
              &unsafe { (*cst_node).generics_close_position },
              ">",
              false,
            );
          } else {
            self.writer.symbol(">");
          }
        }
        if !cst_node.is_null() {
          self.maybe_advance_and_write(&unsafe { (*cst_node).equals_position }, "=", false);
        } else {
          self
            .writer
            .maybe_space(unsafe { &(*a.type_ptr).base.location.begin }, 2);
          self.writer.symbol("=");
        }
        self.visualize_type_annotation(a.type_ptr);
      }
    } else if let Some(t) =
      unsafe { ast_node_try_as_mut::<AstStatTypeFunction>(program as *mut AstStat as *mut AstNode) }
    {
      if self.write_types {
        let cst_node =
          self.lookup_cst_node::<CstStatTypeFunction>(program as *mut AstStat as *mut AstNode);
        if t.exported {
          self.writer.keyword("export");
        }
        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).type_keyword_position });
        } else {
          self.writer.space();
        }
        self.writer.keyword("type");
        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).function_keyword_position });
        } else {
          self.writer.space();
        }
        self.writer.keyword("function");
        self.advance(t.name_location.begin);
        self.writer.identifier(t.name.as_str_or_empty());
        self.visualize_function_body(t.body);
      }
    } else if let Some(a) =
      unsafe { ast_node_try_as_mut::<AstStatError>(program as *mut AstStat as *mut AstNode) }
    {
      self.writer.symbol("(error-stat");
      let no_statements = a.statements.is_empty();
      for (i, &expression) in a.expressions.iter().enumerate() {
        self
          .writer
          .symbol(if i == 0 && no_statements { ": " } else { ", " });
        self.visualize_ast_expr(expression);
      }
      let no_expressions = a.expressions.is_empty();
      for (i, &statement) in a.statements.iter().enumerate() {
        self
          .writer
          .symbol(if i == 0 && no_expressions { ": " } else { ", " });
        self.visualize_ast_stat(statement);
      }
      self.writer.symbol(")");
    } else if let Some(a) = unsafe {
      ast_node_try_as_mut::<AstStatDeclareGlobal>(program as *mut AstStat as *mut AstNode)
    } {
      self.writer.keyword("declare");
      self.advance(a.name_location.begin);
      self.writer.identifier(a.name.as_str_or_empty());
      self.writer.symbol(":");
      self.visualize_type_annotation(a.type_);
    } else if let Some(c) =
      unsafe { ast_node_try_as_mut::<AstStatClass>(program as *mut AstStat as *mut AstNode) }
    {
      if FFlag::DebugLuauUserDefinedClasses.get() {
        self.writer.keyword("class");
        self.advance(unsafe { (*c.name).location.begin });
        self
          .writer
          .identifier(unsafe { (*c.name).name.as_str_or_empty() });
        for member in AstArray::iter(&c.members) {
          match member {
            Variant2::V0(prop) => {
              let prop: &AstClassProperty = prop;
              self.advance(prop.qualifier_location.begin);
              self.writer.keyword("public");
              self.advance(prop.name_location.begin);
              self.writer.identifier(prop.name.as_str_or_empty());
              if self.write_types && !prop.ty.is_null() {
                LUAU_ASSERT!(prop.type_colon_location.is_some());
                self.advance(prop.type_colon_location.unwrap().begin);
                self.writer.symbol(":");
                self.visualize_type_annotation(prop.ty);
              }
            }
            Variant2::V1(method) => {
              let method: &AstClassMethod = method;
              if let Some(qualifier_location) = method.qualifier_location {
                self.advance(qualifier_location.begin);
                self.writer.keyword("public");
              }
              self.advance(method.keyword_location.begin);
              self.writer.keyword("function");
              self.advance(method.name_location.begin);
              self
                .writer
                .identifier(method.function_name.as_str_or_empty());
              self.visualize_function_body(method.function);
            }
          }
        }
        self.writer.newline();
        self.writer.keyword("end");
        self.writer.newline();
      }
    } else {
      LUAU_ASSERT!(false);
    }

    if program.has_semicolon {
      self.advance_before(program.base.location.end, 1);
      self.writer.symbol(";");
    }
  }

  /// 遍历局部变量列表，逐个写逗号分隔符与本地位置；`colon_positions` 为
  /// CST 提供的冒号位置（缺失时用 `Position::missing()`）。
  fn visualize_local_vars(
    &mut self,
    vars: &AstArray<*mut AstLocal>,
    var_comma: &mut CommaSeparatorInserter,
    colon_positions: Option<&[Position]>,
  ) {
    for (i, var) in AstArray::iter(vars).enumerate() {
      var_comma.operator_call(self.writer);
      let colon_position = if let Some(colons) = colon_positions {
        LUAU_ASSERT!(colons.len() > i);
        // SAFETY: 解析器保证 vars_annotation_colon_positions 与 vars 等长
        //（LUAU_ASSERT 同一不变式），C++ 侧 `data[i]` 同样无越界检查。
        unsafe { *colons.get_unchecked(i) }
      } else {
        Position::missing()
      };
      // SAFETY: vars 元素指向 arena 中存活的 AstLocal。
      self.visualize_ast_local_position(unsafe { &**var }, colon_position);
    }
  }
}
