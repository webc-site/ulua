//! visualize 系列打印器 unsafe 总说明：本文件内的 unsafe 块遵循同一不变式——
//! 解引用的裸指针均指向 arena 中存活的观测节点；`ast_node_as`/`cst_node_as`
//! 下转前已经过 class_index 匹配，各 `visualize_*` 函数只传入解析中静态
//! 类型与动态类型一致的节点。CST 侧经 `lookup_cst_node` 返回 `Option<&T>`，
//! 判空与字段读取全部走安全代码。

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic,
    cst_type_pack_explicit::CstTypePackExplicit,
    cst_type_pack_generic::CstTypePackGeneric,
    position::Position,
    printer::{IntoNodePtr, Printer},
    writer::Writer,
  },
  rtti::ast_node_try_as,
};

impl<'a, W: Writer> Printer<'a, W> {
  pub fn visualize_type_pack_annotation<T: IntoNodePtr<AstTypePack>>(
    &mut self,
    annotation: T,
    for_var_arg: bool,
    unconditionally_parenthesize: bool,
    for_function_return: bool,
  ) {
    // SAFETY: annotation 指向 arena 中存活的 AstTypePack 派生节点
    let annotation = unsafe { &*annotation.into_node_ptr() };
    self.advance(annotation.base.location.begin);

    if let Some(variadic_tp) = ast_node_try_as::<AstTypePackVariadic>(&annotation.base) {
      if !for_var_arg {
        self.writer.symbol("...");
      }
      self.visualize_type_annotation(variadic_tp.variadic_type);
      return;
    }

    if let Some(generic_tp) = ast_node_try_as::<AstTypePackGeneric>(&annotation.base) {
      self
        .writer
        .symbol(generic_tp.generic_name.as_str_or_empty());

      if let Some(cst_node) = self.lookup_cst_node::<CstTypePackGeneric>(&annotation.base) {
        self.advance(cst_node.ellipsis_position);
      }

      self.writer.symbol("...");
      return;
    }

    if let Some(explicit_tp) = ast_node_try_as::<AstTypePackExplicit>(&annotation.base) {
      LUAU_ASSERT!(!for_var_arg);

      if let Some(cst_node) = self.lookup_cst_node::<CstTypePackExplicit>(&annotation.base) {
        self.visualize_type_list(
          &explicit_tp.type_list,
          false,
          cst_node.open_parentheses_position,
          cst_node.close_parentheses_position,
          cst_node.comma_positions.as_slice(),
        );
        return;
      }

      if for_function_return {
        let pack_size = explicit_tp.type_list.types.size
          + usize::from(!explicit_tp.type_list.tail_type.is_null());

        self.visualize_type_list(
          &explicit_tp.type_list,
          pack_size != 1,
          Position::missing(),
          Position::missing(),
          &[],
        );
        return;
      }

      self.visualize_type_list(
        &explicit_tp.type_list,
        unconditionally_parenthesize,
        Position::missing(),
        Position::missing(),
        &[],
      );
      return;
    }

    LUAU_ASSERT!(false);
  }
}
