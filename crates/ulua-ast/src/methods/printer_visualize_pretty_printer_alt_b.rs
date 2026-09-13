//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。

use core::{
  ptr::{null, null_mut},
  slice::from_raw_parts,
  str::from_utf8_unchecked,
};

use ulua_common::functions::{escape::escape, format_g::format_g};

use crate::{
  functions::to_string_ast_alt_b::to_string,
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_expr_varargs::AstExprVarargs,
    ast_node::AstNode,
    comma_separator_inserter::CommaSeparatorInserter,
    cst_expr_call::CstExprCall,
    cst_expr_constant_integer::CstExprConstantInteger,
    cst_expr_constant_number::CstExprConstantNumber,
    cst_expr_constant_string::CstExprConstantString,
    cst_expr_explicit_type_instantiation::CstExprExplicitTypeInstantiation,
    cst_expr_group::CstExprGroup,
    cst_expr_index_expr::CstExprIndexExpr,
    cst_expr_interp_string::CstExprInterpString,
    cst_expr_op::CstExprOp,
    cst_expr_table::{CstExprTable, CstExprTableSeparator},
    cst_expr_type_assertion::CstExprTypeAssertion,
    printer::Printer,
  },
  rtti::{ast_node_as, ast_node_is, ast_node_try_as_mut},
};

pub trait IntoAstExprMut {
  fn into_ast_expr_mut(self) -> *mut AstExpr;
}

impl IntoAstExprMut for *mut AstExpr {
  fn into_ast_expr_mut(self) -> *mut AstExpr {
    self
  }
}

impl IntoAstExprMut for &*mut AstExpr {
  fn into_ast_expr_mut(self) -> *mut AstExpr {
    *self
  }
}

impl IntoAstExprMut for &mut AstExpr {
  fn into_ast_expr_mut(self) -> *mut AstExpr {
    self
  }
}

