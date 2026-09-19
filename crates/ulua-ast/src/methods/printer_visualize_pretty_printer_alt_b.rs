//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。CST 侧经 `lookup_cst_node` 返回 `Option<&T>`，
//! 判空与字段读取全部走安全代码。
//!
//! 节点子指针不再在调用点解引用：直接传裸指针给 `visualize_*`（`IntoNodePtr`
//! 归一）；`AstArray<*mut T>` 遍历统一走 `iter_nodes`。

use ulua_common::functions::format_g::format_g;

use crate::{
  functions::{escape_bytes::escape_bytes, to_string_ast_alt_b::to_str},
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
    position::EMPTY_POSITIONS,
    printer::{IntoNodePtr, Printer},
    writer::Writer,
  },
  rtti::{ast_node_is, ast_node_try_as},
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_ast_expr<E: IntoNodePtr<AstExpr>>(&mut self, expr: E) {
    let expr = expr.into_node_ptr();
    if expr.is_null() {
      return;
    }

    // SAFETY: expr 指向 arena 中存活的 AstExpr 派生节点
    let expr_ref = unsafe { &*expr };
    // cpp 侧 `AstNode* node = expr` 的只读形态：下转与 CST 查表都以共享借用为
    // 入参（打印器只写 Writer，从不写节点）。
    let node = &expr_ref.base;
    self.advance(expr_ref.base.location.begin);

    if let Some(a) = ast_node_try_as::<AstExprGroup>(node) {
      self.writer.symbol("(");
      self.visualize_ast_expr(a.expr);

      let cst_node = self.lookup_cst_node::<CstExprGroup>(node);
      if let Some(cst_node) = cst_node {
        self.maybe_advance_and_write(&cst_node.close_position, ")", false);
      } else {
        self.advance_before(a.base.base.location.end, 1);
        self.writer.symbol(")");
      }
    } else if ast_node_is::<AstExprConstantNil>(node) {
      self.writer.keyword("nil");
    } else if let Some(a) = ast_node_try_as::<AstExprConstantBool>(node) {
      self.writer.keyword(if a.value { "true" } else { "false" });
    } else if let Some(a) = ast_node_try_as::<AstExprConstantNumber>(node) {
      if let Some(cst_node) = self.lookup_cst_node::<CstExprConstantNumber>(node) {
        // 数字源文本直切片（非 fixup 产物）；literal 走字节通道。
        self.writer.literal(cst_node.value.as_bytes());
      } else if a.value.is_infinite() {
        self.writer.literal(if a.value.is_sign_positive() {
          "1e500".as_bytes()
        } else {
          "-1e500".as_bytes()
        });
      } else if a.value.is_nan() {
        self.writer.literal("0/0".as_bytes());
      } else if Self::printer_is_integerish(a.value) {
        // itoa 栈上缓冲直写，免 String 堆分配
        let mut buf = itoa::Buffer::new();
        self.writer.literal(buf.format(a.value as i32).as_bytes());
      } else {
        self.writer.literal(format_g(a.value, 17).as_bytes());
      }
    } else if let Some(a) = ast_node_try_as::<AstExprConstantInteger>(node) {
      if let Some(cst_node) = self.lookup_cst_node::<CstExprConstantInteger>(node) {
        // 数字源文本直切片（非 fixup 产物）；literal 走字节通道。
        self.writer.literal(cst_node.value.as_bytes());
      } else if a.value >= 0 {
        let mut buf = itoa::Buffer::new();
        self
          .writer
          .literal(format!("{}i", buf.format(a.value)).as_bytes());
      } else {
        self
          .writer
          .literal(format!("0x{:x}i", a.value as u64).as_bytes());
      }
    } else if let Some(a) = ast_node_try_as::<AstExprConstantString>(node) {
      if let Some(cst_node) = self.lookup_cst_node::<CstExprConstantString>(node) {
        // 源文本直切片（引号内原文，非 fixup 产物）。
        self.writer.source_string(
          cst_node.source_string.as_bytes(),
          cst_node.quote_style,
          cst_node.block_depth,
        );
      } else {
        // value 经 lexer_fixup_quoted_bytes，可含任意字节（`"\xff"` 非
        // UTF-8），必须走字节通道（cpp `std::string_view` 直写语义）。
        self.writer.string(a.value.as_bytes());
      }
    } else if let Some(a) = ast_node_try_as::<AstExprLocal>(node) {
      // SAFETY: local 指向 arena 存活的 AstLocal
      self.writer.identifier(unsafe { &*a.local }.name.as_bytes());
    } else if let Some(a) = ast_node_try_as::<AstExprGlobal>(node) {
      self.writer.identifier(a.name.as_bytes());
    } else if ast_node_is::<AstExprVarargs>(node) {
      self.writer.symbol("...");
    } else if let Some(a) = ast_node_try_as::<AstExprCall>(node) {
      self.visualize_ast_expr(a.func);

      let cst_node = self.lookup_cst_node::<CstExprCall>(node);

      if self.write_types
        && (a.type_arguments.size > 0 || cst_node.is_some_and(|cst| !cst.explicit_types.is_null()))
      {
        self.visualize_explicit_type_instantiation(
          a.type_arguments,
          cst_node.and_then(|cst| unsafe { cst.explicit_types.as_ref() }),
        );
      }

      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.open_parens), "(");

      let mut comma = CommaSeparatorInserter::new(
        cst_node.map_or(EMPTY_POSITIONS, |cst| cst.comma_positions.as_slice()),
      );
      for arg in a.args.iter() {
        comma.operator_call(self.writer);
        self.visualize_ast_expr(*arg);
      }

      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_parens), ")");
    } else if let Some(a) = ast_node_try_as::<AstExprIndexName>(node) {
      self.visualize_ast_expr(a.expr);
      self.advance(a.op_position);
      // 单字符编码进栈缓冲，免 String 堆分配（cpp `std::string(1, a->op)`）
      let mut opbuf = [0u8; 4];
      self
        .writer
        .symbol((a.op as u8 as char).encode_utf8(&mut opbuf));
      self.advance(a.index_location.begin);
      self.writer.write(a.index.as_bytes());
    } else if let Some(a) = ast_node_try_as::<AstExprIndexExpr>(node) {
      let cst_node = self.lookup_cst_node::<CstExprIndexExpr>(node);
      self.visualize_ast_expr(a.expr);

      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.open_bracket_position), "[");

      self.visualize_ast_expr(a.index);

      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_bracket_position), "]");
    } else if let Some(a) = ast_node_try_as::<AstExprFunction>(node) {
      for attr in a.attributes.iter_nodes() {
        self.visualize_attribute(attr);
      }
      self.writer.keyword("function");
      self.visualize_function_body(a);
    } else if let Some(a) = ast_node_try_as::<AstExprTable>(node) {
      self.writer.symbol("{");

      let cst_node = self.lookup_cst_node::<CstExprTable>(node);
      // CST 与 AST items 由解析器成对构造，长度一致（C++ 同处有断言）
      let cst_items = match cst_node {
        Some(cst) => {
          ulua_common::LUAU_ASSERT!(cst.items.len() == a.items.len());
          cst.items.as_slice()
        }
        None => &[],
      };

      for (index, item) in a.items.iter().enumerate() {
        // 无 CST 时逐项补逗号分隔符
        if cst_items.is_empty() && index > 0 {
          self.writer.symbol(",");
        }

        match cst_items.get(index) {
          Some(cst_item) => match item.kind {
            ItemKind::List => {}
            ItemKind::Record => {
              // SAFETY: item.key 指向 arena 存活的 AstExpr 派生节点；Record 项
              // key 由解析器保证为常量字符串（断言同一不变式），class_index
              // 匹配后经安全 try_as 下转。
              let key = unsafe { item.key.as_ref() }
                .and_then(|k| ast_node_try_as::<AstExprConstantString>(&k.base));
              ulua_common::LUAU_ASSERT!(key.is_some());
              if let Some(key) = key {
                self.advance(key.base.base.location.begin);
                self.writer.identifier(key.value.as_bytes());
                self.advance(cst_item.equals_position);
                self.writer.symbol("=");
              }
            }
            ItemKind::General => {
              ulua_common::LUAU_ASSERT!(cst_item.indexer_open_position.has_value());
              self.maybe_advance_and_write(&cst_item.indexer_open_position, "[", true);
              self.visualize_ast_expr(item.key);
              self.maybe_advance_and_write(&cst_item.indexer_close_position, "]", false);
              self.maybe_advance_and_write(&cst_item.equals_position, "=", false);
            }
          },
          None => match item.kind {
            ItemKind::List => {}
            ItemKind::Record => {
              // SAFETY: 同上（Record 项 key 为常量字符串）
              let key = unsafe { item.key.as_ref() }
                .and_then(|k| ast_node_try_as::<AstExprConstantString>(&k.base));
              ulua_common::LUAU_ASSERT!(key.is_some());
              if let Some(key) = key {
                self.advance(key.base.base.location.begin);
                self.writer.identifier(key.value.as_bytes());
                // SAFETY: value 指向 arena 存活的 AstExpr
                self
                  .writer
                  .maybe_space(&unsafe { &*item.value }.base.location.begin, 1);
                self.writer.symbol("=");
              }
            }
            ItemKind::General => {
              self.writer.symbol("[");
              self.visualize_ast_expr(item.key);
              self.writer.symbol("]");
              // SAFETY: value 指向 arena 存活的 AstExpr
              self
                .writer
                .maybe_space(&unsafe { &*item.value }.base.location.begin, 1);
              self.writer.symbol("=");
            }
          },
        }

        // SAFETY: value 指向 arena 存活的 AstExpr
        let value = unsafe { &*item.value };
        self.advance(value.base.location.begin);
        self.visualize_ast_expr(value);

        if let Some(cst_item) = cst_items.get(index) {
          let separator = cst_item.separator;
          if separator != CstExprTableSeparator::Missing {
            ulua_common::LUAU_ASSERT!(cst_item.separator_position.has_value());
            self.maybe_advance_and_write(
              &cst_item.separator_position,
              if separator == CstExprTableSeparator::Comma {
                ","
              } else {
                ";"
              },
              true,
            );
          }
        }
      }

      let mut end_pos = expr_ref.base.location.end;
      if end_pos.column > 0 {
        end_pos.column -= 1;
      }
      self.advance(end_pos);
      self.writer.symbol("}");
      self.advance(expr_ref.base.location.end);
    } else if let Some(a) = ast_node_try_as::<AstExprUnary>(node) {
      if let Some(cst_node) = self.lookup_cst_node::<CstExprOp>(node) {
        self.advance(cst_node.op_position);
      }

      match a.op {
        AstExprUnaryOp::Not => self.writer.keyword("not"),
        AstExprUnaryOp::Minus => self.writer.symbol("-"),
        AstExprUnaryOp::Len => self.writer.symbol("#"),
      }
      self.visualize_ast_expr(a.expr);
    } else if let Some(a) = ast_node_try_as::<AstExprBinary>(node) {
      self.visualize_ast_expr(a.left);

      if let Some(cst_node) = self.lookup_cst_node::<CstExprOp>(node) {
        self.advance(cst_node.op_position);
      } else {
        // 操作符宽度决定与右操作数的间距（C++ 同名逻辑）
        let gap = match a.op {
          AstExprBinaryOp::Add
          | AstExprBinaryOp::Sub
          | AstExprBinaryOp::Mul
          | AstExprBinaryOp::Div
          | AstExprBinaryOp::FloorDiv
          | AstExprBinaryOp::Mod
          | AstExprBinaryOp::Pow
          | AstExprBinaryOp::CompareLt
          | AstExprBinaryOp::CompareGt => 2,
          AstExprBinaryOp::Concat
          | AstExprBinaryOp::CompareNe
          | AstExprBinaryOp::CompareEq
          | AstExprBinaryOp::CompareLe
          | AstExprBinaryOp::CompareGe
          | AstExprBinaryOp::Or => 3,
          AstExprBinaryOp::And => 4,
          AstExprBinaryOp::OpCount => {
            ulua_common::LUAU_ASSERT!(false);
            0
          }
        };
        // SAFETY: right 指向 arena 存活的 AstExpr
        self
          .writer
          .maybe_space(&unsafe { &*a.right }.base.location.begin, gap);
      }

      self.writer.symbol(to_str(a.op));
      self.visualize_ast_expr(a.right);
    } else if let Some(a) = ast_node_try_as::<AstExprTypeAssertion>(node) {
      self.visualize_ast_expr(a.expr);

      if self.write_types {
        match self.lookup_cst_node::<CstExprTypeAssertion>(node) {
          Some(cst_node) => self.advance(cst_node.op_position),
          None => {
            // SAFETY: annotation 指向 arena 存活的 AstType
            self
              .writer
              .maybe_space(&unsafe { &*a.annotation }.base.location.begin, 2)
          }
        }
        self.writer.symbol("::");
        self.visualize_type_annotation(a.annotation);
      }
    } else if let Some(a) = ast_node_try_as::<AstExprIfElse>(node) {
      self.writer.keyword("if");
      self.visualize_else_if_expr(a);
    } else if let Some(a) = ast_node_try_as::<AstExprInterpString>(node) {
      let cst_node = self.lookup_cst_node::<CstExprInterpString>(node);

      self.writer.symbol("`");

      let expressions = a.expressions.as_slice();

      for (index, string) in AstArray::iter(&a.strings).enumerate() {
        if let Some(cst) = cst_node {
          if index > 0 {
            // string_positions 与 strings 等长（解析器成对构造）；越界仅解析
            // 器 bug，跳过 advance。
            if let Some(pos) = cst.string_positions.as_slice().get(index) {
              self.advance(*pos);
            }
            self.writer.symbol("}");
          }

          // source_strings 与 strings 等长（解析器成对构造）；越界仅解析器
          // bug，跳过该段（cpp 侧 data[index] 同样依赖该不变式）。
          if let Some(source_string) = cst.source_strings.as_slice().get(index) {
            // 源文本直切片（引号内原文，非 fixup 产物）。
            self.writer.write_multiline(source_string.as_bytes());
          }
        } else {
          // strings 经 fixup 可含任意字节；escape_bytes 字节版转义后直写。
          self.writer.write(&escape_bytes(string.as_bytes(), true));
        }

        if let Some(&expression) = expressions.get(index) {
          self.writer.symbol("{");
          self.visualize_ast_expr(expression);
          if cst_node.is_none() {
            self.writer.symbol("}");
          }
        }
      }

      self.writer.symbol("`");
    } else if let Some(a) = ast_node_try_as::<AstExprError>(node) {
      self.writer.symbol("(error-expr");

      for (i, &expression) in a.expressions.iter().enumerate() {
        self.writer.symbol(if i == 0 { ": " } else { ", " });
        self.visualize_ast_expr(expression);
      }

      self.writer.symbol(")");
    } else if let Some(a) = ast_node_try_as::<AstExprInstantiate>(node) {
      self.visualize_ast_expr(a.expr);

      if self.write_types {
        let cst_expr_node = self.lookup_cst_node::<CstExprExplicitTypeInstantiation>(node);
        self.visualize_explicit_type_instantiation(
          a.type_arguments,
          cst_expr_node.map(|cst| &cst.instantiation),
        );
      }
    } else {
      ulua_common::LUAU_ASSERT!(false);
    }
  }
}
