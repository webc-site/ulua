//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。CST 侧经 `lookup_cst_node` 返回 `Option<&T>`，
//! 判空与字段读取全部走安全代码。
//!
//! 节点子指针不再在调用点解引用：直接传裸指针给 `visualize_*`（`IntoNodePtr`
//! 归一）；`AstArray<*mut T>` 遍历统一走 `iter_mut_nodes`。

use crate::records::{
  ast_expr_function::AstExprFunction,
  comma_separator_inserter::CommaSeparatorInserter,
  cst_expr_function::CstExprFunction,
  cst_generic_type_pack::CstGenericTypePack,
  printer::{IntoNodePtr, Printer},
  writer::Writer,
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_function_body<F: IntoNodePtr<AstExprFunction>>(&mut self, func: F) {
    // SAFETY: func 指向 arena 中存活的 AstExprFunction
    let func = unsafe { &mut *func.into_node_ptr() };
    let cst_node = self.lookup_cst_node::<CstExprFunction>(&mut func.base.base);

    if func.generics.size > 0 || func.generic_packs.size > 0 {
      let mut comma = CommaSeparatorInserter::new(
        cst_node.map_or(&[], |cst| cst.generics_comma_positions.as_slice()),
      );

      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.open_generics_position), "<");

      for generic_ty in func.generics.iter_mut_nodes() {
        comma.operator_call(self.writer);
        self.writer.advance(&generic_ty.base.location.begin);
        self.writer.identifier(generic_ty.name.as_bytes());
      }

      for pack in func.generic_packs.iter_mut_nodes() {
        comma.operator_call(self.writer);
        self.writer.advance(&pack.base.location.begin);
        self.writer.identifier(pack.name.as_bytes());

        if let Some(cst) = self.lookup_cst_node::<CstGenericTypePack>(&mut pack.base) {
          self.advance(cst.ellipsis_position);
        }

        self.writer.symbol("...");
      }

      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_generics_position), ">");
    }

    if let Some(arg_location) = func.arg_location.as_ref() {
      self.advance(arg_location.begin);
    }
    self.writer.symbol("(");

    let mut comma =
      CommaSeparatorInserter::new(cst_node.map_or(&[], |cst| cst.args_comma_positions.as_slice()));
    // CST 冒号位置与 args 成对构造（解析器保证等长）；越界仅解析器 bug，
    // 退化直接写 ":"。
    let colon_positions = cst_node.map(|cst| cst.args_annotation_colon_positions.as_slice());

    for (i, local) in func.args.iter_mut_nodes().enumerate() {
      comma.operator_call(self.writer);
      self.advance(local.location.begin);
      self.writer.identifier(local.name.as_bytes());

      if self.write_types && !local.annotation.is_null() {
        match colon_positions.and_then(|c| c.get(i)) {
          Some(colon) => self.maybe_advance_and_write(colon, ":", false),
          None => self.writer.symbol(":"),
        }

        self.visualize_type_annotation(local.annotation);
      }
    }

    if func.vararg {
      comma.operator_call(self.writer);

      self.advance(func.vararg_location.begin);
      self.writer.symbol("...");

      if self.write_types && !func.vararg_annotation.is_null() {
        self.maybe_advance_or_symbol(
          cst_node.map(|cst| &cst.vararg_annotation_colon_position),
          ":",
        );

        self.visualize_type_pack_annotation(func.vararg_annotation, true, true, false);
      }
    }

    if let Some(arg_location) = func.arg_location.as_ref() {
      self.advance_before(arg_location.end, 1);
    }
    self.writer.symbol(")");

    if self.write_types && !func.return_annotation.is_null() {
      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.return_specifier_position), ":");

      if cst_node.is_none() {
        self.writer.space();
      }

      self.visualize_type_pack_annotation(func.return_annotation, false, false, true);
    }

    // SAFETY: body 指向 arena 存活的 AstStatBlock
    let body = unsafe { &mut *func.body };
    self.visualize_block_ast_stat_block(&mut *body);
    self.advance(body.base.base.location.end);
    self.writer.keyword("end");
  }
}