impl<'a> Printer<'a> {
  pub fn visualize_ast_expr<E: IntoAstExprMut>(&mut self, expr: E) {
    let expr = expr.into_ast_expr_mut();
    if expr.is_null() {
      return;
    }

    let node = expr as *mut AstNode;
    let expr_ref = unsafe { &mut *expr };
    self.advance(expr_ref.base.location.begin);

    if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprGroup>(node) } {
      self.writer.symbol("(");
      self.visualize_ast_expr(a.expr);

      let cst_node = self.lookup_cst_node::<CstExprGroup>(node);
      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).close_position }, ")", false);
      } else {
        self.advance_before(a.base.base.location.end, 1);
        self.writer.symbol(")");
      }
    } else if ast_node_is::<AstExprConstantNil>(node) {
      self.writer.keyword("nil");
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprConstantBool>(node) } {
      self.writer.keyword(if a.value { "true" } else { "false" });
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprConstantNumber>(node) } {
      let cst_node = self.lookup_cst_node::<CstExprConstantNumber>(node);
      if !cst_node.is_null() {
        let value = unsafe {
          from_utf8_unchecked(from_raw_parts(
            (*cst_node).value.data as *const u8,
            (*cst_node).value.size,
          ))
        };
        self.writer.literal(value);
      } else if a.value.is_infinite() {
        self.writer.literal(if a.value.is_sign_positive() {
          "1e500"
        } else {
          "-1e500"
        });
      } else if a.value.is_nan() {
        self.writer.literal("0/0");
      } else if Printer::printer_is_integerish(a.value) {
        self.writer.literal(&(a.value as i32).to_string());
      } else {
        self.writer.literal(&format_g(a.value, 17));
      }
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprConstantInteger>(node) } {
      let cst_node = self.lookup_cst_node::<CstExprConstantInteger>(node);
      if !cst_node.is_null() {
        let value = unsafe {
          from_utf8_unchecked(from_raw_parts(
            (*cst_node).value.data as *const u8,
            (*cst_node).value.size,
          ))
        };
        self.writer.literal(value);
      } else if a.value >= 0 {
        self.writer.literal(&format!("{}i", a.value));
      } else {
        self.writer.literal(&format!("0x{:x}i", a.value as u64));
      }
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprConstantString>(node) } {
      let cst_node = self.lookup_cst_node::<CstExprConstantString>(node);
      if !cst_node.is_null() {
        let source = unsafe {
          from_utf8_unchecked(from_raw_parts(
            (*cst_node).source_string.data as *const u8,
            (*cst_node).source_string.size,
          ))
        };
        self
          .writer
          .source_string(source, unsafe { (*cst_node).quote_style }, unsafe {
            (*cst_node).block_depth
          });
      } else {
        let value =
          unsafe { from_utf8_unchecked(from_raw_parts(a.value.data as *const u8, a.value.size)) };
        self.writer.string(value);
      }
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprLocal>(node) } {
      self
        .writer
        .identifier(unsafe { (*a.local).name.as_str_or_empty() });
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprGlobal>(node) } {
      self.writer.identifier(a.name.as_str_or_empty());
    } else if ast_node_is::<AstExprVarargs>(node) {
      self.writer.symbol("...");
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprCall>(node) } {
      self.visualize_ast_expr(a.func);

      let cst_node = self.lookup_cst_node::<CstExprCall>(node);

      if self.write_types
        && (a.type_arguments.size > 0
          || (!cst_node.is_null() && unsafe { !(*cst_node).explicit_types.is_null() }))
      {
        self.visualize_explicit_type_instantiation(
          a.type_arguments,
          if !cst_node.is_null() {
            unsafe { (*cst_node).explicit_types }
          } else {
            null_mut()
          },
        );
      }

      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).open_parens }, "(", false);
      } else {
        self.writer.symbol("(");
      }

      let mut comma = CommaSeparatorInserter::new(
        self.writer,
        if !cst_node.is_null() {
          unsafe { (*cst_node).comma_positions.data }
        } else {
          null()
        },
      );
      for arg in a.args.iter() {
        comma.operator_call(self.writer);
        self.visualize_ast_expr(*arg);
      }

      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).close_parens }, ")", false);
      } else {
        self.writer.symbol(")");
      }
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprIndexName>(node) } {
      self.visualize_ast_expr(a.expr);
      self.advance(a.op_position);
      let op = (a.op as u8 as char).to_string();
      self.writer.symbol(&op);
      self.advance(a.index_location.begin);
      self.writer.write(a.index.as_str_or_empty());
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprIndexExpr>(node) } {
      let cst_node = self.lookup_cst_node::<CstExprIndexExpr>(node);
      self.visualize_ast_expr(a.expr);

      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).open_bracket_position }, "[", false);
      } else {
        self.writer.symbol("[");
      }

      self.visualize_ast_expr(a.index);

      if !cst_node.is_null() {
        self.maybe_advance_and_write(unsafe { &(*cst_node).close_bracket_position }, "]", false);
      } else {
        self.writer.symbol("]");
      }
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprFunction>(node) } {
      for attr in a.attributes.iter() {
        self.visualize_attribute(unsafe { &mut **attr });
      }
      self.writer.keyword("function");
      self.visualize_function_body(a);
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprTable>(node) } {
      self.writer.symbol("{");

      let cst_node = self.lookup_cst_node::<CstExprTable>(node);
      let mut cst_item = if !cst_node.is_null() {
        ulua_common::LUAU_ASSERT!(unsafe { (*cst_node).items.size == a.items.size });
        unsafe { (*cst_node).items.data }
      } else {
        null()
      };
      let mut first = true;

      for item in a.items.iter() {
        if cst_item.is_null() {
          if first {
            first = false;
          } else {
            self.writer.symbol(",");
          }
        }

        match item.kind {
          ItemKind::List => {}
          ItemKind::Record => {
            let key = unsafe { ast_node_as::<AstExprConstantString>(item.key as *mut AstNode) };
            ulua_common::LUAU_ASSERT!(!key.is_null());
            let key = unsafe { &mut *key };
            self.advance(key.base.base.location.begin);
            let value = unsafe {
              from_utf8_unchecked(from_raw_parts(key.value.data as *const u8, key.value.size))
            };
            self.writer.identifier(value);

            if !cst_item.is_null() {
              self.advance(unsafe { (*cst_item).equals_position });
            } else {
              unsafe {
                self
                  .writer
                  .maybe_space(&(*item.value).base.location.begin, 1);
              }
            }
            self.writer.symbol("=");
          }
          ItemKind::General => {
            if !cst_item.is_null() {
              ulua_common::LUAU_ASSERT!(unsafe { (*cst_item).indexer_open_position.has_value() });
              self.maybe_advance_and_write(
                unsafe { &(*cst_item).indexer_open_position },
                "[",
                true,
              );
              self.visualize_ast_expr(item.key);
              self.maybe_advance_and_write(
                unsafe { &(*cst_item).indexer_close_position },
                "]",
                false,
              );
              self.maybe_advance_and_write(unsafe { &(*cst_item).equals_position }, "=", false);
            } else {
              self.writer.symbol("[");
              self.visualize_ast_expr(item.key);
              self.writer.symbol("]");
              unsafe {
                self
                  .writer
                  .maybe_space(&(*item.value).base.location.begin, 1);
              }
              self.writer.symbol("=");
            }
          }
        }

        unsafe {
          self.advance((*item.value).base.location.begin);
        }
        self.visualize_ast_expr(item.value);

        if !cst_item.is_null() {
          let separator = unsafe { (*cst_item).separator };
          if separator != CstExprTableSeparator::Missing {
            ulua_common::LUAU_ASSERT!(unsafe { (*cst_item).separator_position.has_value() });
            self.maybe_advance_and_write(
              unsafe { &(*cst_item).separator_position },
              if separator == CstExprTableSeparator::Comma {
                ","
              } else {
                ";"
              },
              true,
            );
          }
          cst_item = unsafe { cst_item.add(1) };
        }
      }

      let mut end_pos = expr_ref.base.location.end;
      if end_pos.column > 0 {
        end_pos.column -= 1;
      }
      self.advance(end_pos);
      self.writer.symbol("}");
      self.advance(expr_ref.base.location.end);
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprUnary>(node) } {
      let cst_node = self.lookup_cst_node::<CstExprOp>(node);
      if !cst_node.is_null() {
        self.advance(unsafe { (*cst_node).op_position });
      }

      match a.op {
        AstExprUnaryOp::Not => self.writer.keyword("not"),
        AstExprUnaryOp::Minus => self.writer.symbol("-"),
        AstExprUnaryOp::Len => self.writer.symbol("#"),
      }
      self.visualize_ast_expr(a.expr);
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprBinary>(node) } {
      self.visualize_ast_expr(a.left);

      let cst_node = self.lookup_cst_node::<CstExprOp>(node);
      if !cst_node.is_null() {
        self.advance(unsafe { (*cst_node).op_position });
      } else {
        match a.op {
          AstExprBinaryOp::Add
          | AstExprBinaryOp::Sub
          | AstExprBinaryOp::Mul
          | AstExprBinaryOp::Div
          | AstExprBinaryOp::FloorDiv
          | AstExprBinaryOp::Mod
          | AstExprBinaryOp::Pow
          | AstExprBinaryOp::CompareLt
          | AstExprBinaryOp::CompareGt => unsafe {
            self.writer.maybe_space(&(*a.right).base.location.begin, 2);
          },
          AstExprBinaryOp::Concat
          | AstExprBinaryOp::CompareNe
          | AstExprBinaryOp::CompareEq
          | AstExprBinaryOp::CompareLe
          | AstExprBinaryOp::CompareGe
          | AstExprBinaryOp::Or => unsafe {
            self.writer.maybe_space(&(*a.right).base.location.begin, 3);
          },
          AstExprBinaryOp::And => unsafe {
            self.writer.maybe_space(&(*a.right).base.location.begin, 4);
          },
          AstExprBinaryOp::OpCount => ulua_common::LUAU_ASSERT!(false),
        }
      }

      let op = to_string(a.op);
      self.writer.symbol(&op);
      self.visualize_ast_expr(a.right);
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprTypeAssertion>(node) } {
      self.visualize_ast_expr(a.expr);

      if self.write_types {
        let cst_node = self.lookup_cst_node::<CstExprTypeAssertion>(node);
        if !cst_node.is_null() {
          self.advance(unsafe { (*cst_node).op_position });
        } else {
          unsafe {
            self
              .writer
              .maybe_space(&(*a.annotation).base.location.begin, 2);
          }
        }
        self.writer.symbol("::");
        unsafe {
          self.visualize_type_annotation(&mut *a.annotation);
        }
      }
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprIfElse>(node) } {
      self.writer.keyword("if");
      self.visualize_else_if_expr(a);
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprInterpString>(node) } {
      let cst_node = self.lookup_cst_node::<CstExprInterpString>(node);

      self.writer.symbol("`");

      for (index, string) in AstArray::iter(&a.strings).enumerate() {
        if !cst_node.is_null() {
          if index > 0 {
            self.advance(unsafe { *(*cst_node).string_positions.data.add(index) });
            self.writer.symbol("}");
          }

          let source_string = unsafe { *(*cst_node).source_strings.data.add(index) };
          let source = unsafe {
            from_utf8_unchecked(from_raw_parts(
              source_string.data as *const u8,
              source_string.size,
            ))
          };
          self.writer.write_multiline(source);
        } else {
          let value =
            unsafe { from_utf8_unchecked(from_raw_parts(string.data as *const u8, string.size)) };
          self.writer.write(&escape(value, true));
        }

        if index < a.expressions.size {
          self.writer.symbol("{");
          self.visualize_ast_expr(unsafe { *a.expressions.data.add(index) });
          if cst_node.is_null() {
            self.writer.symbol("}");
          }
        }
      }

      self.writer.symbol("`");
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprError>(node) } {
      self.writer.symbol("(error-expr");

      for (i, &expression) in a.expressions.iter().enumerate() {
        self.writer.symbol(if i == 0 { ": " } else { ", " });
        self.visualize_ast_expr(expression);
      }

      self.writer.symbol(")");
    } else if let Some(a) = unsafe { ast_node_try_as_mut::<AstExprInstantiate>(node) } {
      self.visualize_ast_expr(a.expr);

      if self.write_types {
        let cst_expr_node = self.lookup_cst_node::<CstExprExplicitTypeInstantiation>(node);
        self.visualize_explicit_type_instantiation(
          a.type_arguments,
          if !cst_expr_node.is_null() {
            unsafe { &(*cst_expr_node).instantiation }
          } else {
            null()
          },
        );
      }
    } else {
      ulua_common::LUAU_ASSERT!(false);
    }
  }
}
